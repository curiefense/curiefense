import pytest
from e2e.core.base_helper import cli, target, log_fixture, BaseHelper
import time
from typing import List, Optional


class RateLimitHelper:

    @staticmethod
    def check_503_response(target, path, ttl, limit):
        for i in range(limit):
            response = target.query(f"/{path}")
            assert response.status_code == 200
        response = target.query(f"/{path}")
        assert response.status_code == 503
        time.sleep(ttl)
        response = target.query(f"/{path}")
        assert response.status_code == 200

    @staticmethod
    def check_rate_limits_action_503_with_params(target, path, ttl, limit, params):
        for i in range(limit):
            response = target.query(f"/{path}", **params[i])
            assert response.status_code == 200
        response = target.query(f"/{path}", **params[limit])
        assert response.status_code == 503
        time.sleep(ttl)
        response = target.query(f"/{path}", **params[limit + 1])
        assert response.status_code == 200

    @staticmethod
    def check_503_response_change_path(target, path, ttl, limit):
        for i in range(limit):
            response = target.query(f"/{path}{i}")
            assert response.status_code == 200
        response = target.query(f"/{path}{limit}")
        assert response.status_code == 503
        time.sleep(ttl)
        response = target.query(f"/{path}{limit + 1}")
        assert response.status_code == 200

    @staticmethod
    def check_rate_limits_action_tag_only_with_pattern(log_fixture, target, path, field, pattern, limit):
        val_of_log = 0
        for i in range(limit):
            response = target.query(f"/{path}")
            assert response.status_code == 200
            val_of_log = log_fixture.check_log_pattern_updates(field, pattern)
        response = target.query(f"/{path}")
        assert response.status_code == 200
        assert log_fixture.check_log_pattern_updates(field, pattern) == val_of_log + 1

    @staticmethod
    def check_rl_action_tag_with_pattern_with_params(log_fixture, target, path, field, pattern, limit, params):
        val_of_log = 0
        for i in range(limit):
            response = target.query(f"/{path}", **params[i])
            assert response.status_code == 200
            val_of_log = log_fixture.check_log_pattern_updates(field, pattern)
        response = target.query(f"/{path}", **params[limit])
        new_val_of_log = log_fixture.check_log_pattern_updates(field, pattern)
        assert response.status_code == 200
        assert new_val_of_log == val_of_log + 1

    @staticmethod
    def check_rate_limits_response_action(target, path, response_body, status, ttl, limit):
        for i in range(limit):
            response = target.query(f"/{path}")
            assert response.status_code == 200
            assert response_body not in response.text
        response = target.query(f"/{path}")
        assert response.status_code == int(status)
        assert response_body in response.text
        time.sleep(ttl)
        response = target.query(f"/{path}")
        assert response.status_code == 200
        assert response_body not in response.text

    @staticmethod
    def check_rl_response_action_with_params(target, path, response_body, status, ttl, limit, params):
        for i in range(limit):
            response = target.query(f"/{path}", **params[i])
            assert response.status_code == 200
            assert response_body not in response.text
        response = target.query(f"/{path}", **params[limit])
        assert response.status_code == int(status)
        assert response_body in response.text
        time.sleep(ttl)
        response = target.query(f"/{path}", **params[limit + 1])
        assert response.status_code == 200
        assert response_body not in response.text

    @staticmethod
    def check_rate_limit_challenge_action(target, path, ttl, limit):
        for i in range(limit):
            response = target.query(f"/{path}")
            assert response.status_code == 200
            assert not BaseHelper.verify_pattern_in_html(response.content, "<html><head><meta")
        response = target.query(f"/{path}")
        assert response.status_code == 247
        assert BaseHelper.verify_pattern_in_html(response.content, "<html><head><meta")
        time.sleep(ttl)
        response = target.query(f"/{path}")
        assert response.status_code == 200
        assert not BaseHelper.verify_pattern_in_html(response.content, "<html><head><meta")

    @staticmethod
    def check_rate_limit_challenge_action_with_params(target, path, ttl, limit, params):
        for i in range(limit):
            response = target.query(f"/{path}", **params[i])
            assert response.status_code == 200
            assert not BaseHelper.verify_pattern_in_html(response.content, "<html><head><meta")
        response = target.query(f"/{path}", **params[limit])
        assert response.status_code == 247
        assert BaseHelper.verify_pattern_in_html(response.content, "<html><head><meta")
        time.sleep(ttl)
        response = target.query(f"/{path}", **params[limit + 1])
        assert response.status_code == 200
        assert not BaseHelper.verify_pattern_in_html(response.content, "<html><head><meta")

    @staticmethod
    def check_challenge_response_change_path(target, path, ttl, limit):
        for i in range(limit):
            response = target.query(f"/{path}{i}")
            assert response.status_code == 200
            assert not BaseHelper.verify_pattern_in_html(response.content, "<html><head><meta")
        response = target.query(f"/{path}{limit}")
        assert response.status_code == 247
        assert BaseHelper.verify_pattern_in_html(response.content, "<html><head><meta")
        time.sleep(ttl)
        response = target.query(f"/{path}{limit + 1}")
        assert response.status_code == 200
        assert not BaseHelper.verify_pattern_in_html(response.content, "<html><head><meta")

    @staticmethod
    def check_rate_limit_redirect_action(target, path, status, ttl, limit, location):
        for i in range(limit):
            response = target.query(f"/{path}")
            assert response.status_code == 200
            assert 'location' not in response.headers
        response = target.query(f"/{path}")
        assert response.status_code == int(status)
        assert 'location' in response.headers
        assert response.headers['location'] == location
        time.sleep(ttl)
        response = target.query(f"/{path}")
        assert response.status_code == 200
        assert 'location' not in response.headers

    @staticmethod
    def check_rl_redirect_action_with_params(target, path, status, ttl, limit, location, params):
        for i in range(limit):
            response = target.query(f"/{path}", **params[i])
            assert response.status_code == 200
            assert 'location' not in response.headers
        response = target.query(f"/{path}", **params[limit])
        assert response.status_code == int(status)
        assert 'location' in response.headers
        assert response.headers['location'] == location
        time.sleep(ttl)
        response = target.query(f"/{path}", **params[limit + 1])
        assert response.status_code == 200
        assert 'location' not in response.headers

    @staticmethod
    def remove_include_exclude_from_ratelimit_json(rule):
        rule["include"], rule["exclude"] = [], []
        return rule

    @staticmethod
    def ratelimit_countby_helper(target, name, param1, param2, nocount=False):
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

    @staticmethod
    def ratelimit_event_param_helper(target, name, params):
        limit = len(params)
        for i in range(limit - 1):
            assert target.is_reachable(
                f"/event-{name}/1/", **params[i]
            ), f"Request for value #{i + 1} with {name} event should be allowed"
        assert not target.is_reachable(
            f"/event-{name}/1/", **params[limit - 1]
        ), f"Request for value #{limit} with {name} event should be denied"
        for i in range(limit):
            assert not target.is_reachable(
                f"/event-{name}/1/", **params[i]
            ), f"Request for value #{i + 1} with {name} event should be denied"
        time.sleep(10)
        for i in range(limit - 1):
            assert target.is_reachable(
                f"/event-{name}/1/", **params[i]
            ), f"Request for value #{i + 1} with {name} event should be allowed"

    @staticmethod
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
        add_rl_rule("scope-ipv4-include", incl_attrs={"ip": BaseHelper.IP4_US})
        add_rl_rule("scope-ipv4-exclude", excl_attrs={"ip": BaseHelper.IP4_US})
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
            param_ext={"status": "123", "ttl": "10", "content": "Content"},
            subaction_params={"content": "Response body", "status": "123"},
        )
        add_rl_rule(
            "action-ban-redirect",
            action="ban",
            subaction="redirect",
            param_ext={"duration": "10"},
            subaction_ext={"status": "124", "ttl": "10", "location": "/redirect/"},
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

        rl_urlmap = [
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
        return (rl_rules, rl_urlmap, prof_rules)


@pytest.fixture(scope="class")
def ratelimit_config(cli, target, api_config):
    cli.revert_and_enable()
    # Add new RL rules
    rl_rules = cli.call(f"doc get {BaseHelper.TEST_CONFIG_NAME} ratelimits")
    (new_rules, new_urlmap, new_profiling) = RateLimitHelper.gen_rl_rules(target.authority())
    rl_rules.extend(new_rules)
    # Apply new profiling
    cli.call(
        f"doc update {BaseHelper.TEST_CONFIG_NAME} {api_config['url_map']} /dev/stdin",
        inputjson=new_profiling,
    )
    # Apply rl_rules
    cli.call(f"doc update {BaseHelper.TEST_CONFIG_NAME} ratelimits /dev/stdin", inputjson=rl_rules)
    # Apply new_urlmap
    cli.call(f"doc update {BaseHelper.TEST_CONFIG_NAME} {api_config['url_map']} /dev/stdin", inputjson=new_urlmap)
    cli.publish_and_apply()
