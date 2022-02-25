#!/usr/bin/env python3

# Python requirements: pytest requests
# install curieconfctl:
# (cd ../curiefense/curieconf/utils ; pip3 install .)
# (cd ../curiefense/curieconf/client ; pip3 install .)
#
# To run this with minikube (does not support IPv6):
#
# pytest --base-protected-url http://$(minikube ip):30081 --base-conf-url http://$(minikube ip):30000/api/v2/ --base-ui-url http://$(minikube ip):30080 --elasticsearch-url http://$IP:30200 .      # pylint: disable=line-too-long
#
# To run this with docker-compose:
# pytest --base-protected-url http://localhost:30081/ --base-conf-url http://localhost:30000/api/v2/ --base-ui-url http://localhost:30080 --elasticsearch-url http://localhost:9200 .      # pylint: disable=line-too-long

# pylint: disable=too-many-lines,too-many-public-methods
# pylint: disable=too-many-arguments,too-few-public-methods,too-many-statements
# pylint: disable=missing-function-docstring,missing-module-docstring
# pylint: disable=missing-class-docstring

# This is not really a problem for fixtures
# pylint: disable=redefined-outer-name

# This is often wrong: fixtures are not mentioned in the function, but they
# define the required test environment
# pylint: disable=unused-argument

# This follows examples from the pytest doc: tests are class methods, even
# though they don't use self
# pylint: disable=no-self-use


from typing import List, Optional
from urllib.parse import urlparse
import json
import logging
import random
import string
import subprocess
import time
import pytest
import requests

log = logging.getLogger("e2e")

# --- Helpers ---
TEST_CONFIG_NAME = "master"


class CliHelper:
    def __init__(self, base_url):
        self._base_url = base_url
        self._initial_version_cache = None

    def call(self, args, inputjson=None):
        logging.info("Calling CLI with arguments: %s", args)
        cmd = ["curieconfctl", "-u", self._base_url, "-o", "json"]
        cmd += args.split(" ")
        indata = None
        if inputjson:
            indata = json.dumps(inputjson).encode("utf-8")

        process = subprocess.run(
            cmd,
            shell=False,
            input=indata,
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        if process.stdout:
            logging.debug("CLI output: %s", process.stdout)

            try:
                return json.loads(process.stdout.decode("utf-8"))
            except json.JSONDecodeError:
                return process.stdout.decode("utf-8")
        else:
            return []

    def delete_test_config(self):
        self.call("conf delete test")

    def initial_version(self):
        if not self._initial_version_cache:
            versions = self.call("conf list-versions master")
            if "version" not in versions[-3]:
                print("Unsupported curieconfctl output", versions)
                raise TypeError("Unsupported curieconfctl output")
            self._initial_version_cache = versions[-3]["version"]
        return self._initial_version_cache

    def empty_acl(self):
        version = self.initial_version()
        return self.call(f"doc get master aclprofiles --version {version}")

    def revert_and_enable(self, acl=True, content_filter=True):
        version = self.initial_version()
        self.call(f"conf revert {TEST_CONFIG_NAME} {version}")
        securitypolicy = self.call(f"doc get {TEST_CONFIG_NAME} securitypolicies")
        securitypolicy[0]["map"][0]["acl_active"] = acl
        securitypolicy[0]["map"][0]["content_filter_active"] = content_filter
        self.call(
            f"doc update {TEST_CONFIG_NAME} securitypolicies /dev/stdin",
            inputjson=securitypolicy,
        )

    def publish_and_apply(self):
        buckets = self.call("key get system publishinfo")

        for bucket in buckets["buckets"]:
            if bucket["name"] == "prod":
                url = bucket["url"]
        self.call(f"tool publish master {url}")
        time.sleep(20)


@pytest.fixture(scope="session")
def cli(request):
    return CliHelper(request.config.getoption("--base-conf-url"))


class TargetHelper:
    def __init__(self, base_url):
        self._base_url = base_url

    def query(
        self, path="/", suffix="", method="GET", headers=None, srcip=None, **kwargs
    ):
        # specifying a path helps spot tests easily in the access log
        if headers is None:
            headers = {}
        if srcip is not None:
            headers["X-Forwarded-For"] = srcip
        res = requests.request(
            method=method, url=self._base_url + path + suffix, headers=headers, **kwargs
        )
        return res

    def is_reachable(self, *args, **kwargs):
        res = self.query(*args, **kwargs)
        return res.status_code in [200, 404]

    def authority(self) -> str:
        return urlparse(self._base_url).netloc


@pytest.fixture(scope="session")
def target(request):
    url = request.config.getoption("--base-protected-url").rstrip("/")
    return TargetHelper(url)


# geo=US, company=SPRINTLINK, asn=1239
IP4_US = "199.0.0.1"

# geo=JP, company=Softbank BB Corp., asn=17676
IP4_JP = "126.0.0.1"

# geo=AU, company=CLOUDFLARENET, asn=13335
IP4_CLOUDFLARE = "1.0.0.0"

# geo=FR, company=Orange, asn=3215
IP4_ORANGE = "2.0.0.0"

IP6_1 = "0000:0000:0000:0000:0000:0000:0000:0001"
IP6_2 = "0000:0000:0000:0000:0000:0000:0000:0002"


class LogHelper:
    def __init__(self, base_url, es_url):
        self._base_url = base_url
        self._es_url = es_url + "/_search"

    def check_log_pattern(self, pattern):
        data = {
            "query": {"bool": {"must": {"match": {"request.attributes.uri": pattern}}}}
        }
        res = requests.get(self._es_url, json=data)
        nbhits = res.json()["hits"]["total"]["value"]
        if nbhits == 1:
            return True
        else:
            print("Pattern %r" % (pattern,))
            print("Request result %r" % (res,))
            return False


@pytest.fixture(scope="session")
def log_fixture(request):
    url = request.config.getoption("--base-ui-url").rstrip("/")
    es_url = request.config.getoption("--elasticsearch-url").rstrip("/")
    return LogHelper(url, es_url)


class ACLHelper:
    def __init__(self, cli):
        self._cli = cli

    def set_acl(self, updates: dict):
        acl = self._cli.empty_acl()
        # update acl
        for key, value in updates.items():
            acl[0][key].append(value)
        self._cli.call(
            f"doc update {TEST_CONFIG_NAME} aclprofiles /dev/stdin", inputjson=acl
        )

    def reset_and_set_acl(self, updates: dict):
        self._cli.revert_and_enable()
        self.set_acl(updates)
        self._cli.publish_and_apply()


@pytest.fixture(scope="session")
def acl(cli):
    return ACLHelper(cli)


@pytest.fixture(scope="class")
def default_config(cli):
    cli.revert_and_enable()
    cli.publish_and_apply()


@pytest.fixture(scope="function", params=["headers", "cookies", "params"])
def section(request):
    return request.param


# --- Tests ---


class TestLogs:
    def test_logs(self, default_config, cli, target, log_fixture):
        test_pattern = "/test" + "".join(
            [random.choice(string.ascii_lowercase) for i in range(20)]
        )
        assert target.is_reachable(test_pattern)
        time.sleep(10)
        assert log_fixture.check_log_pattern(test_pattern)


class TestACL:
    def test_enforce_deny_all(self, acl, target):
        acl.reset_and_set_acl({"force_deny": "all"})
        assert not target.is_reachable("/deny-all")

    def test_passthrough_all(self, acl, target):
        acl.reset_and_set_acl({"deny": "all", "passthrough": "all"})
        assert target.is_reachable("/deny-passthrough-all")

    def test_allow_bot_all(self, acl, target):
        acl.reset_and_set_acl({"allow_bot": "all"})
        assert not target.is_reachable(
            "/allow_bot-all", headers={"Long-Header": "not_alphanum" * 1500}
        )
        assert target.is_reachable()

    def test_deny_bot_all(self, acl, target):
        acl.reset_and_set_acl({"deny_bot": "all"})
        res = target.query(path="/deny_bot-all")
        assert res.status_code == 247
        assert ";;window.rbzns={bereshit:" in res.text

    def test_allow_all(self, acl, target):
        acl.reset_and_set_acl({"allow": "all", "deny": "all"})
        assert not target.is_reachable(
            "/allow-deny-all", headers={"Long-Header": "not_alphanum" * 1500}
        )
        assert target.is_reachable()

    def test_deny_all(self, acl, target):
        acl.reset_and_set_acl({"deny": "all"})
        assert not target.is_reachable("/deny-all")

    def test_ip_asn(self, acl, target):
        acl.reset_and_set_acl({"deny": "asn:1239"})
        assert not target.is_reachable("/acl-asn", srcip=IP4_US)
        assert target.is_reachable("/")

    def test_ipv4(self, acl, target):
        acl.reset_and_set_acl({"deny": "ip:199-0-0-1"})
        assert not target.is_reachable("/acl-ipv4", srcip=IP4_US)
        assert target.is_reachable("/")

    def test_geo(self, acl, target):
        acl.reset_and_set_acl({"deny": "geo:united-states"})
        assert not target.is_reachable("/acl-geo", srcip=IP4_US)
        assert target.is_reachable("/acl-geo", srcip=IP4_JP)
        assert target.is_reachable("/")

    def test_ipv6(self, acl, target):
        acl.reset_and_set_acl({"deny": "ip:0000:0000:0000:0000:0000:0000:0000:0001"})
        assert not target.is_reachable("/acl-ipv6", srcip=IP6_1)
        assert target.is_reachable("/")


# --- Rate limit tests ---


def gen_rl_rules(authority):
    rl_rules = []
    prof_rules = []
    map_path = {}

    def build_profiling_rule(id: str, name: str, prefix: str, **kwargs) -> List[str]:
        for n in ["cookies", "headers", "args", "attrs"]:
            r: Optional[str] = kwargs.get("%s_%s" % (prefix, n))
            if r is None:
                continue
            if isinstance(r, dict):
                (k, v) = list(r.items())[0]
                if n == "attrs":
                    if k == "tags":
                        return [v]
                    entry = [k, v, "annotation"]
                else:
                    entry = [n, [k, v], "annotation"]
            else:
                entry = [n, r, "annotation"]
            prof_rules.append(
                {
                    "id": id,
                    "name": name,
                    "source": "self-managed",
                    "mdate": "2020-11-22T00:00:00.000Z",
                    "description": "E2E test tag rules",
                    "entries_relation": "OR",
                    "active": True,
                    "tags": [id],
                    "rule": {
                        "relation": "OR",
                        "sections": [
                            {
                                "relation": "OR",
                                "entries": [entry],
                            },
                        ],
                    },
                }
            )
            return [id]
        return []

    def add_rl_rule(
        path, action_ext=None, subaction_ext=None, param_ext=None, **kwargs
    ):
        rule_id = f"e2e1{len(rl_rules):0>9}"
        incl_id = f"incl{len(rl_rules):0>9}"
        excl_id = f"excl{len(rl_rules):0>9}"

        if subaction_ext is None:
            subaction_ext = {}
        if action_ext is None:
            action_ext = {}
        if param_ext is None:
            param_ext = {}
        map_path[path] = rule_id
        incl = build_profiling_rule(incl_id, incl_id, "incl", **kwargs)
        excl = build_profiling_rule(excl_id, excl_id, "excl", **kwargs)
        rl_rules.append(
            {
                "id": rule_id,
                "name": "Rate Limit Rule 3/10 " + path,
                "description": "3 requests per 10 seconds",
                "timeframe": "10",
                "thresholds": [
                    {
                        "limit": "3",
                        "action": {
                            "type": kwargs.get("action", "default"),
                            "params": {
                                "action": {
                                    "type": kwargs.get("subaction", "default"),
                                    "params": kwargs.get("subaction_params", {}),
                                    **subaction_ext,
                                },
                                **param_ext,
                            },
                            **action_ext,
                        },
                    }
                ],
                "include": incl,
                "exclude": excl,
                "key": kwargs.get("key", [{"attrs": "ip"}]),
                "pairwith": kwargs.get("pairwith", {"self": "self"}),
            }
        )

    # RL scope
    add_rl_rule(
        "scope-cookies",
        incl_cookies={"include": "true"},
        excl_cookies={"exclude": "true"},
    )
    add_rl_rule(
        "scope-headers",
        incl_headers={"include": "true"},
        excl_headers={"exclude": "true"},
    )
    add_rl_rule(
        "scope-params", incl_args={"include": "true"}, excl_args={"exclude": "true"}
    )
    add_rl_rule(
        "scope-path",
        incl_attrs={"path": "/scope-path/include/"},
        excl_attrs={"path": "/scope-path/include/exclude/"},
    )
    add_rl_rule(
        "scope-uri",
        incl_attrs={"uri": "/scope-uri/include/"},
        excl_attrs={"uri": "/scope-uri/include/exclude/"},
    )
    add_rl_rule("scope-ipv4-include", incl_attrs={"ip": IP4_US})
    add_rl_rule("scope-ipv4-exclude", excl_attrs={"ip": IP4_US})
    add_rl_rule("scope-country-include", incl_attrs={"country": "us"})
    add_rl_rule("scope-country-exclude", excl_attrs={"country": "us"})
    add_rl_rule("scope-company-include", incl_attrs={"company": "CLOUDFLARENET"})
    add_rl_rule("scope-company-exclude", excl_attrs={"company": "CLOUDFLARENET"})
    add_rl_rule("scope-provider-include", incl_attrs={"asn": "1239"})
    add_rl_rule("scope-provider-exclude", excl_attrs={"asn": "1239"})
    add_rl_rule("scope-method-include", incl_attrs={"method": "GET"})
    add_rl_rule("scope-method-exclude", excl_attrs={"method": "GET"})
    add_rl_rule("scope-query-include", incl_attrs={"query": "QUERY"})
    add_rl_rule("scope-query-exclude", excl_attrs={"query": "QUERY"})
    add_rl_rule("scope-authority-include", incl_attrs={"authority": authority})
    add_rl_rule("scope-authority-exclude", excl_attrs={"authority": authority})
    add_rl_rule(
        "scope-other-authority-include", incl_attrs={"authority": "doesnotmatch"}
    )
    add_rl_rule(
        "scope-other-authority-exclude", excl_attrs={"authority": "doesnotmatch"}
    )

    # RL count by 1 value
    add_rl_rule("countby-cookies", key=[{"cookies": "countby"}])
    add_rl_rule("countby-headers", key=[{"headers": "countby"}])
    add_rl_rule("countby-params", key=[{"args": "countby"}])
    add_rl_rule("countby-ipv4", key=[{"attrs": "ip"}])
    add_rl_rule("countby-ipv6", key=[{"attrs": "ip"}])
    # "Provider" in the UI maps to "asn"
    add_rl_rule("countby-provider", key=[{"attrs": "asn"}])
    add_rl_rule("countby-uri", key=[{"attrs": "uri"}])
    add_rl_rule("countby-path", key=[{"attrs": "path"}])
    add_rl_rule("countby-query", key=[{"attrs": "query"}])
    add_rl_rule("countby-method", key=[{"attrs": "method"}])
    add_rl_rule("countby-company", key=[{"attrs": "company"}])
    add_rl_rule("countby-country", key=[{"attrs": "country"}])
    add_rl_rule("countby-authority", key=[{"attrs": "authority"}])
    # RL count by 2 value (same type)
    add_rl_rule(
        "countby2-cookies", key=[{"cookies": "countby1"}, {"cookies": "countby2"}]
    )
    add_rl_rule(
        "countby2-headers", key=[{"headers": "countby1"}, {"headers": "countby2"}]
    )
    add_rl_rule("countby2-params", key=[{"args": "countby1"}, {"args": "countby2"}])
    # RL count by 2 value (different type)
    add_rl_rule(
        "countby-cookies-headers", key=[{"cookies": "countby"}, {"headers": "countby"}]
    )
    add_rl_rule(
        "countby-headers-params", key=[{"headers": "countby"}, {"args": "countby"}]
    )
    add_rl_rule(
        "countby-params-cookies", key=[{"args": "countby"}, {"cookies": "countby"}]
    )
    # RL Event condition
    add_rl_rule("event-cookies", pairwith={"cookies": "event"})
    add_rl_rule("event-headers", pairwith={"headers": "event"})
    add_rl_rule("event-params", pairwith={"args": "event"})
    add_rl_rule("event-ipv4", key=[{"attrs": "path"}], pairwith={"attrs": "ip"})
    add_rl_rule("event-ipv6", key=[{"attrs": "path"}], pairwith={"attrs": "ip"})
    # "Provider" in the UI maps to "asn"
    add_rl_rule("event-provider", key=[{"attrs": "path"}], pairwith={"attrs": "asn"})
    add_rl_rule("event-uri", pairwith={"attrs": "uri"})
    add_rl_rule("event-path", pairwith={"attrs": "path"})
    add_rl_rule("event-query", pairwith={"attrs": "query"})
    add_rl_rule("event-method", pairwith={"attrs": "method"})
    add_rl_rule("event-company", key=[{"attrs": "path"}], pairwith={"attrs": "company"})
    add_rl_rule("event-country", key=[{"attrs": "path"}], pairwith={"attrs": "country"})
    add_rl_rule("event-authority", pairwith={"attrs": "authority"})
    # action
    add_rl_rule("action-challenge", action="challenge")
    add_rl_rule("action-monitor", action="monitor")
    add_rl_rule(
        "action-response",
        action="response",
        param_ext={"status": "123", "content": "Response body"},
    )
    add_rl_rule(
        "action-redirect",
        action="redirect",
        param_ext={"status": "124", "location": "/redirect/"},
    )
    add_rl_rule(
        "action-ban-503",
        action="ban",
        subaction="default",
        param_ext={"duration": "10"},
        excl_attrs={"tags": "allowlist"},
        incl_attrs={"tags": "blocklist"},
    )
    add_rl_rule(
        "action-ban-challenge",
        action="ban",
        subaction="challenge",
        param_ext={"duration": "10"},
        subaction_params={"action": {"type": "default", "params": {}}},
    )
    add_rl_rule(
        "action-ban-tagonly",
        action="ban",
        subaction="monitor",
        param_ext={"duration": "10"},
        subaction_params={"action": {"type": "default", "params": {}}},
    )
    add_rl_rule(
        "action-ban-response",
        action="ban",
        subaction="response",
        param_ext={"status": "123", "duration": "10", "content": "Content"},
        subaction_params={"content": "Response body", "status": "123"},
    )
    add_rl_rule(
        "action-ban-redirect",
        action="ban",
        subaction="redirect",
        param_ext={"duration": "10"},
        subaction_ext={"status": "124", "duration": "10", "location": "/redirect/"},
        subaction_params={
            "location": "/redirect",
            "status": "301",
            "action": {"type": "default", "params": {}},
        },
    )
    add_rl_rule(
        "action-ban-header",
        action="ban",
        subaction="request_header",
        param_ext={"duration": "10"},
        subaction_ext={"headers": "Header-Name"},
        subaction_params={
            "headers": {"foo": "bar"},
            "action": {"type": "default", "params": {}},
        },
    )
    add_rl_rule(
        "action-header",
        action="request_header",
        action_ext={"headers": "Header-Name"},
        param_ext={"headers": {"foo": "bar"}},
    )

    rl_securitypolicy = [
        {
            "id": "__default__",
            "name": "default entry",
            "match": "__default__",
            "map": [
                {
                    "name": "default",
                    "match": "/",
                    "acl_profile": "__default__",
                    "acl_active": True,
                    "content_filter_profile": "__default__",
                    "content_filter_active": True,
                    "limit_ids": [],
                }
            ]
            + [
                {
                    "name": k,
                    "match": f"/{k}/",
                    "acl_profile": "__default__",
                    "acl_active": True,
                    "content_filter_profile": "__default__",
                    "content_filter_active": True,
                    "limit_ids": [v],
                }
                for k, v in map_path.items()
            ],
        }
    ]
    return (rl_rules, rl_securitypolicy, prof_rules)


@pytest.fixture(scope="class")
def ratelimit_config(cli, target):
    cli.revert_and_enable()
    # Add new RL rules
    rl_rules = cli.call(f"doc get {TEST_CONFIG_NAME} ratelimits")
    (new_rules, new_securitypolicy, new_profiling) = gen_rl_rules(target.authority())
    rl_rules.extend(new_rules)
    # Apply new profiling
    cli.call(
        f"doc update {TEST_CONFIG_NAME} globalfilters /dev/stdin",
        inputjson=new_profiling,
    )
    # Apply rl_rules
    cli.call(f"doc update {TEST_CONFIG_NAME} ratelimits /dev/stdin", inputjson=rl_rules)
    # Apply new_securitypolicy
    cli.call(
        f"doc update {TEST_CONFIG_NAME} securitypolicies /dev/stdin",
        inputjson=new_securitypolicy,
    )
    cli.publish_and_apply()


class TestRateLimit:
    def test_ratelimit_scope_include(self, target, ratelimit_config, section):
        # rate limit: max 3 requests within 10 seconds
        param = {section: {"include": "true"}}
        for i in range(1, 4):
            assert target.is_reachable(
                f"/scope-{section}/include/{i}", **param
            ), f"Request #{i} for {section} should be allowed"
        assert not target.is_reachable(
            f"/scope-{section}/include/4", **param
        ), f"Request #4 for {section} should be blocked by the rate limit"
        time.sleep(10)
        assert target.is_reachable(
            f"/scope-{section}/include/5", **param
        ), f"Request #5 for {section} should be allowed"

    def test_ratelimit_scope_include_exclude(self, target, ratelimit_config, section):
        # rate limit: max 3 requests within 10 seconds
        param = {section: {"include": "true", "exclude": "true"}}
        for i in range(1, 5):
            assert target.is_reachable(
                f"/scope-{section}/include-exclude/{i}", **param
            ), f"Request #{i} for {section} should be allowed"

    def test_ratelimit_scope_exclude(self, target, ratelimit_config, section):
        # rate limit: max 3 requests within 10 seconds
        param = {section: {"exclude": "true"}}
        for i in range(1, 5):
            assert target.is_reachable(
                f"/scope-{section}/exclude/{i}", **param
            ), f"Request #{i} for {section} should be allowed"

    def test_ratelimit_scope_path_include(self, target, ratelimit_config):
        # rate limit: max 3 requests within 10 seconds
        for i in range(1, 4):
            assert target.is_reachable(
                f"/scope-path/include/{i}"
            ), f"Request #{i} for path should be allowed"
        assert not target.is_reachable(
            "/scope-path/include/4"
        ), "Request #4 for path should be blocked by the rate limit"
        time.sleep(10)
        assert target.is_reachable(
            "/scope-path/include/5"
        ), "Request #5 for path should be allowed"

    def test_ratelimit_scope_path_include_exclude(self, target, ratelimit_config):
        # rate limit: max 3 requests within 10 seconds
        for i in range(1, 5):
            assert target.is_reachable(
                f"/scope-path/include/exclude/{i}"
            ), f"Request #{i} for path should be allowed"

    def test_ratelimit_scope_uri_include(self, target, ratelimit_config):
        # rate limit: max 3 requests within 10 seconds
        for i in range(1, 4):
            assert target.is_reachable(
                f"/scope-uri/include/{i}"
            ), f"Request #{i} for uri should be allowed"
        assert not target.is_reachable(
            "/scope-uri/include/4"
        ), "Request #4 for uri should be blocked by the rate limit"
        time.sleep(10)
        assert target.is_reachable(
            "/scope-uri/include/5"
        ), "Request #5 for uri should be allowed"

    def test_ratelimit_scope_uri_include_exclude(self, target, ratelimit_config):
        # rate limit: max 3 requests within 10 seconds
        for i in range(1, 5):
            assert target.is_reachable(
                f"/scope-uri/include/exclude/{i}"
            ), f"Request #{i} for uri should be allowed"

    def test_ratelimit_scope_ipv4_include(self, target, ratelimit_config):
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-ipv4-include/included", srcip=IP4_US
            ), f"Request #{i} for included ipv4 should be allowed"
        assert not target.is_reachable(
            "/scope-ipv4-include/included", srcip=IP4_US
        ), "Request #4 for included ipv4 should be denied"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-ipv4-include/not-included", srcip=IP4_JP
            ), f"Request #{i} for non included ipv4 should be allowed"

    def test_ratelimit_scope_ipv4_exclude(self, target, ratelimit_config):
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-ipv4-exclude/excluded", srcip=IP4_US
            ), f"Request #{i} for excluded ipv4 should be allowed"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-ipv4-exclude/not-excluded", srcip=IP4_JP
            ), f"Request #{i} for non excluded ipv4 should be allowed"
        assert not target.is_reachable(
            "/scope-ipv4-exclude/not-excluded", srcip=IP4_JP
        ), "Request #4 for non excluded ipv4 should be denied"

    def test_ratelimit_scope_country_include(self, target, ratelimit_config):
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-country-include/included", srcip=IP4_US
            ), f"Request #{i} for included country should be allowed"
        assert not target.is_reachable(
            "/scope-country-include/included", srcip=IP4_US
        ), "Request #4 for included country should be denied"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-country-include/not-included", srcip=IP4_JP
            ), f"Request #{i} for non included country should be allowed"

    def test_ratelimit_scope_country_exclude(self, target, ratelimit_config):
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-country-exclude/excluded", srcip=IP4_US
            ), f"Request #{i} for excluded country should be allowed"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-country-exclude/not-excluded", srcip=IP4_JP
            ), f"Request #{i} for non excluded country should be allowed"
        assert not target.is_reachable(
            "/scope-country-exclude/not-excluded", srcip=IP4_JP
        ), "Request #4 for non excluded country should be denied"

    def test_ratelimit_scope_company_include(self, target, ratelimit_config):
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-company-include/included", srcip=IP4_CLOUDFLARE
            ), f"Request #{i} for included company should be allowed"
        assert not target.is_reachable(
            "/scope-company-include/included", srcip=IP4_CLOUDFLARE
        ), "Request #4 for included company should be denied"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-company-include/not-included", srcip=IP4_US
            ), f"Request #{i} for non included company should be allowed"

    def test_ratelimit_scope_company_exclude(self, target, ratelimit_config):
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-company-exclude/excluded", srcip=IP4_CLOUDFLARE
            ), f"Request #{i} for excluded company should be allowed"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-company-exclude/not-excluded", srcip=IP4_US
            ), f"Request #{i} for non excluded company should be allowed"
        assert not target.is_reachable(
            "/scope-company-exclude/not-excluded", srcip=IP4_US
        ), "Request #4 for non excluded company should be denied"

    def test_ratelimit_scope_provider_include(self, target, ratelimit_config):
        # "provider" means "asn"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-provider-include/included", srcip=IP4_US
            ), f"Request #{i} for included provider should be allowed"
        assert not target.is_reachable(
            "/scope-provider-include/included", srcip=IP4_US
        ), "Request #4 for included provider should be denied"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-provider-include/not-included", srcip=IP4_JP
            ), f"Request #{i} for non included provider should be allowed"

    def test_ratelimit_scope_provider_exclude(self, target, ratelimit_config):
        # "provider" means "asn"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-provider-exclude/excluded", srcip=IP4_US
            ), f"Request #{i} for excluded provider should be allowed"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-provider-exclude/not-excluded", srcip=IP4_JP
            ), f"Request #{i} for non excluded provider should be allowed"
        assert not target.is_reachable(
            "/scope-provider-exclude/not-excluded", srcip=IP4_JP
        ), "Request #4 for non excluded provider should be denied"

    def test_ratelimit_scope_method_include(self, target, ratelimit_config):
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-method-include/included"
            ), f"Request #{i} for included method should be allowed"
        assert not target.is_reachable(
            "/scope-method-include/included"
        ), "Request #4 for included method should be denied"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-method-include/not-included", method="HEAD"
            ), f"Request #{i} for non included method should be allowed"

    def test_ratelimit_scope_method_exclude(self, target, ratelimit_config):
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-method-exclude/excluded"
            ), f"Request #{i} for excluded method should be allowed"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-method-exclude/not-excluded", method="HEAD"
            ), f"Request #{i} for non excluded method should be allowed"
        assert not target.is_reachable(
            "/scope-method-exclude/not-excluded", method="HEAD"
        ), "Request #4 for non excluded method should be denied"

    def test_ratelimit_scope_query_include(self, target, ratelimit_config):
        # if "QUERY" is a substring of the query, rate limiting applies
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-query-include/included?QUERY"
            ), f"Request #{i} for included query should be allowed"
        assert not target.is_reachable(
            "/scope-query-include/included?QUERY"
        ), "Request #4 for included query should be denied"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-query-include/not-included?SOMETHINGELSE"
            ), f"Request #{i} for non included query should be allowed"

    def test_ratelimit_scope_query_exclude(self, target, ratelimit_config):
        # if "QUERY" is a substring of the query, rate limiting does not apply
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-query-exclude/excluded?QUERY"
            ), f"Request #{i} for excluded query should be allowed"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-query-exclude/not-excluded?SOMETHINGELSE"
            ), f"Request #{i} for non excluded query should be allowed"
        assert not target.is_reachable(
            "/scope-query-exclude/not-excluded?SOMETHINGELSE"
        ), "Request #4 for non excluded query should be denied"

    def test_ratelimit_scope_authority_include(self, target, ratelimit_config):
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-authority-include/included"
            ), f"Request #{i} for included authority should be allowed"
        assert not target.is_reachable(
            "/scope-authority-include/included"
        ), "Request #4 for included authority should be denied"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-other-authority-include/not-included"
            ), f"Request #{i} for non included authority should be allowed"

    def test_ratelimit_scope_authority_exclude(self, target, ratelimit_config):
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-authority-exclude/excluded"
            ), f"Request #{i} for excluded authority should be allowed"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-other-authority-exclude/not-excluded"
            ), f"Request #{i} for non excluded authority should be allowed"
        assert not target.is_reachable(
            "/scope-other-authority-exclude/not-excluded"
        ), "Request #4 for non excluded authority should be denied"

    def ratelimit_countby_helper(self, target, name, param1, param2, nocount=False):
        def disp(i):
            # do not change URLs when countby is set to uri or path
            if nocount:
                return ""
            return i

        for i in range(1, 4):
            assert target.is_reachable(
                f"/countby-{name}/1/{disp(i)}", **param1
            ), f"Request #{i} with {name} countby 1 should be allowed"
            assert target.is_reachable(
                f"/countby-{name}/2/{disp(i)}", **param2
            ), f"Request #{i} with {name} countby 2 should be allowed"
            # empty {name} -> not counted
            # assert target.is_reachable(f"/countby-{name}/3/{disp(i)}"), \
            #     f"Request #{i} with no {name} should be allowed"
        assert not target.is_reachable(
            f"/countby-{name}/2/{disp(4)}", **param1
        ), f"Request #4 with {name} countby 1 should be blocked"
        assert not target.is_reachable(
            f"/countby-{name}/2/{disp(4)}", **param2
        ), f"Request #4 with {name} countby 2 should be blocked"
        # assert not target.is_reachable(f"/countby-{name}/3/{disp(4)}"), \
        #     f"Request #{i} with no {name} should be denied"
        time.sleep(10)
        assert target.is_reachable(
            f"/countby-{name}/2/{disp(5)}", **param1
        ), f"Request #5 with {name} countby 1 should be allowed"
        assert target.is_reachable(
            f"/countby-{name}/2/{disp(5)}", **param2
        ), f"Request #5 with {name} countby 2 should be allowed"
        # assert target.is_reachable(f"/countby-{name}/3/{disp(5)}"), \
        #     f"Request #{i} with no {name} should be denied"

    def test_ratelimit_countby_section(self, target, ratelimit_config, section):
        param1 = {section: {"countby": "1"}}
        param2 = {section: {"countby": "2"}}
        self.ratelimit_countby_helper(target, section, param1, param2)

    def test_ratelimit_countby_ipv4(self, target, ratelimit_config):
        param1 = {"srcip": IP4_US}
        param2 = {"srcip": IP4_JP}
        self.ratelimit_countby_helper(target, "ipv4", param1, param2)

    def test_ratelimit_countby_ipv6(self, target, ratelimit_config):
        param1 = {"srcip": IP6_1}
        param2 = {"srcip": IP6_2}
        self.ratelimit_countby_helper(target, "ipv6", param1, param2)

    def test_ratelimit_countby_provider(self, target, ratelimit_config):
        # "provider" means "asn"
        param1 = {"srcip": IP4_US}
        param2 = {"srcip": IP4_JP}
        self.ratelimit_countby_helper(target, "provider", param1, param2)

    def test_ratelimit_countby_uri(self, target, ratelimit_config):
        param1 = {}
        param2 = {}
        self.ratelimit_countby_helper(target, "uri", param1, param2, nocount=True)

    def test_ratelimit_countby_path(self, target, ratelimit_config):
        param1 = {}
        param2 = {}
        self.ratelimit_countby_helper(target, "path", param1, param2, nocount=True)

    def test_ratelimit_countby_query(self, target, ratelimit_config):
        param1 = {"suffix": "?QUERY-1"}
        param2 = {"suffix": "?QUERY-2"}
        self.ratelimit_countby_helper(target, "query", param1, param2)

    def test_ratelimit_countby_method(self, target, ratelimit_config):
        param1 = {"method": "HEAD"}
        param2 = {"method": "GET"}
        self.ratelimit_countby_helper(target, "method", param1, param2)

    def test_ratelimit_countby_company(self, target, ratelimit_config):
        param1 = {"srcip": IP4_US}
        param2 = {"srcip": IP4_JP}
        self.ratelimit_countby_helper(target, "company", param1, param2)

    def test_ratelimit_countby_country(self, target, ratelimit_config):
        param1 = {"srcip": IP4_US}
        param2 = {"srcip": IP4_JP}
        self.ratelimit_countby_helper(target, "country", param1, param2)

    def test_ratelimit_countby_authority(self, target, ratelimit_config):
        param1 = {"headers": {"Host": "authority-1"}}
        param2 = {"headers": {"Host": "authority-2"}}
        self.ratelimit_countby_helper(target, "authority", param1, param2)

    def test_ratelimit_countby2_section(self, target, ratelimit_config, section):
        param1 = {section: {"countby1": "1"}}
        param2 = {section: {"countby2": "1"}}
        param12 = {section: {"countby1": "1", "countby2": "1"}}
        for i in range(1, 4):
            assert target.is_reachable(
                f"/countby2-{section}/1/{i}", **param1
            ), f"Request #{i} with {section} countby 1 should be allowed"
            assert target.is_reachable(
                f"/countby2-{section}/2/{i}", **param2
            ), f"Request #{i} with {section} countby 2 should be allowed"
            assert target.is_reachable(
                f"/countby2-{section}/2/{i}", **param12
            ), f"Request #{i} with {section} countby 1&2 should be allowed"
        assert target.is_reachable(
            f"/countby2-{section}/2/4", **param1
        ), f"Request #4 with {section} countby 1 should not be blocked"
        assert target.is_reachable(
            f"/countby2-{section}/2/4", **param2
        ), f"Request #4 with {section} countby 2 should not be blocked"
        assert not target.is_reachable(
            f"/countby2-{section}/2/4", **param12
        ), f"Request #4 with {section} countby 1&2 should be blocked"
        time.sleep(10)
        assert target.is_reachable(
            f"/countby2-{section}/2/5", **param1
        ), f"Request #5 with {section} countby 1 should be allowed"
        assert target.is_reachable(
            f"/countby2-{section}/2/5", **param2
        ), f"Request #5 with {section} countby 2 should be allowed"
        assert target.is_reachable(
            f"/countby2-{section}/2/5", **param12
        ), f"Request #5 with {section} countby 1&2 should be allowed"

    def test_ratelimit_countby_2sections(self, target, ratelimit_config, section):
        # condition: have countby set for 2 sections
        othersection = {"headers": "params", "cookies": "headers", "params": "cookies"}[
            section
        ]
        param1 = {section: {"countby": "1"}}
        param2 = {othersection: {"countby": "1"}}
        param12 = {section: {"countby": "1"}, othersection: {"countby": "1"}}
        for i in range(1, 4):
            assert target.is_reachable(
                f"/countby-{section}-{othersection}/1/{i}", **param1
            ), f"Request #{i} with {section} countby 1 should be allowed"
            assert target.is_reachable(
                f"/countby-{section}-{othersection}/2/{i}", **param2
            ), f"Request #{i} with {section} countby 2 should be allowed"
            assert target.is_reachable(
                f"/countby-{section}-{othersection}/2/{i}", **param12
            ), f"Request #{i} with {section} countby 1&2 should be allowed"
        assert target.is_reachable(
            f"/countby-{section}-{othersection}/2/4", **param1
        ), f"Request #4 with {section} countby 1 should not be blocked"
        assert target.is_reachable(
            f"/countby-{section}-{othersection}/2/4", **param2
        ), f"Request #4 with {section} countby 2 should not be blocked"
        assert not target.is_reachable(
            f"/countby-{section}-{othersection}/2/4", **param12
        ), f"Request #4 with {section} countby 1&2 should be blocked"
        time.sleep(10)
        assert target.is_reachable(
            f"/countby-{section}-{othersection}/2/5", **param1
        ), f"Request #5 with {section} countby 1 should be allowed"
        assert target.is_reachable(
            f"/countby-{section}-{othersection}/2/5", **param2
        ), f"Request #5 with {section} countby 2 should be allowed"
        assert target.is_reachable(
            f"/countby-{section}-{othersection}/2/5", **param12
        ), f"Request #5 with {section} countby 1&2 should be allowed"

    def ratelimit_event_param_helper(self, target, name, params):
        limit = len(params)
        for i in range(limit - 1):
            assert target.is_reachable(
                f"/event-{name}/1/", **params[i]
            ), f"Request for value #{i+1} with {name} event should be allowed"
        assert not target.is_reachable(
            f"/event-{name}/1/", **params[limit - 1]
        ), f"Request for value #{limit} with {name} event should be denied"
        for i in range(limit):
            assert not target.is_reachable(
                f"/event-{name}/1/", **params[i]
            ), f"Request for value #{i+1} with {name} event should be denied"
        time.sleep(10)
        for i in range(limit - 1):
            assert target.is_reachable(
                f"/event-{name}/1/", **params[i]
            ), f"Request for value #{i+1} with {name} event should be allowed"

    def test_ratelimit_event_section(self, target, ratelimit_config, section):
        params = [{section: {"event": f"{i}"}} for i in range(1, 5)]
        self.ratelimit_event_param_helper(target, section, params)

    def test_ratelimit_event_ipv4(self, target, ratelimit_config):
        params = [{"srcip": f"199.0.0.{i}"} for i in range(1, 5)]
        self.ratelimit_event_param_helper(target, "ipv4", params)

    def test_ratelimit_event_ipv6(self, target, ratelimit_config):
        params = [
            {"srcip": f"0000:0000:0000:0000:0000:0000:0000:000{i}"} for i in range(1, 5)
        ]
        self.ratelimit_event_param_helper(target, "ipv6", params)

    def test_ratelimit_event_provider(self, target, ratelimit_config):
        # "provider" means "asn"
        params = [{"srcip": ip} for ip in (IP4_US, IP4_JP, IP4_CLOUDFLARE, IP4_ORANGE)]
        self.ratelimit_event_param_helper(target, "provider", params)

    def test_ratelimit_event_uri(self, target, ratelimit_config):
        # URI is different for each query, nothing more needs changing
        params = [{"suffix": f"{i}"} for i in range(1, 5)]
        self.ratelimit_event_param_helper(target, "uri", params)

    def test_ratelimit_event_path(self, target, ratelimit_config):
        # Path is different for each query, nothing more needs changing
        params = [{"suffix": f"{i}"} for i in range(1, 5)]
        self.ratelimit_event_param_helper(target, "path", params)

    def test_ratelimit_event_query(self, target, ratelimit_config):
        params = [{"suffix": f"?QUERY-{i}"} for i in range(1, 5)]
        self.ratelimit_event_param_helper(target, "query", params)

    def test_ratelimit_event_method(self, target, ratelimit_config):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        self.ratelimit_event_param_helper(target, "method", params)

    def test_ratelimit_event_company(self, target, ratelimit_config):
        params = [{"srcip": ip} for ip in (IP4_US, IP4_JP, IP4_CLOUDFLARE, IP4_ORANGE)]
        self.ratelimit_event_param_helper(
            target,
            "company",
            params,
        )

    def test_ratelimit_event_country(self, target, ratelimit_config):
        params = [{"srcip": ip} for ip in (IP4_US, IP4_JP, IP4_CLOUDFLARE, IP4_ORANGE)]
        self.ratelimit_event_param_helper(target, "country", params)

    def test_ratelimit_event_authority(self, target, ratelimit_config):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(1, 5)]
        self.ratelimit_event_param_helper(target, "authority", params)


# --- Tag rules tests (formerly profiling lists) ---

TEST_GLOBALFILTERS = {
    "id": "e2e000000000",
    "name": "e2e test tag rules",
    "source": "self-managed",
    "mdate": "2020-11-22T00:00:00.000Z",
    "description": "E2E test tag rules",
    "entries_relation": "OR",
    "active": True,
    "tags": ["e2e-test"],
    "rule": {
        "relation": "OR",
        "sections": [
            {
                "relation": "OR",
                "entries": [
                    ["cookies", ["e2e", "value"], "annotation"],
                    ["headers", ["e2e", "value"], "annotation"],
                    ["method", "(POST|PUT)", "annotation"],
                    ["path", "/e2e-globalfilters-path/", "annotation"],
                    ["query", "e2e=value", "annotation"],
                    ["uri", "/e2e-globalfilters-uri", "annotation"],
                    ["ip", IP6_1, "annotation"],
                    ["ip", IP4_US, "annotation"],
                    [
                        "country",
                        "jp",
                        "annotation",
                    ],  # TODO: discuss is this should work using caps
                    ["asn", "13335", "annotation"],
                ],
            },
            {
                "relation": "AND",
                "entries": [
                    ["path", "/e2e-and/", "annotation"],
                    ["cookies", ["e2e-and", "value"], "annotation"],
                ],
            },
        ],
    },
}


@pytest.fixture(scope="session", params=[True, False], ids=["active", "inactive"])
def active(request):
    return request.param


@pytest.fixture(scope="class")
def globalfilters_config(cli, acl, active):
    cli.revert_and_enable()
    acl.set_acl({"force_deny": "e2e-test", "passthrough": "all"})
    # Apply TEST_GLOBALFILTERS
    TEST_GLOBALFILTERS["active"] = active
    # 'updating' contentfilterprofiles with a list containing a single entry adds this
    # entry, without removing pre-existing ones.
    cli.call(
        f"doc update {TEST_CONFIG_NAME} globalfilters /dev/stdin",
        inputjson=[TEST_GLOBALFILTERS],
    )
    cli.publish_and_apply()


class TestGlobalFilters:
    def test_cookies(self, target, globalfilters_config, active):
        assert (
            target.is_reachable("/e2e-globalfilters-cookies", cookies={"e2e": "value"})
            is not active
        )
        assert (
            target.is_reachable(
                "/e2e-globalfilters-cookies", cookies={"e2e": "allowed"}
            )
            is True
        )

    def test_headers(self, target, globalfilters_config, active):
        assert (
            target.is_reachable("/e2e-globalfilters-headers", headers={"e2e": "value"})
            is not active
        )
        assert (
            target.is_reachable(
                "/e2e-globalfilters-headers", headers={"e2e": "allowed"}
            )
            is True
        )

    def test_method(self, target, globalfilters_config, active):
        assert (
            target.is_reachable("/e2e-globalfilters-method-GET", method="GET") is True
        )
        assert (
            target.is_reachable("/e2e-globalfilters-method-POST", method="POST")
            is not active
        )
        assert (
            target.is_reachable("/e2e-globalfilters-method-PUT", method="PUT")
            is not active
        )

    def test_path(self, target, globalfilters_config, active):
        assert target.is_reachable("/e2e-globalfilters-path/") is not active
        assert target.is_reachable("/e2e-globalfilters-valid-path/") is True

    def test_query(self, target, globalfilters_config, active):
        assert (
            target.is_reachable("/e2e-globalfilters-query", params={"e2e": "value"})
            is not active
        )
        assert (
            target.is_reachable("/e2e-globalfilters-query", params={"e2e": "allowed"})
            is True
        )

    def test_uri(self, target, globalfilters_config, active):
        assert target.is_reachable("/e2e-globalfilters-uri") is not active
        assert target.is_reachable("/e2e-globalfilters-allowed-uri") is True

    def test_ipv4(self, target, globalfilters_config, active):
        assert target.is_reachable("/tag-ipv4-1", srcip=IP4_US) is not active
        assert target.is_reachable("/tag-ipv4-2", srcip=IP4_ORANGE) is True

    def test_ipv6(self, target, globalfilters_config, active):
        assert target.is_reachable("/tag-ipv6-1", srcip=IP6_1) is not active
        assert target.is_reachable("/tag-ipv6-2", srcip=IP6_2) is True

    def test_country(self, target, globalfilters_config, active):
        # JP address (Softbank)
        assert target.is_reachable("/tag-country", srcip=IP4_JP) is not active

    def test_asn(self, target, globalfilters_config, active):
        # ASN 13335
        assert target.is_reachable("/tag-asn", srcip=IP4_CLOUDFLARE) is not active

    def test_and(self, target, globalfilters_config, active):
        assert (
            target.is_reachable("/e2e-and/", cookies={"e2e-and": "value"}) is not active
        )
        assert (
            target.is_reachable("/not-e2e-and/", cookies={"e2e-and": "value"}) is True
        )
        assert (
            target.is_reachable("/e2e-and/", cookies={"not-e2e-and": "value"}) is True
        )


# --- Security Policies tests ---


ACL_BYPASSALL = {
    "id": "e2e00ac10000",
    "name": "e2e-denyall-acl",
    "allow": [],
    "allow_bot": [],
    "deny_bot": [],
    "passthrough": ["all"],
    "force_deny": [],
    "deny": [],
}

CONTENT_FILTER_SHORT_HEADERS = {
    "id": "e2e000000002",
    "name": "e2e content filter short headers",
    "ignore_alphanum": True,
    "args": {"names": [], "regex": []},
    "headers": {"max_length": 50, "max_count": 42, "names": [], "regex": []},
    "cookies": {"names": [], "regex": []},
}

CONTENT_FILTER_MISC_HEADERS = {
    "id": "e2e000000002m",
    "name": "e2e waf misc headers",
    "ignore_alphanum": False,
    "args": {
        "max_count": 5,
        "max_length": 1024,
        "min_risk": 1,
        "names": [
            {
                "exclusions": {},
                "key": "a",
                "mask": False,
                "reg": "^[A-Z]+",
                "restrict": False,
            },
            {
                "exclusions": {},
                "key": "b",
                "mask": False,
                "reg": "^[A-Z]+",
                "restrict": True,
            },
            {
                "exclusions": {},
                "key": "c",
                "mask": True,
                "reg": "^[A-Z]+",
                "restrict": True,
            },
            {
                "exclusions": {},
                "key": "d",
                "mask": True,
                "reg": "^[A-Z]+",
                "restrict": False,
            },
        ],
        "regex": [],
    },
    "headers": {
        "names": [],
        "regex": [],
        "max_length": 50,
        "max_count": 42,
        "min_risk": 3,
    },
    "cookies": {
        "names": [],
        "regex": [],
        "max_length": 1024,
        "max_count": 42,
        "min_risk": 4,
    },
    "path": {
        "names": [],
        "regex": [],
        "max_length": 1024,
        "max_count": 42,
        "min_risk": 5,
    },
}

SECURITYPOLICY = [
    {
        "id": "e2e000000001",
        "name": "e2e Security Policy",
        "match": ".*",
        "map": [
            {
                "name": "acl",
                "match": "/acl/",
                "acl_profile": "__default__",
                "acl_active": True,
                "content_filter_profile": "__default__",
                "content_filter_active": False,
                "limit_ids": [],
                "isnew": True,
            },
            {
                "name": "acl-passthroughall",
                "match": "/acl-passthroughall/",
                "acl_profile": "e2e00ac10000",
                "acl_active": True,
                "content_filter_profile": "__default__",
                "content_filter_active": True,
                "limit_ids": [],
                "isnew": True,
            },
            {
                "name": "acl-content-filter",
                "match": "/acl-content-filter/",
                "acl_profile": "__default__",
                "acl_active": True,
                "content_filter_profile": "__default__",
                "content_filter_active": True,
                "limit_ids": [],
                "isnew": True,
            },
            {
                "name": "content-filter",
                "match": "/content-filter/",
                "acl_profile": "__default__",
                "acl_active": False,
                "content_filter_profile": "__default__",
                "content_filter_active": True,
                "limit_ids": [],
                "isnew": True,
            },
            {
                "name": "content-filter-short-headers",
                "match": "/content-filter-short-headers/",
                "acl_profile": "__default__",
                "acl_active": False,
                "content_filter_profile": "e2e000000002",
                "content_filter_active": True,
                "limit_ids": [],
                "isnew": True,
            },
            {
                "name": "waf-misc-headers",
                "match": "/waf-misc-headers/",
                "acl_profile": "__default__",
                "acl_active": False,
                "content_filter_profile": "e2e000000002m",
                "content_filter_active": True,
                "limit_ids": [],
                "isnew": True,
            },
            {
                "name": "nofilter",
                "match": "/nofilter/",
                "acl_profile": "__default__",
                "acl_active": False,
                "content_filter_profile": "__default__",
                "content_filter_active": False,
                "limit_ids": [],
            },
        ],
    }
]


@pytest.fixture(scope="class")
def securitypolicy_config(cli, acl):
    cli.revert_and_enable()
    # Add ACL entry
    default_acl = cli.empty_acl()
    default_acl[0]["force_deny"].append("all")
    default_acl.append(ACL_BYPASSALL)
    cli.call(
        f"doc update {TEST_CONFIG_NAME} aclprofiles /dev/stdin", inputjson=default_acl
    )
    # Add content filter profile entry
    contentfilterprofile = cli.call(f"doc get {TEST_CONFIG_NAME} contentfilterprofiles")
    contentfilterprofile.append(CONTENT_FILTER_SHORT_HEADERS)
    contentfilterprofile.append(CONTENT_FILTER_MISC_HEADERS)
    cli.call(
        f"doc update {TEST_CONFIG_NAME} contentfilterprofiles /dev/stdin",
        inputjson=contentfilterprofile,
    )
    # Add securitypolicy entry SECURITYPOLICY
    cli.call(
        f"doc update {TEST_CONFIG_NAME} securitypolicies /dev/stdin",
        inputjson=SECURITYPOLICY,
    )
    cli.publish_and_apply()


class TestSecurityPolicy:
    def test_nofilter(self, target, securitypolicy_config):
        assert target.is_reachable("/nofilter/")
        assert target.is_reachable(
            "/nofilter/", headers={"Long-header": "Overlong_header" * 100}
        )

    def test_content_filter(self, target, securitypolicy_config):
        assert target.is_reachable("/content-filter/")
        assert not target.is_reachable(
            "/content-filter/", headers={"Long-header": "Overlong_header" * 100}
        )

    def test_aclfilter(self, target, securitypolicy_config):
        assert not target.is_reachable("/acl/")
        assert not target.is_reachable(
            "/acl/", headers={"Long-header": "Overlong_header" * 100}
        )

    def test_nondefault_aclfilter_passthroughall(self, target, securitypolicy_config):
        assert target.is_reachable("/acl-passthroughall/")
        assert target.is_reachable(
            "/acl-passthroughall/", headers={"Long-header": "Overlong_header" * 100}
        )

    def test_acl_content_filter(self, target, securitypolicy_config):
        assert not target.is_reachable("/acl-content-filter/")
        assert not target.is_reachable(
            "/acl/", headers={"Long-header": "Overlong_header" * 100}
        )

    def test_nondefault_content_filter_profile_short_headers(
        self, target, securitypolicy_config
    ):
        assert target.is_reachable(
            "/content-filter-short-headers/", headers={"Short-header": "0123456789" * 5}
        )
        assert not target.is_reachable(
            "/content-filter-short-headers/",
            headers={"Long-header": "0123456789" * 5 + "A"},
        )

    def test_unrestricted_waf_match(self, target, securitypolicy_config):
        assert target.is_reachable("/waf-misc-headers/x?a=ABCDE")

    def test_unrestricted_waf_nomatch(self, target, securitypolicy_config):
        assert target.is_reachable("/waf-misc-headers/x?a=12345")

    def test_restricted_waf_match(self, target, securitypolicy_config):
        assert target.is_reachable("/waf-misc-headers/x?b=ABCDE")

    def test_restricted_waf_nomatch(self, target, securitypolicy_config):
        assert not target.is_reachable("/waf-misc-headers/x?b=12345")


# --- Content Filter Profiles tests ---


class TestContentFilterLengthCount:
    def test_length_overlong(self, default_config, target, section):
        # default limit: len 1024
        assert not target.is_reachable(
            f"/overlong-{section}",
            **{section: {f"Long-{section}": f"Overlong_{section}" * 100}},
        ), f"Reachable despite overlong {section}"

    def test_length_short(self, default_config, target, section):
        assert target.is_reachable(
            f"/short-{section}", headers={f"Short-{section}": f"Short_{section}"}
        ), f"Not reachable despite short {section}"

    def test_count_few(self, default_config, target, section):
        # default limit: 512 for args, 42 for other sections
        values = {}
        for i in range(10):
            values[f"{section}-{i}"] = "not_alphanum"
        assert target.is_reachable(
            f"/few-{section}", **{section: values}
        ), f"Not reachable despite few {section}"

    def test_count_toomany(self, default_config, target, section):
        values = {}
        for i in range(513):
            values[f"{section}-{i}"] = "not_alphanum"
        assert not target.is_reachable(
            f"/too-many-{section}", **{section: values}
        ), f"Reachable despite too many {section}"


CONTENT_FILTER_PARAM_CONSTRAINTS = {
    "names": [
        {
            "key": "name-norestrict",
            "reg": "[v]+[a]{1}l?u*e",
            "restrict": False,
            "exclusions": {"100140": "rule"},
        },
        {
            "key": "name-restrict",
            "reg": "[v]+[a]{1}l?u*e",
            "restrict": True,
            "exclusions": {},
        },
    ],
    "regex": [
        {
            "key": "reg[e]x{1}-norestrict",
            "reg": "[v]+[a]{1}l?u*e",
            "restrict": False,
            "exclusions": {"100140": "rule"},
        },
        {
            "key": "reg[e]x{1}-restrict",
            "reg": "[v]+[a]{1}l?u*e",
            "restrict": True,
            "exclusions": {},
        },
    ],
}


@pytest.fixture(
    scope="session", params=[True, False], ids=["ignore_alphanum", "no_ignore_alphanum"]
)
def ignore_alphanum(request):
    return request.param


@pytest.fixture(scope="class")
def content_filter_param_config(cli, request, ignore_alphanum):
    cli.revert_and_enable()
    # Apply CONTENT_FILTER_PARAM_CONSTRAINTS
    contentfilterprofile = cli.call(f"doc get {TEST_CONFIG_NAME} contentfilterprofiles")
    for k in ("args", "headers", "cookies", "path"):
        contentfilterprofile[0][k] = CONTENT_FILTER_PARAM_CONSTRAINTS
    contentfilterprofile[0]["ignore_alphanum"] = ignore_alphanum
    cli.call(
        f"doc update {TEST_CONFIG_NAME} contentfilterprofiles /dev/stdin",
        inputjson=contentfilterprofile,
    )

    cli.publish_and_apply()


@pytest.fixture(scope="function", params=["name", "regex"])
def name_regex(request):
    return request.param


@pytest.fixture(scope="function", params=["restrict", "norestrict"])
def restrict(request):
    return request.param


class TestContentFilterParamsConstraints:
    def test_allowlisted_value(
        self, content_filter_param_config, section, name_regex, restrict, target
    ):
        paramname = name_regex + "-" + restrict
        assert target.is_reachable(
            f"/allowlisted-value-{paramname}", **{section: {paramname: "value"}}
        ), f"Not reachable despite allowlisted {section} value"

    def test_non_allowlisted_value_restrict(
        self, content_filter_param_config, section, name_regex, target, ignore_alphanum
    ):
        paramname = name_regex + "-restrict"
        if ignore_alphanum:
            assert target.is_reachable(
                f"/blocklisted-value-{paramname}-restrict-ignore_alphanum",
                **{section: {paramname: "invalid"}},
            ), f"Not reachable despite alphanum blocklisted {section} value (restrict is enabled)"
        else:
            assert not target.is_reachable(
                f"/blocklisted-value-{paramname}-restrict",
                **{section: {paramname: "invalid"}},
            ), f"Reachable despite blocklisted {section} value (restrict is enabled)"

    def test_non_allowlisted_value_norestrict_no_content_filter_match(
        self, content_filter_param_config, section, name_regex, target
    ):
        paramname = name_regex + "-norestrict"
        assert target.is_reachable(
            f"/blocklisted-value-{paramname}", **{section: {paramname: "invalid"}}
        ), f"Not reachable despite 'restricted' not checked (non-matching {section} value)"

    def test_non_allowlisted_value_norestrict_content_filter_match(
        self, content_filter_param_config, section, name_regex, target
    ):
        paramname = name_regex + "-norestrict"
        assert not target.is_reachable(
            f"/blocklisted-value-{paramname}-content-filter-match",
            **{section: {paramname: "../../../../../"}},
        ), f"Reachable despite matching content filter rule 100116 (non-matching {section} value)"

    def test_non_allowlisted_value_norestrict_content_filter_match_excludesig(
        self, content_filter_param_config, section, name_regex, target
    ):
        paramname = name_regex + "-norestrict"
        assert target.is_reachable(
            f"/blocklisted-value-{paramname}-content-filter-match-excludedsig",
            **{section: {paramname: "htaccess"}},
        ), f"Not reachable despite excludesig for rule 100140 ({section} value)"


# --- Content Filter Rules tests ---


@pytest.fixture(
    scope="function", params=[(100140, "htaccess"), (100112, "../../../../../")]
)
def content_filter_rules(request):
    return request.param


class TestContentFilterRules:
    def test_content_filter_rule(
        self,
        content_filter_param_config,
        target,
        section,
        content_filter_rules,
        ignore_alphanum,
    ):
        ruleid, rulestr = content_filter_rules
        has_nonalpha = "." in rulestr
        if ignore_alphanum and not has_nonalpha:
            assert target.is_reachable(
                f"/content-filter-rule-{section}", **{section: {"key": rulestr}}
            ), f"Unreachable despite ignore_alphanum=True for rule {ruleid}"
        else:
            assert not target.is_reachable(
                f"/content-filter-rule-{section}", **{section: {"key": rulestr}}
            ), f"Reachable despite matching rule {ruleid}"
