#!/usr/bin/env python3

# Python requirements: pytest requests requests_toolbelt
# install curieconfctl:
# (cd ../curiefense/curieconf/utils ; pip3 install .)
# (cd ../curiefense/curieconf/client ; pip3 install .)
#
# To run this with minikube (does not support IPv6):
#
# export IP=$(minikube ip)
# pytest --base-protected-url http://$IP:30081 --base-conf-url http://$IP:30000/api/v3/ --base-ui-url http://$IP:30080 --elasticsearch-url http://$IP:30200 --luatests-path ../curiefense/curieproxy/rust/luatests .      # pylint: disable=line-too-long
#
# To run this with docker-compose:
# export IP=localhost
# pytest --base-protected-url http://$IP:30081/ --base-conf-url http://$IP:30000/api/v3/ --base-ui-url http://$IP:30080 --elasticsearch-url http://$IP:9200 --luatests-path ../curiefense/curieproxy/rust/luatests .      # pylint: disable=line-too-long

from typing import Any, Dict, List, Optional
import reqflip
import json
import logging
import os
import pytest
import random
import requests
import string
import subprocess
import time
from urllib.parse import urljoin

# --- Helpers ---
TEST_CONFIG_NAME = "prod"


class CliHelper:
    def __init__(self, base_url: str):
        self._base_url = base_url
        self._initial_version_cache = None

    def call(self, args: str, inputjson: Any = None) -> Any:
        logging.info("Calling CLI with arguments: %s", args)
        cmd = ["curieconfctl", "-u", self._base_url, "-o", "json"]
        cmd += args.split(" ")
        indata = None
        if inputjson:
            indata = json.dumps(inputjson).encode("utf-8")

        try:
            process = subprocess.run(
                cmd,
                shell=False,
                input=indata,
                check=True,
                capture_output=True,
            )
            if process.stdout:
                logging.debug("CLI output: %s", process.stdout)

                try:
                    return json.loads(process.stdout.decode("utf-8"))
                except json.JSONDecodeError:
                    return process.stdout.decode("utf-8")
            else:
                return []
        except subprocess.CalledProcessError as e:
            print("stdout:" + e.stdout.decode("utf-8", errors="ignore"))
            print("stderr:" + e.stderr.decode("utf-8", errors="ignore"))
            raise e

    def delete_test_config(self):
        self.call("conf delete test")

    def initial_version(self):
        if not self._initial_version_cache:
            versions = self.call("conf list-versions prod")
            if "version" not in versions[-3]:
                print("Unsupported curieconfctl output", versions)
                raise TypeError("Unsupported curieconfctl output")
            self._initial_version_cache = versions[-3]["version"]
        return self._initial_version_cache

    def empty_acl(self):
        version = self.initial_version()
        return self.call(f"doc get prod aclprofiles --version {version}")

    def publish_and_apply(self):
        buckets = self.call("key get system publishinfo")

        url = "????"
        for bucket in buckets["buckets"]:
            if bucket["name"] == "prod":
                url = bucket["url"]
        self.call(f"tool publish prod {url}")
        time.sleep(20)

    def set_configuration(self, luatests_path: str):
        for cmdname, path in [
            ("actions", "actions.json"),
            ("aclprofiles", "acl-profiles.json"),
            ("contentfilterprofiles", "contentfilter-profiles.json"),
            ("contentfilterrules", "contentfilter-rules.json"),
            ("flowcontrol", "flow-control.json"),
            ("globalfilters", "globalfilter-lists.json"),
            ("ratelimits", "limits.json"),
            ("securitypolicies", "securitypolicy.json"),
        ]:
            cfgpath = os.path.join(luatests_path, "config", "json", path)
            ret = self.call(f"doc delete {TEST_CONFIG_NAME} {cmdname}")
            assert ret == {"ok": True}
            ret = self.call(f"doc create {TEST_CONFIG_NAME} {cmdname} {cfgpath}")
            assert ret == {"ok": True}
        self.publish_and_apply()


@pytest.fixture(scope="session")
def luatests_path(request: pytest.FixtureRequest) -> str:
    path = request.config.getoption("--luatests-path")
    assert isinstance(path, str), "bad lua test"
    return path


@pytest.fixture(scope="session")
def curieconfig(request: pytest.FixtureRequest, luatests_path: str):
    if not request.config.getoption("--ignore-config"):
        conf_url = request.config.getoption("--base-conf-url")
        assert isinstance(conf_url, str), "--base-conf-url unset"
        cli = CliHelper(conf_url)
        cli.set_configuration(luatests_path)


class RequestHelper:
    def __init__(self, base_url: str, hops: int, flip_requests: bool):
        self._base_url = base_url
        self._hops = hops
        self._flip = flip_requests

    def request(
        self,
        method: str,
        path: str,
        headers: Dict[str, str],
        ip: Optional[str],
        body: Optional[str],
    ) -> requests.Response:
        if ip:
            ip_lst = [ip] + ["10.0.0.%d" % step for step in range(self._hops - 1)]
            headers["x-forwarded-for"] = ",".join(ip_lst)
        url = urljoin(self._base_url, path)
        print(f"{method} {url} {headers} {ip} {body}")
        return requests.request(
            method=method,
            headers=headers,
            data=body,
            url=url,
        )

    def run(self, req: Any) -> requests.Response:
        method: str = "GET"
        path: str = "/"
        headers: Dict[str, str] = {}

        k: str
        v: str
        for k, v in req["headers"].items():
            if k.startswith(":"):
                if k == ":method":
                    method = v
                elif k == ":authority":
                    headers["Host"] = v
                elif k == ":path":
                    path = v
            else:
                headers[k.lower()] = v

        res = self.request(method, path, headers, req.get("ip"), req.get("body"))
        if self._flip:
            # Also send copies of the request, flipping bits one by one
            # This is a "light fuzzing" approach
            reqflip.bitflip_send(res)
        return res


@pytest.fixture(scope="session")
def requester(curieconfig: None, request: pytest.FixtureRequest, flip_requests: bool):
    target_url = request.config.getoption("--base-protected-url")
    assert isinstance(target_url, str)
    hops = request.config.getoption("--xff-hops")
    assert isinstance(hops, int)
    return RequestHelper(target_url.rstrip("/"), hops, flip_requests)


@pytest.fixture(scope="session")
def flip_requests(request: pytest.FixtureRequest):
    return request.config.getoption("--flip-requests")


def test_logging(request: pytest.FixtureRequest, requester: RequestHelper):
    """
    Tests request logs got published to the Elasticsearch
    """
    es_url = request.config.getoption("--elasticsearch-url")
    assert isinstance(es_url, str)
    es_url = urljoin(es_url, "/_search")
    test_pattern = "/test" + "".join(
        [random.choice(string.ascii_lowercase) for _ in range(20)]
    )
    res = requester.request("GET", test_pattern, {}, None, None)
    assert res.status_code == 200
    # NOTE: wait for Elastic to index data
    for _ in range(15):
        time.sleep(4)
        mdata = {"query": {"bool": {"must": {"match": {"path": test_pattern}}}}}
        res = requests.get(es_url, json=mdata)
        data = res.json()
        num_hits = data["hits"]["total"]["value"]
        if num_hits > 0:
            break

    hits = data["hits"]["hits"]
    assert len(hits) == 1
    entry = hits[0]["_source"]
    assert entry["path"] == test_pattern
    assert entry["method"] == "GET"
    assert entry["blocked"] == False
    assert entry["response_code"] == 200


def test_raw_request(raw_request: Any, requester: RequestHelper):
    req = raw_request
    if "human" in req and req["human"]:
        pytest.skip("Ignoring because of humanity test")
    if "plugins" in req:
        pytest.skip("Ignoring because of plugin test")
    res = requester.run(req)
    response = req["response"]
    if "block_mode" in response and response["block_mode"]:
        expected = (
            response["real_status"] if "real_status" in response else response["status"]
        )
    else:
        expected = 200
    assert expected == res.status_code


def test_rate_limit(
    limit_request: List[Any],
    requester: RequestHelper,
    max_time_limit: int,
    flip_requests: bool,
):
    if flip_requests:
        pytest.skip("only run raw requests for bit flip tests")
    time.sleep(max_time_limit)
    for step, req in enumerate(limit_request):
        res = requester.run(req)
        if req["pass"]:
            assert res.status_code == 200, "at step %d" % step
        else:
            assert res.status_code != 200, "at step %d" % step
        time.sleep(req["delay"])


def test_flow_control(
    flow_request: List[Any],
    requester: RequestHelper,
    max_time_limit: int,
    flip_requests: bool,
):
    if flip_requests:
        pytest.skip("only run raw requests for bit flip tests")
    time.sleep(max_time_limit)
    for step, req in enumerate(flow_request):
        res = requester.run(req)
        if req["pass"]:
            assert res.status_code == 200, "at step %d" % step
        else:
            assert res.status_code != 200, "at step %d" % step
        time.sleep(req["delay"])
