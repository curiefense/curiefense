from enum import Enum


class CounterTypes(Enum):
    REGULAR = 1
    # response codes
    STATUS_CODES = 2
    # 2xx, 3xx, etc..
    STATUS_CLASSES = 3
    # http methods , GET, PUT, etc...
    METHODS = 4


REGULAR = CounterTypes.REGULAR
STATUS_CODES = CounterTypes.STATUS_CODES
STATUS_CLASSES = CounterTypes.STATUS_CLASSES
METHODS = CounterTypes.METHODS

# mapping of counters to REGULAR, SESSION and PROCESSING_TIME. the two other ones are recognized by regex
counters_format = {
    "hits": REGULAR,
    "blocks": REGULAR,
    "report": REGULAR,
    "human": REGULAR,
    "bot": REGULAR,
    "challenge": REGULAR,
    "total_downstream_bytes": REGULAR,
    "total_upstream_bytes": REGULAR,
    "unique_blocked_ip": REGULAR,
    "unique_blocked_uri": REGULAR,
    "unique_blocked_user_agent": REGULAR,
    "unique_passed_user_agent": REGULAR,
    "unique_asn": REGULAR,
    "unique_blocked_asn": REGULAR,
    "unique_passed_asn": REGULAR,
    "unique_passed_ip": REGULAR,
    "unique_passed_uri": REGULAR,
    "unique_ip": REGULAR,
    "unique_uri": REGULAR,
    "unique_user_agent": REGULAR,
    "unique_country": REGULAR,
    "unique_blocked_country": REGULAR,
    "unique_passed_country": REGULAR,
    "max_process_time": REGULAR,
    "min_process_time": REGULAR,
    "avg_process_time": REGULAR,
    "methods": METHODS,
    "status": STATUS_CODES,
    "status_classes": STATUS_CLASSES
}

# counters
name_changes = {
    "d_bytes": "total_downstream_bytes",
    "u_bytes": "total_upstream_bytes"
}

# validating format validity, in case new keys will be entered
for counter, value in counters_format.items():
    if value not in [REGULAR, STATUS_CODES, STATUS_CLASSES, METHODS]:
        raise TypeError(f"{counter} is not of a legal type in counters_format")
