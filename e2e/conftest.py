from typing import Any, Iterator, List, Tuple
import json
import os
import pytest

# run with --log-level info for debugging tests & fixtures


def pytest_addoption(parser):
    parser.addoption("--all", action="store_true", help="run all combinations")
    parser.addoption(
        "--base-conf-url",
        help="Base url for confserver API",
        type=str,
        default="http://localhost:30000/api/v1/",
    )
    parser.addoption(
        "--base-protected-url",
        help="Base URL for the protected website",
        default="http://localhost:30081",
    )
    parser.addoption(
        "--base-ui-url",
        help="Base URL for the UI server",
        default="http://localhost:30080",
    )
    parser.addoption(
        "--elasticsearch-url",
        help="Elasticsearch URL (ex. http://localhost:9200)",
        default="",
    )
    parser.addoption(
        "--luatests-path", required=True, help="Path to the luatests directory"
    )
    parser.addoption(
        "--xff-hops",
        required=False,
        default=1,
        type=int,
        help="Number of XFF trusted hops",
    )
    parser.addoption(
        "--ignore-config",
        action="store_true",
        help="Ignore configuration phase, run the tests on the current configuration",
        default=False,
    )
    parser.addoption(
        "--flip-requests",
        help="For each request to the protected urls, also send len(request)*8 requests with 1 flipped bit",
        action="store_true",
        default=False,
    )


def case_load(luatests_path: str, path: str) -> Iterator[Tuple[str, Any]]:
    for file in os.listdir(os.path.join(luatests_path, path)):
        if not file.endswith(".json"):
            continue
        with open(os.path.join(luatests_path, path, file)) as f:
            yield (file, json.load(f))


def rename_test(testname: str) -> str:
    return testname.replace(" ", "_")


def pytest_generate_tests(metafunc: pytest.Metafunc):
    luatests_path = metafunc.config.getoption("--luatests-path")
    assert isinstance(luatests_path, str)
    if "raw_request" in metafunc.fixturenames:
        params: List[Tuple[str, Any]] = []
        for fname, elements in case_load(luatests_path, "raw_requests"):
            for element in elements:
                params.append((fname, element))
        metafunc.parametrize(
            "raw_request",
            [param[1] for param in params],
            ids=["%s/%s" % (p[0], rename_test(p[1]["name"])) for p in params],
        )
    if "max_time_limit" in metafunc.fixturenames:
        max_time_limit = 0
        with open(
            os.path.join(luatests_path, "config", "json", "limits.json"), "r"
        ) as lfile:
            for limit in json.load(lfile):
                for t in limit["thresholds"]:
                    max_time_limit = max(max_time_limit, t["limit"])
        with open(
            os.path.join(luatests_path, "config", "json", "flow-control.json"), "r"
        ) as lfile:
            for flow in json.load(lfile):
                max_time_limit = max(max_time_limit, flow["timeframe"])
        metafunc.parametrize("max_time_limit", [max_time_limit])
    if "limit_request" in metafunc.fixturenames:
        cases = list(case_load(luatests_path, "ratelimit"))
        metafunc.parametrize(
            "limit_request",
            [case[1] for case in cases],
            ids=[case[0] for case in cases],
        )
    if "flow_request" in metafunc.fixturenames:
        cases = list(case_load(luatests_path, "flows"))
        metafunc.parametrize(
            "flow_request", [case[1] for case in cases], ids=[case[0] for case in cases]
        )
