"""
this script converts BQ JSON structure to ES structure
 input stdin
output stdout
"""

import fileinput
import json
import sys
import datetime

from collections import defaultdict

# now = datetime.datetime.now

# tag_entry = lambda: defaultdict(int)
# tag = defaultdict(lambda: tag_entry)
# tsentry = defaultdict(lambda: tag)

tags_queue = defaultdict(lambda: defaultdict(lambda: defaultdict(int)))
counters_queue = defaultdict(lambda: defaultdict(int))


def status_class(status):
    s = int(status / 100)
    if s in [1, 2, 3, 4, 5]:
        return "status_%ixx" % s
    else:
        return "status_xxx"


def split_static_tag(tag):
    static_tags = (
        "container",
        "aclid",
        "aclname",
        "wafid",
        "wafname",
        "proxy",
        "ip",
        "asn",
        "geo",
        "secprofile",
        "securitypolicy-entry",
        "securitypolicy_entry",
        "securitypolicy" "wafid",
        "wafname",
    )

    parts = tag.split(":", 1)
    if ":" not in tag:
        ## legacy used - instead of :
        parts = tag.split("-", 1)

    if len(parts) > 1:
        prefix = parts[0]
        if prefix in static_tags:
            return (prefix, parts[1])
        else:
            return ("tags", tag)
    else:
        return ("tags", tag)


def set_metrics(eljson, this_second):
    cnt_queue = counters_queue[this_second]
    tg_queue = tags_queue[this_second]

    cnt_queue["http_request_total"] += 1

    req_bytes = eljson["request"].get("bodybytes", None)
    if not req_bytes:
        req_bytes = eljson["request"]["headers"].get("content-length", "0")
        req_bytes = int(req_bytes)

    cnt_queue["request_bytes"] += req_bytes
    cnt_queue["response_bytes"] += eljson["response"].get("bodybytes", 0)

    _class = status_class(eljson["response"]["code"])
    status_code = str(eljson["response"]["code"])
    # print(tg_queue)
    # print(tg_queue["status_code"])
    # print(tg_queue["status_code"][status_code])
    tg_queue["status_code"][status_code] += 1
    tg_queue["status_class"][_class] += 1
    upstream = eljson.get("upstream", None)

    if upstream:
        tg_queue["origin"][upstream["remoteaddress"]] += 1
        tg_queue["origin_status_code"][status_code] += 1
        tg_queue["origin_status_class"][_class] += 1

    tg_queue["method"][eljson["method"]] += 1
    tg_queue["path"][eljson["path"]] += 1
    tg_queue["blocked"][eljson["blocked"] and "1" or "0"] += 1

    tags = eljson["tags"]
    for tag in tags:
        a, b = split_static_tag(tag)
        tg_queue[a][b] += 1


def top_x(data, top=25):
    trim_keys = ("path", "asn", "geo", "ip")
    for key in trim_keys:
        data[key] = dict(
            sorted(data[key].items(), key=lambda x: x[1], reverse=True)[:top]
        )
    return data


def merge_metrics(key):
    tags_queue[key].update(counters_queue[key])
    metrics = top_x(tags_queue[key])
    metrics["@timestamp"] = key
    metrics = json.dumps(metrics)

    del tags_queue[key]
    del counters_queue[key]

    return metrics


def process_metrics(eljson, this_second):
    ## second level metric
    ## "2021-02-09T07:01:29.315000"[:19] > '2021-02-09T07:01:29'
    this_second = this_second[:19]
    set_metrics(eljson, this_second)

    ## py2 vs py3 and len of keys()
    queue_keys = list(tags_queue.keys())
    keyslen = len(queue_keys)
    if keyslen > 1:
        for key in queue_keys:
            if key != this_second:
                return merge_metrics(key)

    return None


def main():
    for line in fileinput.input():
        print(line)


if __name__ == "__main__":
    main()
