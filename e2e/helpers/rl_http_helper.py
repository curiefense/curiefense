import pytest
from e2e.core.base_helper import cli, target, log_fixture, BaseHelper
from e2e.helpers.rl_helper import RateLimitHelper
from typing import List, Optional


class RateLimitHTTPHelper:

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
                path, ttl, limit, action_ext=None, subaction_ext=None, param_ext=None, **kwargs
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
                    "name": path,
                    "description": 'rate limit http test',
                    "timeframe": ttl,
                    "thresholds": [
                        {
                            "limit": limit,
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
            path="test_503_action_event_http_request_count_by_ip",
            ttl=2,
            limit=5,
            key=[{"attrs": 'ip'}]
        ),

        add_rl_rule(
            path="test_503_action_event_http_request_count_by_asn",
            ttl=2,
            limit=5,
            key=[{"attrs": 'asn'}]
        ),
        add_rl_rule(
            path="test_503_action_event_http_request_count_by_path",
            ttl=2,
            limit=5,
            key=[{"attrs": 'path'}]
        ),
        add_rl_rule(
            path="test_503_action_event_http_request_count_by_query",
            ttl=2,
            limit=5,
            key=[{"attrs": 'query'}]
        ),
        add_rl_rule(
            path="test_503_action_event_http_request_count_by_country",
            ttl=2,
            limit=5,
            key=[{"attrs": 'country'}]
        ),
        add_rl_rule(
            path="test_503_action_event_http_request_count_by_company",
            ttl=2,
            limit=5,
            key=[{"attrs": 'company'}]
        ),
        add_rl_rule(
            path="test_503_action_event_http_request_count_by_uri",
            ttl=2,
            limit=5,
            key=[{"attrs": 'uri'}]
        ),
        add_rl_rule(
            path="test_503_action_event_http_request_count_by_method",
            ttl=2,
            limit=5,
            key=[{"attrs": 'method'}]
        ),
        add_rl_rule(
            path="test_503_action_event_http_request_count_by_authority",
            ttl=2,
            limit=5,
            key=[{"attrs": 'authority'}]
        ),
        add_rl_rule(
            path="test_tag_only_http_ip",
            ttl=20,
            limit=1,
            action_ext=({"type": "monitor"}),
            key=[{"attrs": "ip"}]
        ),
        add_rl_rule(
            path="test_tag_only_http_asn",
            ttl=20,
            limit=1,
            action_ext=({"type": "monitor"}),
            key=[{"attrs": "asn"}]
        ),
        add_rl_rule(
            path="test_tag_only_http_uri",
            ttl=20,
            limit=1,
            action_ext=({"type": "monitor"}),
            key=[{"attrs": "uri"}]
        ),
        add_rl_rule(
            path="test_tag_only_http_path",
            ttl=20,
            limit=1,
            action_ext=({"type": "monitor"}),
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_tag_only_http_query",
            ttl=20,
            limit=1,
            action_ext=({"type": "monitor"}),
            key=[{"attrs": "query"}]
        ),
        add_rl_rule(
            path="test_tag_only_http_method",
            ttl=20,
            limit=1,
            action_ext=({"type": "monitor"}),
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_tag_only_http_company",
            ttl=20,
            limit=1,
            action_ext=({"type": "monitor"}),
            key=[{"attrs": "company"}]
        ),
        add_rl_rule(
            path="test_tag_only_http_country",
            ttl=20,
            limit=1,
            action_ext=({"type": "monitor"}),
            key=[{"attrs": "country"}]
        ),
        add_rl_rule(
            path="test_tag_only_http_authority",
            ttl=20,
            limit=1,
            action_ext=({"type": "monitor"}),
            key=[{"attrs": "authority"}]
        ),

        add_rl_rule(
            path="test_response_http_ip",
            ttl=3,
            limit=4,
            action_ext=({"type": "response"}),
            param_ext={"status": "302", "content": "response_body_ip"},
            key=[{"attrs": "ip"}]
        ),

        add_rl_rule(
            path="test_response_http_uri",
            ttl=3,
            limit=4,
            action_ext=({"type": "response"}),
            param_ext={"status": "503", "content": "response_body_uri"},
            key=[{"attrs": "uri"}]
        ),

        add_rl_rule(
            path="test_response_http_query",
            ttl=3,
            limit=4,
            action_ext=({"type": "response"}),
            param_ext={"status": "302", "content": "response_body_query"},
            key=[{"attrs": "query"}]
        ),

        add_rl_rule(
            path="test_response_http_path",
            ttl=3,
            limit=4,
            action_ext=({"type": "response"}),
            param_ext={"status": "302", "content": "response_body_path"},
            key=[{"attrs": "path"}]
        ),

        add_rl_rule(
            path="test_response_http_method",
            ttl=3,
            limit=4,
            action_ext=({"type": "response"}),
            param_ext={"status": "302", "content": "response_body_method"},
            key=[{"attrs": "method"}]
        ),

        add_rl_rule(
            path="test_response_http_asn",
            ttl=3,
            limit=4,
            action_ext=({"type": "response"}),
            param_ext={"status": "302", "content": "response_body_asn"},
            key=[{"attrs": "asn"}]
        ),

        add_rl_rule(
            path="test_response_http_country",
            ttl=3,
            limit=4,
            action_ext=({"type": "response"}),
            param_ext={"status": "302", "content": "response_body_country"},
            key=[{"attrs": "country"}]
        ),

        add_rl_rule(
            path="test_response_http_company",
            ttl=3,
            limit=4,
            action_ext=({"type": "response"}),
            param_ext={"status": "302", "content": "response_body_company"},
            key=[{"attrs": "company"}]
        ),

        add_rl_rule(
            path="test_response_http_authority",
            ttl=3,
            limit=4,
            action_ext=({"type": "response"}),
            param_ext={"status": "302", "content": "response_body_authority"},
            key=[{"attrs": "authority"}]
        ),

        add_rl_rule(
            path="test_challenge_http_ip",
            ttl=4,
            limit=5,
            action_ext=({"type": "challenge"}),
            key=[{"attrs": "ip"}]
        ),
        add_rl_rule(
            path="test_challenge_http_uri",
            ttl=4,
            limit=5,
            action_ext=({"type": "challenge"}),
            key=[{"attrs": "uri"}]
        ),
        add_rl_rule(
            path="test_challenge_http_query",
            ttl=4,
            limit=5,
            action_ext=({"type": "challenge"}),
            key=[{"attrs": "query"}]
        ),
        add_rl_rule(
            path="test_challenge_http_path",
            ttl=4,
            limit=5,
            action_ext=({"type": "challenge"}),
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_challenge_http_method",
            ttl=4,
            limit=5,
            action_ext=({"type": "challenge"}),
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_challenge_http_asn",
            ttl=4,
            limit=5,
            action_ext=({"type": "challenge"}),
            key=[{"attrs": "asn"}]
        ),
        add_rl_rule(
            path="test_challenge_http_company",
            ttl=4,
            limit=5,
            action_ext=({"type": "challenge"}),
            key=[{"attrs": "company"}]
        ),
        add_rl_rule(
            path="test_challenge_http_country",
            ttl=4,
            limit=5,
            action_ext=({"type": "challenge"}),
            key=[{"attrs": "country"}]
        ),
        add_rl_rule(
            path="test_challenge_http_authority",
            ttl=4,
            limit=5,
            action_ext=({"type": "challenge"}),
            key=[{"attrs": "authority"}]
        ),

        add_rl_rule(
            path="test_redirect_http_ip",
            ttl=3,
            limit=3,
            action_ext=({"type": "redirect"}),
            param_ext={"status": "200", "location": "https://yahoo.com"},
            key=[{"attrs": "ip"}]
        ),

        add_rl_rule(
            path="test_redirect_http_uri",
            ttl=3,
            limit=3,
            action_ext=({"type": "redirect"}),
            param_ext={"status": "200", "location": "https://yahoo.com"},
            key=[{"attrs": "uri"}]
        ),

        add_rl_rule(
            path="test_redirect_http_query",
            ttl=3,
            limit=3,
            action_ext=({"type": "redirect"}),
            param_ext={"status": "200", "location": "https://yahoo.com"},
            key=[{"attrs": "query"}]
        ),
        add_rl_rule(
            path="test_redirect_http_path",
            ttl=3,
            limit=3,
            action_ext=({"type": "redirect"}),
            param_ext={"status": "200", "location": "https://yahoo.com"},
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_redirect_http_asn",
            ttl=3,
            limit=3,
            action_ext=({"type": "redirect"}),
            param_ext={"status": "200", "location": "https://yahoo.com"},
            key=[{"attrs": "asn"}]
        ),
        add_rl_rule(
            path="test_redirect_http_method",
            ttl=3,
            limit=3,
            action_ext=({"type": "redirect"}),
            param_ext={"status": "200", "location": "https://yahoo.com"},
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_redirect_http_company",
            ttl=3,
            limit=3,
            action_ext=({"type": "redirect"}),
            param_ext={"status": "200", "location": "https://yahoo.com"},
            key=[{"attrs": "company"}]
        ),
        add_rl_rule(
            path="test_redirect_http_country",
            ttl=3,
            limit=3,
            action_ext=({"type": "redirect"}),
            param_ext={"status": "200", "location": "https://yahoo.com"},
            key=[{"attrs": "country"}]
        ),
        add_rl_rule(
            path="test_redirect_http_authority",
            ttl=3,
            limit=3,
            action_ext=({"type": "redirect"}),
            param_ext={"status": "200", "location": "https://yahoo.com"},
            key=[{"attrs": "authority"}]
        ),

        add_rl_rule(
            path="test_ban_http_subaction_503_count_by_ip",
            ttl=2,
            limit=4,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "ip"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_503_count_by_uri",
            ttl=2,
            limit=4,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "uri"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_503_count_by_query",
            ttl=2,
            limit=4,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "query"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_503_count_by_path",
            ttl=2,
            limit=4,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_503_count_by_asn",
            ttl=2,
            limit=4,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "asn"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_503_count_by_method",
            ttl=2,
            limit=4,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_503_count_by_company",
            ttl=2,
            limit=4,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "company"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_503_count_by_country",
            ttl=2,
            limit=4,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "country"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_503_count_by_authority",
            ttl=2,
            limit=4,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_challenge_count_by_ip",
            ttl=5,
            limit=5,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(6)},
            subaction="challenge",
            key=[{"attrs": "ip"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_challenge_count_by_uri",
            ttl=5,
            limit=5,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(6)},
            subaction="challenge",
            key=[{"attrs": "uri"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_challenge_count_by_query",
            ttl=5,
            limit=5,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(6)},
            subaction="challenge",
            key=[{"attrs": "query"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_challenge_count_by_path",
            ttl=5,
            limit=5,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(6)},
            subaction="challenge",
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_challenge_count_by_method",
            ttl=5,
            limit=5,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(6)},
            subaction="challenge",
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_challenge_count_by_company",
            ttl=5,
            limit=5,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(6)},
            subaction="challenge",
            key=[{"attrs": "company"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_challenge_count_by_country",
            ttl=5,
            limit=5,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(6)},
            subaction="challenge",
            key=[{"attrs": "ip"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_challenge_count_by_asn",
            ttl=5,
            limit=5,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(6)},
            subaction="challenge",
            key=[{"attrs": "asn"}]
        ),
        add_rl_rule(
            path="test_ban_http_subaction_challenge_count_by_authority",
            ttl=5,
            limit=5,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(6)},
            subaction="challenge",
            key=[{"attrs": "ip"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_tag_count_by_ip",
            ttl=20,
            limit=1,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(22)},
            subaction="monitor",
            key=[{"attrs": "ip"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_tag_count_by_uri",
            ttl=20,
            limit=1,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(22)},
            subaction="monitor",
            key=[{"attrs": "uri"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_tag_count_by_query",
            ttl=20,
            limit=1,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(22)},
            subaction="monitor",
            key=[{"attrs": "query"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_tag_count_by_path",
            ttl=20,
            limit=1,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(22)},
            subaction="monitor",
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_tag_count_by_asn",
            ttl=20,
            limit=1,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(22)},
            subaction="monitor",
            key=[{"attrs": "asn"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_tag_count_by_method",
            ttl=20,
            limit=1,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(22)},
            subaction="monitor",
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_tag_count_by_company",
            ttl=20,
            limit=1,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(22)},
            subaction="monitor",
            key=[{"attrs": "company"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_tag_count_by_country",
            ttl=20,
            limit=1,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(22)},
            subaction="monitor",
            key=[{"attrs": "country"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_tag_count_by_authority",
            ttl=20,
            limit=1,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(22)},
            subaction="monitor",
            key=[{"attrs": "authority"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_response_count_by_ip",
            ttl=2,
            limit=3,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            subaction="response",
            subaction_params={"status": "503", "content": "response_body_ip"},
            key=[{"attrs": "ip"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_response_count_by_uri",
            ttl=2,
            limit=3,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            subaction="response",
            subaction_params={"status": "503", "content": "response_body_uri"},
            key=[{"attrs": "uri"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_response_count_by_query",
            ttl=2,
            limit=3,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            subaction="response",
            subaction_params={"status": "503", "content": "response_body_query"},
            key=[{"attrs": "query"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_response_count_by_path",
            ttl=2,
            limit=3,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            subaction="response",
            subaction_params={"status": "503", "content": "response_body_path"},
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_response_count_by_asn",
            ttl=2,
            limit=3,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            subaction="response",
            subaction_params={"status": "503", "content": "response_body_asn"},
            key=[{"attrs": "asn"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_response_count_by_method",
            ttl=2,
            limit=3,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            subaction="response",
            subaction_params={"status": "503", "content": "response_body_method"},
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_response_count_by_ip",
            ttl=2,
            limit=3,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            subaction="response",
            subaction_params={"status": "503", "content": "response_body_ip"},
            key=[{"attrs": "ip"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_response_count_by_company",
            ttl=2,
            limit=3,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            subaction="response",
            subaction_params={"status": "503", "content": "response_body_company"},
            key=[{"attrs": "company"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_response_count_by_country",
            ttl=2,
            limit=3,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            subaction="response",
            subaction_params={"status": "503", "content": "response_body_country"},
            key=[{"attrs": "country"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_response_count_by_authority",
            ttl=2,
            limit=3,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            subaction="response",
            subaction_params={"status": "503", "content": "response_body_authority"},
            key=[{"attrs": "authority"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_redirect_count_by_ip",
            ttl=3,
            limit=7,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(5)},
            subaction="redirect",
            subaction_params={"status": "200", "location": "https://google.com"},
            key=[{"attrs": "ip"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_redirect_count_by_asn",
            ttl=3,
            limit=7,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(5)},
            subaction="redirect",
            subaction_params={"status": "200", "location": "https://google.com"},
            key=[{"attrs": "asn"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_redirect_count_by_country",
            ttl=3,
            limit=7,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(5)},
            subaction="redirect",
            subaction_params={"status": "200", "location": "https://google.com"},
            key=[{"attrs": "country"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_redirect_count_by_path",
            ttl=3,
            limit=7,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(5)},
            subaction="redirect",
            subaction_params={"status": "200", "location": "https://google.com"},
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_redirect_count_by_query",
            ttl=3,
            limit=7,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(5)},
            subaction="redirect",
            subaction_params={"status": "200", "location": "https://google.com"},
            key=[{"attrs": "query"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_redirect_count_by_uri",
            ttl=3,
            limit=7,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(5)},
            subaction="redirect",
            subaction_params={"status": "200", "location": "https://google.com"},
            key=[{"attrs": "uri"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_redirect_count_by_company",
            ttl=3,
            limit=7,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(5)},
            subaction="redirect",
            subaction_params={"status": "200", "location": "https://google.com"},
            key=[{"attrs": "company"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_redirect_count_by_method",
            ttl=3,
            limit=7,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(5)},
            subaction="redirect",
            subaction_params={"status": "200", "location": "https://google.com"},
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_ban_http_sub_redirect_count_by_authority",
            ttl=3,
            limit=7,
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(5)},
            subaction="redirect",
            subaction_params={"status": "200", "location": "https://google.com"},
            key=[{"attrs": "authority"}]
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
                               "limit_ids": ["e2e100000000"],
                           }
                       ]
                       + [
                           {
                               "name": k,
                               "match": f"/{k}",
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
        return rl_rules, rl_urlmap


@pytest.fixture(scope="class")
def ratelimit_http_config(cli, target, api_config):
    cli.revert_and_enable()
    # Add new RL rules
    rl_rules = cli.call(f"doc get {BaseHelper.TEST_CONFIG_NAME} ratelimits")
    (new_rules, new_urlmap) = RateLimitHTTPHelper.gen_rl_rules(target.authority())
    rules_without_include_exclude = []
    # remove include/exclude default values
    for rule in new_rules:
        rule_without_include_exclude = RateLimitHelper.remove_include_exclude_from_ratelimit_json(rule)
        rules_without_include_exclude.append(rule_without_include_exclude)
    rl_rules.extend(rules_without_include_exclude)
    cli.call(f"doc update {BaseHelper.TEST_CONFIG_NAME} ratelimits /dev/stdin", inputjson=rl_rules)
    # Apply NEW_URLMAP
    cli.call(f"doc update {BaseHelper.TEST_CONFIG_NAME} {api_config['url_map']} /dev/stdin", inputjson=new_urlmap)
    cli.publish_and_apply()