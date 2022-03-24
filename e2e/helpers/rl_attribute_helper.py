import pytest
from e2e.core.base_helper import cli, target, log_fixture, BaseHelper
from e2e.helpers.rl_helper import RateLimitHelper
from typing import List, Optional


class RateLimitCookieHelper:

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
            path="test_503_action_path_attribute_count_by_ip",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "path"}),
            key=[{"attrs": 'ip'}]
        ),
        add_rl_rule(
            path="test_503_action_path_attribute_count_by_asn",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "path"}),
            key=[{"attrs": 'asn'}]
        ),
        add_rl_rule(
            path="test_503_action_path_attribute_count_by_path",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "path"}),
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_503_action_path_attribute_count_by_country",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "path"}),
            key=[{"attrs": 'country'}]
        ),
        add_rl_rule(
            path="test_503_action_path_attribute_count_by_company",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "path"}),
            key=[{"attrs": 'company'}]
        ),
        add_rl_rule(
            path="test_503_action_path_attribute_count_by_method",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "path"}),
            key=[{"attrs": 'method'}]
        ),
        add_rl_rule(
            path="test_503_action_path_attribute_count_by_authority",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "path"}),
            key=[{"attrs": 'authority'}]
        ),
        add_rl_rule(
            path="test_503_action_uri_attribute_count_by_ip",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "uri"}),
            key=[{"attrs": 'ip'}]
        ),
        add_rl_rule(
            path="test_503_action_uri_attribute_count_by_asn",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "uri"}),
            key=[{"attrs": 'asn'}]
        ),
        add_rl_rule(
            path="test_503_action_uri_attribute_count_by_query",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "uri"}),
            key=[{"attrs": 'query'}]
        ),
        add_rl_rule(
            path="test_503_action_uri_attribute_count_by_country",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "uri"}),
            key=[{"attrs": 'country'}]
        ),
        add_rl_rule(
            path="test_503_action_uri_attribute_count_by_company",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "uri"}),
            key=[{"attrs": 'company'}]
        ),
        add_rl_rule(
            path="test_503_action_uri_attribute_count_by_method",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "uri"}),
            key=[{"attrs": 'method'}]
        ),
        add_rl_rule(
            path="test_503_action_uri_attribute_count_by_authority",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "uri"}),
            key=[{"attrs": 'authority'}]
        ),

        add_rl_rule(
            path="test_503_action_query_attribute_count_by_ip",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "query"}),
            key=[{"attrs": 'ip'}]
        ),
        add_rl_rule(
            path="test_503_action_query_attribute_count_by_asn",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "query"}),
            key=[{"attrs": 'asn'}]
        ),

        add_rl_rule(
            path="test_503_action_query_attribute_count_by_country",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "query"}),
            key=[{"attrs": 'country'}]
        ),
        add_rl_rule(
            path="test_503_action_query_attribute_count_by_company",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "query"}),
            key=[{"attrs": 'company'}]
        ),
        add_rl_rule(
            path="test_503_action_query_attribute_count_by_method",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "query"}),
            key=[{"attrs": 'method'}]
        ),
        add_rl_rule(
            path="test_503_action_query_attribute_count_by_authority",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "query"}),
            key=[{"attrs": 'authority'}]
        ),



        add_rl_rule(
            path="test_503_action_ipv4_attribute_count_by_path",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'path'}]
        ),
        add_rl_rule(
            path="test_503_action_ipv4_attribute_count_by_method",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'method'}]
        ),
        add_rl_rule(
            path="test_503_action_ipv4_attribute_count_by_uri",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'uri'}]
        ),
        add_rl_rule(
            path="test_503_action_ipv4_attribute_count_by_query",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'query'}]
        ),
        add_rl_rule(
            path="test_503_action_ipv4_attribute_count_by_country",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'country'}]
        ),
        add_rl_rule(
            path="test_503_action_ipv4_attribute_count_by_company",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'company'}]
        ),
        add_rl_rule(
            path="test_503_action_ipv4_attribute_count_by_asn",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'asn'}]
        ),
        add_rl_rule(
            path="test_503_action_ipv4_attribute_count_by_query",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'query'}]
        ),
        add_rl_rule(
            path="test_503_action_ipv4_attribute_count_by_authority",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'authority'}]
        ),

        add_rl_rule(
            path="test_503_action_ipv6_attribute_count_by_path",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'path'}]
        ),
        add_rl_rule(
            path="test_503_action_ipv6_attribute_count_by_method",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'method'}]
        ),
        add_rl_rule(
            path="test_503_action_ipv6_attribute_count_by_uri",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'uri'}]
        ),
        add_rl_rule(
            path="test_503_action_ipv6_attribute_count_by_query",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'query'}]
        ),
        add_rl_rule(
            path="test_503_action_ipv6_attribute_count_by_company",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'company'}]
        ),
        add_rl_rule(
            path="test_503_action_ipv6_attribute_count_by_asn",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'asn'}]
        ),
        add_rl_rule(
            path="test_503_action_ipv6_attribute_count_by_authority",
            ttl=5,
            limit=3,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'authority'}]
        ),

        add_rl_rule(
            path="test_503_action_asn_attribute_count_by_path",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'path'}]
        ),
        add_rl_rule(
            path="test_503_action_asn_attribute_count_by_uri",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'uri'}]
        ),
        add_rl_rule(
            path="test_503_action_asn_attribute_count_by_query",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'query'}]
        ),
        add_rl_rule(
            path="test_503_action_asn_attribute_count_by_method",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'method'}]
        ),
        add_rl_rule(
            path="test_503_action_asn_attribute_count_by_authority",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'authority'}]
        ),
        add_rl_rule(
            path="test_503_action_company_attribute_count_by_path",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'path'}]
        ),
        add_rl_rule(
            path="test_503_action_company_attribute_count_by_query",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'query'}]
        ),
        add_rl_rule(
            path="test_503_action_company_attribute_count_by_uri",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'uri'}]
        ),
        add_rl_rule(
            path="test_503_action_company_attribute_count_by_method",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'method'}]
        ),
        add_rl_rule(
            path="test_503_action_company_attribute_count_by_authority",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'authority'}]
        ),
        add_rl_rule(
            path="test_503_action_country_attribute_count_by_path",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'path'}]
        ),
        add_rl_rule(
            path="test_503_action_country_attribute_count_by_query",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'query'}]
        ),
        add_rl_rule(
            path="test_503_action_country_attribute_count_by_uri",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'uri'}]
        ),
        add_rl_rule(
            path="test_503_action_country_attribute_count_by_method",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'method'}]
        ),
        add_rl_rule(
            path="test_503_action_country_attribute_count_by_authority",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'authority'}]
        ),

        add_rl_rule(
            path="test_503_action_method_attribute_count_by_ip",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "method"}),
            key=[{"attrs": 'ip'}]
        ),
        add_rl_rule(
            path="test_503_action_method_attribute_count_by_path",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "method"}),
            key=[{"attrs": 'path'}]
        ),
        add_rl_rule(
            path="test_503_action_method_attribute_count_by_uri",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "method"}),
            key=[{"attrs": 'uri'}]
        ),
        add_rl_rule(
            path="test_503_action_method_attribute_count_by_asn",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "method"}),
            key=[{"attrs": 'asn'}]
        ),
        add_rl_rule(
            path="test_503_action_method_attribute_count_by_query",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "method"}),
            key=[{"attrs": 'query'}]
        ),
        add_rl_rule(
            path="test_503_action_method_attribute_count_by_company",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "method"}),
            key=[{"attrs": 'company'}]
        ),
        add_rl_rule(
            path="test_503_action_method_attribute_count_by_country",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "method"}),
            key=[{"attrs": 'country'}]
        ),
        add_rl_rule(
            path="test_503_action_method_attribute_count_by_authority",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "method"}),
            key=[{"attrs": 'authority'}]
        ),
        add_rl_rule(
            path="test_503_action_authority_attribute_count_by_ip",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "authority"}),
            key=[{"attrs": 'ip'}]
        ),
        add_rl_rule(
            path="test_503_action_authority_attribute_count_by_path",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "authority"}),
            key=[{"attrs": 'path'}]
        ),
        add_rl_rule(
            path="test_503_action_authority_attribute_count_by_uri",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "authority"}),
            key=[{"attrs": 'uri'}]
        ),
        add_rl_rule(
            path="test_503_action_authority_attribute_count_by_query",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "authority"}),
            key=[{"attrs": 'query'}]
        ),
        add_rl_rule(
            path="test_503_action_authority_attribute_count_by_asn",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "authority"}),
            key=[{"attrs": 'asn'}]
        ),
        add_rl_rule(
            path="test_503_action_authority_attribute_count_by_country",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "authority"}),
            key=[{"attrs": 'country'}]
        ),
        add_rl_rule(
            path="test_503_action_authority_attribute_count_by_method",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "authority"}),
            key=[{"attrs": 'method'}]
        ),
        add_rl_rule(
            path="test_503_action_authority_attribute_count_by_company",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "authority"}),
            key=[{"attrs": 'company'}]
        ),
        add_rl_rule(
            path="test_challenge_action_ip_attribute_count_by_path",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'path'}],
            action_ext=({"type": "challenge"})
        ),
        add_rl_rule(
            path="test_challenge_action_ip_attribute_count_by_uri",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'uri'}],
            action_ext=({"type": "challenge"})
        ),
        add_rl_rule(
            path="test_challenge_action_ip_attribute_count_by_query",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'query'}],
            action_ext=({"type": "challenge"})
        ),
        add_rl_rule(
            path="test_challenge_action_ip_attribute_count_by_method",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'method'}],
            action_ext=({"type": "challenge"})
        ),
        add_rl_rule(
            path="test_challenge_action_ip_attribute_count_by_authority",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'authority'}],
            action_ext=({"type": "challenge"})
        ),
        add_rl_rule(
            path="test_chl_action_asn_attr_count_by_path",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'path'}],
            action_ext=({"type": "challenge"})
        ),
        add_rl_rule(
            path="test_chl_action_asn_attr_count_by_uri",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'uri'}],
            action_ext=({"type": "challenge"})
        ),
        add_rl_rule(
            path="test_chl_action_asn_attr_count_by_query",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'query'}],
            action_ext=({"type": "challenge"})
        ),
        add_rl_rule(
            path="test_chl_action_asn_attr_count_by_method",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'method'}],
            action_ext=({"type": "challenge"})
        ),
        add_rl_rule(
            path="test_chl_action_asn_attr_count_by_auth",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "asn"}),
            key=[{"attrs": 'authority'}],
            action_ext=({"type": "challenge"})
        ),
        add_rl_rule(
            path="test_tag_action_ip_attr_count_by_path",
            ttl=16,
            limit=1,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'path'}],
            action_ext=({"type": "monitor"})
        ),
        add_rl_rule(
            path="test_tag_action_ip_attr_count_by_method",
            ttl=16,
            limit=1,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'method'}],
            action_ext=({"type": "monitor"})
        ),
        add_rl_rule(
            path="test_tag_action_country_attr_count_by_query",
            ttl=16,
            limit=1,
            pairwith=({"attrs": "country"}),
            key=[{"attrs": 'path'}],
            action_ext=({"type": "monitor"})
        ),
        add_rl_rule(
            path="test_tag_action_country_attr_count_by_authority",
            ttl=16,
            limit=1,
            pairwith=({"attrs": "country"}),
            key=[{"attrs": 'method'}],
            action_ext=({"type": "monitor"})
        ),

        add_rl_rule(
            path="test_response_action_ip_attr_count_by_path",
            ttl=5,
            limit=2,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'path'}],
            action_ext=({"type": "response"}),
            param_ext={"status": "302", "content": "response_body_path_attr_ip"},
        ),
        add_rl_rule(
            path="test_response_action_method_attr_count_by_country",
            ttl=5,
            limit=2,
            pairwith=({"attrs": "method"}),
            key=[{"attrs": 'country'}],
            action_ext=({"type": "response"}),
            param_ext={"status": "503", "content": "response_body_path_attr_method"},
        ),

        add_rl_rule(
            path="test_redirect_action_ip_attr_count_by_path",
            ttl=2,
            limit=2,
            pairwith=({"attrs": "ip"}),
            key=[{"attrs": 'path'}],
            action_ext=({"type": "redirect"}),
            param_ext={"status": "200", "location": "https://yahoo.com"},
        ),
        add_rl_rule(
            path="test_redirect_action_method_attr_count_by_company",
            ttl=2,
            limit=2,
            pairwith=({"attrs": "method"}),
            key=[{"attrs": 'company'}],
            action_ext=({"type": "redirect"}),
            param_ext={"status": "200", "location": "https://yahoo.com"},
        ),
        add_rl_rule(
            path="test_ban_action_by_ip_sub_503_count_path",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "ip"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_ip_sub_503_count_query",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "ip"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "query"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_ip_sub_503_count_uri",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "ip"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "uri"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_ip_sub_503_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "ip"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_ip_sub_503_count_authority",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "ip"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_provider_sub_503_count_uri",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "asn"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "uri"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_provider_sub_503_count_path",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "asn"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_provider_sub_503_count_query",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "asn"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "query"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_provider_sub_503_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "asn"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_provider_sub_503_count_authority",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "asn"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_uri_sub_503_count_ip",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "uri"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "ip"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_uri_sub_503_count_asn",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "uri"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "asn"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_uri_sub_503_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "uri"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_uri_sub_503_count_country",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "uri"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "country"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_uri_sub_503_count_company",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "uri"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "company"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_uri_sub_503_count_authority",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "uri"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_path_sub_503_count_ip",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "path"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "ip"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_path_sub_503_count_asn",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "path"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "asn"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_path_sub_503_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "path"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_path_sub_503_count_country",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "path"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "country"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_path_sub_503_count_company",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "path"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "company"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_path_sub_503_count_authority",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "path"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_query_sub_503_count_ip",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "query"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "ip"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_query_sub_503_count_asn",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "query"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "asn"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_query_sub_503_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "query"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_query_sub_503_count_country",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "query"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "country"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_query_sub_503_count_company",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "query"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "company"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_query_sub_503_count_authority",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "query"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_503_count_ip",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "ip"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_503_count_uri",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "uri"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_503_count_path",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_503_count_query",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "query"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_503_count_asn",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "asn"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_503_count_country",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "country"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_503_count_company",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "company"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_503_count_authority",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_company_sub_503_count_uri",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "company"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "uri"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_company_sub_503_count_path",
            ttl=4,
            limit=2,
            pairwith=({"attrs": "company"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(5)},
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_company_sub_503_count_query",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "company"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "query"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_company_sub_503_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "company"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_company_sub_503_count_authority",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "company"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_country_sub_503_count_uri",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "country"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "uri"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_country_sub_503_count_path",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "country"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_country_sub_503_count_query",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "country"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "query"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_country_sub_503_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "country"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_country_sub_503_count_authority",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "country"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_authority_sub_503_count_ip",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "ip"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_authority_sub_503_count_uri",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "uri"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_authority_sub_503_count_path",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_authority_sub_503_count_query",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "query"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_authority_sub_503_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_authority_sub_503_count_asn",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "asn"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_authority_sub_503_count_company",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "company"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_authority_sub_503_count_country",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "country"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_ip_sub_chl_count_path",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "ip"}),
            action_ext=({"type": "ban"}),
            subaction="challenge",
            param_ext={"duration": str(4)},
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_ip_sub_chl_count_uri",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "ip"}),
            action_ext=({"type": "ban"}),
            subaction="challenge",
            param_ext={"duration": str(4)},
            key=[{"attrs": "uri"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_ip_sub_chl_count_query",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "ip"}),
            action_ext=({"type": "ban"}),
            subaction="challenge",
            param_ext={"duration": str(4)},
            key=[{"attrs": "query"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_ip_sub_chl_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "ip"}),
            action_ext=({"type": "ban"}),
            subaction="challenge",
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_ip_sub_chl_count_authority",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "ip"}),
            action_ext=({"type": "ban"}),
            subaction="challenge",
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_provider_sub_chl_count_uri",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "asn"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "uri"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_provider_sub_chl_count_path",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "asn"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "path"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_provider_sub_chl_count_query",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "asn"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "query"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_provider_sub_chl_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "asn"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_provider_sub_chl_count_authority",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "asn"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_uri_sub_chl_count_ip",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "uri"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "ip"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_uri_sub_chl_count_asn",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "uri"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "asn"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_uri_sub_chl_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "uri"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_uri_sub_chl_count_company",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "uri"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "company"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_uri_sub_chl_count_authority",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "uri"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_path_sub_chl_count_ip",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "path"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "ip"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_path_sub_chl_count_asn",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "path"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "asn"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_path_sub_chl_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "path"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_path_sub_chl_count_company",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "path"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "company"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_path_sub_chl_count_authority",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "path"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_query_sub_chl_count_ip",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "query"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "ip"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_query_sub_chl_count_asn",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "query"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "asn"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_query_sub_chl_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "query"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_query_sub_chl_count_company",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "query"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "company"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_query_sub_chl_count_auth",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "query"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_query_sub_chl_count_auth",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "query"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_chl_count_ip",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "ip"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_chl_count_uri",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "uri"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_chl_count_query",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "query"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_chl_count_path",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "path"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_chl_count_asn",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "asn"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_chl_count_country",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "country"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_chl_count_comp",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "company"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_method_sub_chl_count_auth",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "method"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_company_sub_chl_count_uri",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "company"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "uri"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_company_sub_chl_count_path",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "company"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "path"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_company_sub_chl_count_query",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "company"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "query"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_company_sub_chl_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "company"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_company_sub_chl_count_auth",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "company"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_country_sub_chl_count_uri",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "country"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "uri"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_country_sub_chl_count_query",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "country"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "query"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_country_sub_chl_count_path",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "country"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "path"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_country_sub_chl_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "country"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_country_sub_chl_count_auth",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "country"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "authority"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_auth_sub_chl_count_ip",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "ip"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_auth_sub_chl_count_uri",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "uri"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_auth_sub_chl_count_query",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "query"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_auth_sub_chl_count_path",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "path"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_auth_sub_chl_count_asn",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "asn"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_auth_sub_chl_count_method",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "method"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_auth_sub_chl_count_company",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "company"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_auth_sub_chl_count_country",
            ttl=3,
            limit=2,
            pairwith=({"attrs": "authority"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(4)},
            key=[{"attrs": "country"}],
            subaction="challenge",
        ),
        add_rl_rule(
            path="test_ban_action_by_ip_sub_tag_count_path",
            ttl=16,
            limit=1,
            pairwith=({"attrs": "ip"}),
            action_ext=({"type": "ban"}),
            subaction="monitor",
            param_ext={"duration": str(22)},
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_provider_sub_tag_count_uri",
            ttl=16,
            limit=1,
            pairwith=({"attrs": "asn"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(22)},
            key=[{"attrs": "uri"}],
            subaction="monitor",
        ),

        add_rl_rule(
            path="test_ban_action_by_ip_sub_red_count_path",
            ttl=2,
            limit=2,
            pairwith=({"attrs": "ip"}),
            action_ext=({"type": "ban"}),
            subaction="redirect",
            subaction_params={"status": "200", "location": "https://google.com"},
            param_ext={"duration": str(3)},
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_provider_sub_red_count_uri",
            ttl=2,
            limit=2,
            pairwith=({"attrs": "asn"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(3)},
            key=[{"attrs": "uri"}],
            subaction="redirect",
            subaction_params={"status": "200", "location": "https://google.com"},
        ),

        add_rl_rule(
            path="test_ban_action_by_ip_sub_res_count_path",
            ttl=2,
            limit=2,
            pairwith=({"attrs": "ip"}),
            action_ext=({"type": "ban"}),
            subaction="response",
            subaction_params={"status": "503", "content": "response_attr_ip_by_path"},
            param_ext={"duration": str(3)},
            key=[{"attrs": "path"}]
        ),
        add_rl_rule(
            path="test_ban_action_by_provider_sub_res_count_uri",
            ttl=2,
            limit=2,
            pairwith=({"attrs": "asn"}),
            action_ext=({"type": "ban"}),
            param_ext={"duration": str(3)},
            key=[{"attrs": "uri"}],
            subaction="response",
            subaction_params={"status": "503", "content": "response_attr_ip_by_uri"},
        ),

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
def ratelimit_attribute_config(cli, target, api_config):
    cli.revert_and_enable()
    # Add new RL rules
    rl_rules = cli.call(f"doc get {BaseHelper.TEST_CONFIG_NAME} ratelimits")
    (new_rules, new_urlmap) = RateLimitCookieHelper.gen_rl_rules(target.authority())
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