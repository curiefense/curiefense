import json
import re
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

LOGLEVEL = os.getenv("LOGLEVEL", "INFO").upper()
logging.basicConfig(level=LOGLEVEL)
logger = logging.getLogger("traffic-metrics-exporter")

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
base_labels = ["secpolid", "proxy", "secpolentryid"]
# Object fields that will be flattened process_time: {"avg": 0} -> process_time_avg
flat_properties = ["processing_time", "bytes_sent"]


t3_counters = dict()
os.environ["PROMETHEUS_DISABLE_CREATED_SERIES"] = "true"
for coll in list(REGISTRY._collector_to_names.keys()):
    REGISTRY.unregister(coll)
start_http_server(8911)

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
        "t2_source": {"url": os.getenv("METRICS_URI", "http://curieproxyngx:8999/")},
    }
    return config[key]


@lru_cache
def get_mongodb():
    server_config = get_config("mongodb")
    client = pymongo.MongoClient(server_config["url"])
    return client[server_config["db"]][server_config["collection"]]


def _get_counter_type(counter_name):
    counter_type = counters_format.get(counter_name, False)
    if counter_type:
        return counter_type["type"]
    return False


def switch_hyphens(name):
    return name.replace("-", "_")


def _get_numbers_group(number):
    if 0 == int(number):
        return "0"
    elif 1 <= int(number) <= 5:
        return "1-5"
    elif 6 <= int(number) <= 10:
        return "6-10"
    elif 11 <= int(number) <= 20:
        return "6-20"
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
    t2_dict = flatten_object_properties(t2_dict)
    for counter_name, counter_value in t2_dict.get("counters", {}).items():
        counter_name = name_changes.get(counter_name, counter_name)
        valid_name = switch_hyphens(counter_name)
        counter_type = _get_counter_type(valid_name)
        if not counter_type:
            continue
        counter = t3_counters[valid_name]
        if counter_type == REGULAR:
            counter.labels(app, proxy, profile).inc(counter_value)
        elif counter_type in [AVERAGE, MAX, MIN]:
            # Find average for collected values. The last one will be the right number for the whole period.
            key = f"{proxy}-{app}-{profile}-{valid_name}"
            collect_values(acc_avg, key, counter_value)
            counter.labels(app, proxy, profile).set(
                choose_func(counter_type)(acc_avg[key])
            )
        elif counter_type in [MAX_PER_REQUEST, AVG_PER_REQUEST]:
            for value in counter_value:
                # Collect all and get max/mean. Group by intervals of values.
                group = _get_numbers_group(value["key"])
                key = f"{proxy}-{app}-{profile}-{valid_name}-{group}"
                collect_values(acc_avg, key, value["value"])
                counter.labels(app, proxy, profile, group).set(
                    choose_func(counter_type)(acc_avg[key])
                )
        elif counter_type == COUNTER_BY_KEY:
            for value in counter_value:
                counter.labels(app, proxy, profile, value["key"]).inc(value["value"])
        elif counter_type == COUNTER_OBJECT_BY_KEY:
            for key, value in counter_value.items():
                counter.labels(app, proxy, profile, key).inc(value)


def export_t2(t2: dict):
    client = get_mongodb()
    try:
        client.insert_many(t2)
    except Exception as e:
        logger.exception(e)


def export_t3():
    while True:
        acc_avg = {}
        five_sec_string = q.get()
        five_sec_json = json.loads(five_sec_string)
        export_t2(five_sec_json)
        for agg_sec in five_sec_json:
            start_time = time.time()
            update_t3_counters(agg_sec, acc_avg)


def get_t2():
    config = get_config("t2_source")
    while True:
        start_time = time.time()
        time.time() - start_time
        try:
            five_sec_t2 = requests.get(config["url"]).content.decode()
        except Exception as e:
            logger.exception(e)

        q.put(five_sec_t2)

        time.sleep(5 - (time.time() - start_time))


if __name__ == "__main__":
    t2_receiver = Thread(target=get_t2)
    t3_exporter = Thread(target=export_t3)
    t2_receiver.start()
    t3_exporter.start()
