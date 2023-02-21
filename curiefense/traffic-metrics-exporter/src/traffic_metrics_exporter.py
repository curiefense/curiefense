import json
from threading import Thread
from queue import Queue
import time
import os
from functools import lru_cache
import pymongo
import requests
import logging
from copy import deepcopy
from statistics import mean
from jsonpath_ng import parse
from dateutil.parser import isoparse
from datetime import datetime, timedelta
from prometheus_client import start_http_server, Counter, REGISTRY, Gauge

from utils.prometheus_counters_dict import (
    REGULAR,
    AVERAGE,
    MAX,
    MIN,
    MAX_PER_REQUEST,
    AVG_PER_REQUEST,
    COUNTER_BY_KEY,
    COUNTER_OBJECT_BY_KEY,
    counters_format,
    name_changes,
)

ENABLE_EXPORT_T2 = os.getenv("ENABLE_EXPORT_T2", "True").lower() in ("true", "1", "on")
T2_JSON_PATH = os.getenv("T2_JSON_PATH", "$.[*]")
# Prepare JSONPath parser to extract data from received JSONs
JSON_PARSE_EXPR = parse(T2_JSON_PATH)
LOGLEVEL = os.getenv("LOGLEVEL", "INFO").upper()
logging.basicConfig(level=LOGLEVEL)
logger = logging.getLogger("traffic-metrics-exporter")
SERVER_PORT = int(os.getenv("SERVER_PORT", 8911))
CUSTOM_HEADERS = os.getenv("CUSTOM_HEADERS", "{}")

METRICS_PULL_INTERVAL = int(os.getenv("METRICS_PULL_INTERVAL", 60))

http_methods = [
    "GET",
    "HEAD",
    "POST",
    "PUT",
    "DELETE",
    "CONNECT",
    "OPTIONS",
    "TRACE",
    "PATCH",
]
base_labels = ["secpolid", "proxy", "secpolentryid", "branch", "namespace"]
# Object fields that will be flattened process_time: {"avg": 0} -> process_time_avg
flat_properties = ["processing_time", "bytes_sent"]


t3_counters = dict()
os.environ["PROMETHEUS_DISABLE_CREATED_SERIES"] = "true"
for coll in list(REGISTRY._collector_to_names.keys()):
    REGISTRY.unregister(coll)
start_http_server(SERVER_PORT)

for name, counter_label in counters_format.items():
    counter_name = name
    type = counter_label["type"]
    label = counter_label.get("label")
    more_labels = [label] if label else []
    if type in [REGULAR, COUNTER_BY_KEY, COUNTER_OBJECT_BY_KEY]:
        t3_counters[counter_name] = Counter(counter_name, "", base_labels + more_labels)
    elif type in [AVERAGE, MAX, MIN, MAX_PER_REQUEST, AVG_PER_REQUEST]:
        t3_counters[counter_name] = Gauge(counter_name, "", base_labels + more_labels)

q = Queue()


def get_config(key):
    config = {
        "mongodb": {
            "url": os.getenv("MONGODB_URI", "mongodb://mongodb:27017/"),
            "db": os.getenv("MONGODB_METRICS_DB", "curiemetrics"),
            "collection": os.getenv("MONGODB_METRICS_COLLECTION", "metrics1s"),
        },
        "t2_source": {
            "url": replace_url_placeholders(
                os.getenv("METRICS_URI", "http://curieproxyngx:8999/")
            ),
            "headers": {
                "Host": os.getenv("METRICS_HOST", "metrics.curiefense.io"),
                **json.loads(CUSTOM_HEADERS),
            },
        },
    }
    return config[key]


@lru_cache
def get_mongodb():
    server_config = get_config("mongodb")
    client = pymongo.MongoClient(server_config["url"])
    return client[server_config["db"]][server_config["collection"]]


def replace_url_placeholders(url: str):
    now = datetime.now()
    minute_pattern = "%Y-%m-%dT%H:%M"
    placeholders = {
        "$ISO_TIME_FROM_M": now.strftime(minute_pattern),
        "$ISO_TIME_TO_1M": (now + timedelta(minutes=1)).strftime(minute_pattern),
    }
    for placeholder, value in placeholders.items():
        url = url.replace(placeholder, value)
    return url


def extract_from_response(response: str):
    return [v.value for v in JSON_PARSE_EXPR.find(json.loads(response))]


def _get_counter_type(counter_name):
    counter_type = counters_format.get(counter_name, False)
    if counter_type:
        return counter_type["type"]
    return False


def switch_hyphens(name):
    return name.replace("-", "_")


def _get_sleep_interval(start_time):
    logger.info("start_time %s" % start_time)
    sleep = METRICS_PULL_INTERVAL - (time.time() - start_time)
    return 0 if sleep < 0 else sleep


def _get_numbers_group(number):
    if 0 == int(number):
        return "0"
    elif 1 <= int(number) <= 5:
        return "1-5"
    elif 6 <= int(number) <= 10:
        return "6-10"
    elif 11 <= int(number) <= 20:
        return "11-20"
    elif 21 <= int(number) <= 32:
        return "21-32"
    else:
        return ">32"


def _round_mean(x):
    return round(mean(x), 3)


def collect_values(acc, key, value):
    if not acc.get(key):
        acc[key] = []
    acc[key].append(value)


def take_earliest(agg_list):
    """each element of aggregated data contains list of entries - for the current
    and part of the next minute. Take entry only of the earliest minute.
    """
    earliest_timestamp = min({agg["timestamp"] for agg in agg_list})
    return filter(lambda agg: agg["timestamp"] == earliest_timestamp, agg_list)


def flatten_object_properties(t2_dict: dict):
    t2 = deepcopy(t2_dict)
    for counter_name, counter_value in t2_dict.get("counters", {}).items():
        if counter_name in flat_properties:
            for key, value in counter_value.items():
                t2["counters"][f"{counter_name}_{key}"] = value
    return t2


def choose_func(counter_type):
    return {
        MAX_PER_REQUEST: max,
        AVG_PER_REQUEST: _round_mean,
        AVERAGE: _round_mean,
        MAX: max,
        MIN: min,
    }.get(counter_type)


def update_t3_counters(t2_dict, acc_avg):
    proxy = t2_dict.get("proxy", "")
    app = t2_dict.get("secpolid", "")
    profile = t2_dict.get("secpolentryid", "")
    branch = t2_dict.get("branch", "")
    namespace = t2_dict.get("planet_name", "")
    labels = [app, proxy, profile, branch, namespace]

    t2_dict = flatten_object_properties(t2_dict)
    for counter_name, counter_value in t2_dict.get("counters", {}).items():
        counter_name = name_changes.get(counter_name, counter_name)
        valid_name = switch_hyphens(counter_name)
        counter_type = _get_counter_type(valid_name)
        if not counter_type:
            continue
        counter = t3_counters[valid_name]
        if counter_type == REGULAR:
            counter.labels(*labels).inc(counter_value)
        elif counter_type in [AVERAGE, MAX, MIN]:
            # Find average for collected values. The last one will be the right number for the whole period.
            key = f"{proxy}-{app}-{profile}-{branch}-{valid_name}"
            collect_values(acc_avg, key, counter_value)
            counter.labels(*labels).set(choose_func(counter_type)(acc_avg[key]))
        elif counter_type in [MAX_PER_REQUEST, AVG_PER_REQUEST]:
            for value in counter_value:
                # Collect all and get max/mean. Group by intervals of values.
                group = _get_numbers_group(value["key"])
                key = f"{proxy}-{app}-{profile}-{branch}-{valid_name}-{group}"
                collect_values(acc_avg, key, value["value"])
                counter.labels(*labels, group).set(
                    choose_func(counter_type)(acc_avg[key])
                )
        elif counter_type == COUNTER_BY_KEY:
            for value in counter_value:
                counter.labels(*labels, value["key"]).inc(value["value"])
        elif counter_type == COUNTER_OBJECT_BY_KEY:
            for key, value in counter_value.items():
                counter.labels(*labels, key).inc(value)


def export_t2(t2: dict):
    client = get_mongodb()
    try:
        for item in t2:
            item["timestamp"] = isoparse(item["timestamp"])
        client.insert_many(t2)
    except Exception as e:
        logger.exception(e)


def export_t3():
    while True:
        acc_avg = {}
        five_sec_string = q.get()
        try:
            five_sec_json = extract_from_response(five_sec_string)
            logger.info(five_sec_json)
            if five_sec_json:
                five_sec_json = take_earliest(five_sec_json)
                if ENABLE_EXPORT_T2:
                    export_t2(five_sec_json)

                # Clear all gauges so they do not drag previous values into new intervals
                for key in t3_counters:
                    if isinstance(t3_counters[key], Gauge):
                        t3_counters[key].clear()

                for agg_sec in five_sec_json:
                    start_time = time.time()
                    update_t3_counters(agg_sec, acc_avg)
            else:
                logger.info("there is no data for the current period")
        except Exception as e:
            logger.exception(e)


def get_t2():
    config = get_config("t2_source")
    logger.info("entering the while True")
    while True:
        start_time = time.time()
        logger.info("start time %s" % start_time)
        try:
            logger.info("url: %s" % config["url"])
            logger.info("headers: %s" % config["headers"])

            t2_data = requests.get(config["url"], headers=config["headers"])
            t2_data = t2_data.content.decode()

            q.put(t2_data)
            logger.info("done iteration: %s" % start_time)

        except Exception as e:
            logger.exception(e)

        sleep_interval = _get_sleep_interval(start_time)
        logger.info("sleeping for %s" % sleep_interval)
        time.sleep(sleep_interval)


if __name__ == "__main__":
    t2_receiver = Thread(target=get_t2)
    t3_exporter = Thread(target=export_t3)
    t2_receiver.start()
    t3_exporter.start()
