import pytest
from e2e.core.base_helper import cli, target, BaseHelper
from e2e.helpers.acl_helper import acl
from typing import List, Optional


class WafPoliciesHelper:

    @staticmethod
    def gen_waf_rules():
        waf_rules = []
        map_path = {}

        def add_optional_params(**kwargs):
            if kwargs == {}:
                r: Optional[list] = list()
            else:
                r = []
                for k, v in kwargs.items():
                    r.append(v)
            return r

        def add_waf_rule(
                path, mac=512, mcc=42, mhc=42, mal=1024, mcl=1024, mhl=1024,ignore=True, cookies_names=None,
                cookies_regex=None, args_names=None, args_regex=None, headers_names=None, headers_regex=None
        ):
            rule_id = f"e2e1{len(waf_rules):0>9}"

            if cookies_names is None:
                cookies_names = {}
            if cookies_regex is None:
                cookies_regex = {}

            if args_names is None:
                args_names = {}
            if args_regex is None:
                args_regex = {}

            if headers_names is None:
                headers_names = {}
            if headers_regex is None:
                headers_regex = {}

            map_path[path] = rule_id

            waf_rules.append(
                {
                    "id": rule_id,
                    "name": path,
                    "max_args_count": mac,
                    "max_cookies_count": mcc,
                    "max_headers_count": mhc,
                    "max_arg_length": mal,
                    "max_cookie_length": mcl,
                    "max_header_length": mhl,
                    "ignore_alphanum": ignore,
                    "args": {
                        "names": add_optional_params(**args_names),
                        "regex": add_optional_params(**args_regex)

                    },
                    "cookies": {
                        "names": add_optional_params(**cookies_names),
                        "regex": add_optional_params(**cookies_regex)
                    },

                    "headers": {
                        "names": add_optional_params(**headers_names),
                        "regex": add_optional_params(**headers_regex)
                    }

                }
            )

        add_waf_rule(
            "path",
            cookies_names={
                "1": {
                    "key": "cookie_name",
                    "reg": "52rgwfyrytg",
                    "restrict": True,
                    "exclusions": {"100123": 1}
                },
                "2": {
                    "key": "cookie_name123",
                    "reg": "52rgwfyry43545345tg",
                    "restrict": True,
                    "exclusions": {"100200": 1}
                }
            }

        )
        add_waf_rule(
            "test_section_overlong_length_with_short_value",
            mal=50,
            mhl=50,
            mcl=50
        )
        add_waf_rule(
            "test_section_short_length_with_short_value",
            mal=40,
            mhl=40,
            mcl=40
        )
        add_waf_rule(
            "test_low_args_counter",
            mac=9
        )
        add_waf_rule(
            "test_low_headers_cookies_counter",
            mcc=20,
            mhc=20,
        )
        add_waf_rule(
            "abcd",
            ignore=False,
            cookies_names={
                "1": {
                    "key": "foobar",
                    "reg": "block_cookie",
                    "restrict": True,
                    "mask": False,
                    "exclusions": None,
                }
            }
        )
        add_waf_rule(
            "bcde",
            ignore=False,
            headers_names={
                "1": {
                    "key": "foobar",
                    "reg": "block_header",
                    "restrict": True,
                    "mask": False,
                    "exclusions": None,
                }
            }
        )
        add_waf_rule(
            "cdef",
            ignore=False,
            args_names={
                "1": {
                    "key": "foobar",
                    "reg": "block_arg",
                    "restrict": True,
                    "mask": False,
                    "exclusions": None,
                }
            }
        )
        add_waf_rule(
            "ig_alpha_cook",
            ignore=True,
            cookies_names={
                "1": {
                    "key": "foobar",
                    "reg": "block_cookie",
                    "restrict": True,
                    "mask": False,
                    "exclusions": None,
                }
            }
        )
        add_waf_rule(
            "ig_alph_head",
            ignore=True,
            headers_names={
                "1": {
                    "key": "foobar",
                    "reg": "block_header",
                    "restrict": True,
                    "mask": False,
                    "exclusions": None,
                }
            }
        )
        add_waf_rule(
            "ig_alph_arg",
            ignore=True,
            args_names={
                "1": {
                    "key": "foobar",
                    "reg": "block_arg",
                    "restrict": True,
                    "mask": False,
                    "exclusions": None,
                }
            }
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
                               "limit_ids": [],
                           }
                       ]
                       + [
                           {
                               "name": k,
                               "match": f"/{k}",
                               "acl_profile": "__default__",
                               "acl_active": True,
                               "waf_profile": v,
                               "waf_active": True,
                               "limit_ids": [],
                           }
                           for k, v in map_path.items()
                       ],
            }
        ]
        return (waf_rules, rl_urlmap)

@pytest.fixture(
    scope="session", params=[True, False], ids=["ignore_alphanum", "no_ignore_alphanum"]
)
def ignore_alphanum(request):
    return request.param

@pytest.fixture(scope="function", params=["name", "regex"])
def name_regex(request):
    return request.param


@pytest.fixture(scope="function", params=["restrict", "norestrict"])
def restrict(request):
    return request.param


@pytest.fixture(scope="class")
def waf_test_config(cli, target, api_config):
    cli.revert_and_enable()
    # Add new Waf policies
    waf_rules = cli.call(f"doc get {BaseHelper.TEST_CONFIG_NAME} wafpolicies")
    (new_rules, new_urlmap) = WafPoliciesHelper.gen_waf_rules()
    waf_rules.extend(new_rules)
    # Apply waf_policies
    cli.call(f"doc update {BaseHelper.TEST_CONFIG_NAME} wafpolicies /dev/stdin", inputjson=waf_rules)
    # Apply new_urlmap
    cli.call(f"doc update {BaseHelper.TEST_CONFIG_NAME} {api_config['url_map']} /dev/stdin", inputjson=new_urlmap)
    cli.publish_and_apply()
