import pytest
from e2e.core.base_helpers import cli, target, BaseHelper
import time


class RateLimitHelper:

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

    @staticmethod
    def gen_rl_rules(authority):
        rl_rules = []
        map_path = {}

        def add_rl_rule(
                path, action_ext=None, subaction_ext=None, param_ext=None, **kwargs
        ):
            rule_id = f"e2e1{len(rl_rules):0>9}"
            if subaction_ext is None:
                subaction_ext = {}
            if action_ext is None:
                action_ext = {}
            if param_ext is None:
                param_ext = {}
            map_path[path] = rule_id
            rl_rules.append(
                {
                    "id": rule_id,
                    "name": "Rate Limit Rule 3/10 " + path,
                    "description": "3 requests per 10 seconds",
                    "ttl": "10",
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
                    "include": {
                        "cookies": kwargs.get("incl_cookies", {}),
                        "headers": kwargs.get("incl_headers", {}),
                        "args": kwargs.get("incl_args", {}),
                        "attrs": kwargs.get("incl_attrs", {}),
                    },
                    "exclude": {
                        "cookies": kwargs.get("excl_cookies", {}),
                        "headers": kwargs.get("excl_headers", {}),
                        "args": kwargs.get("excl_args", {}),
                        "attrs": kwargs.get("excl_attrs", {}),
                    },
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
            "scope-params",
            incl_args={"include": "true"},
            excl_args={"exclude": "true"}
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
            param_ext={"ttl": "10"},
            excl_attrs={"tags": "allowlist"},
            incl_attrs={"tags": "blocklist"},
        )
        add_rl_rule(
            "action-ban-challenge",
            action="ban",
            subaction="challenge",
            param_ext={"ttl": "10"},
            subaction_params={"action": {"type": "default", "params": {}}},
        )
        add_rl_rule(
            "action-ban-tagonly",
            action="ban",
            subaction="monitor",
            param_ext={"ttl": "10"},
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
            param_ext={"ttl": "10"},
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
            param_ext={"ttl": "10"},
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
                               "waf_profile": "__default__",
                               "waf_active": True,
                               "limit_ids": ["e2e100000000"],
                           }
                       ]
                       + [
                           {
                               "name": k,
                               "match": f"/{k}/",
                               "acl_profile": "__default__",
                               "acl_active": True,
                               "waf_profile": "__default__",
                               "waf_active": True,
                               "limit_ids": [v],
                           }
                           for k, v in map_path.items()
                       ],
            }
        ]
        return rl_rules, rl_urlmap


@pytest.fixture(scope="class")
def ratelimit_config(cli, target):
    cli.revert_and_enable()
    # Add new RL rules
    rl_rules = cli.call(f"doc get {BaseHelper.TEST_CONFIG_NAME} ratelimits")
    (new_rules, new_urlmap) = RateLimitHelper.gen_rl_rules(target.authority())
    rl_rules.extend(new_rules)
    cli.call(f"doc update {BaseHelper.TEST_CONFIG_NAME} ratelimits /dev/stdin", inputjson=rl_rules)
    # Apply NEW_URLMAP
    cli.call(f"doc update {BaseHelper.TEST_CONFIG_NAME} urlmaps /dev/stdin", inputjson=new_urlmap)
    cli.publish_and_apply()
