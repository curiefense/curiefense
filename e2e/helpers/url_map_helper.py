import pytest
from e2e.core.base_helper import cli, target, BaseHelper
from e2e.helpers.acl_helper import acl


class UrlMapHelper:
    @staticmethod
    def acl_passthrough_json():
        acl_passthrough = {
            "id": "e2e00ac10000",
            "name": "e2e-denyall-acl",
            "allow": [],
            "allow_bot": [],
            "deny_bot": [],
            "passthrough": ["all"],
            "force_deny": [],
            "deny": [],
        }
        return acl_passthrough

    @staticmethod
    def waf_short_headers_json():
        waf_short_headers = {
            "id": "e2e000000002",
            "name": "e2e waf short headers",
            "ignore_alphanum": True,
            "max_header_length": 50,
            "max_cookie_length": 1024,
            "max_arg_length": 1024,
            "max_headers_count": 42,
            "max_cookies_count": 42,
            "max_args_count": 512,
            "args": {"names": [], "regex": []},
            "headers": {"names": [], "regex": []},
            "cookies": {"names": [], "regex": []},
        }
        return waf_short_headers

    @staticmethod
    def url_map_json():
        url_map = [
            {
                "id": "e2e000000001",
                "name": "e2e URL map",
                "match": ".*",
                "map": [
                    {
                        "name": "acl",
                        "match": "/acl/",
                        "acl_profile": "__default__",
                        "acl_active": True,
                        "contentfilterprofiles": "__default__",
                        "waf_active": False,
                        "limit_ids": [],
                        "isnew": True,
                    },
                    {
                        "name": "acl-bypassall",
                        "match": "/acl-bypassall/",
                        "acl_profile": "e2e00ac10000",
                        "acl_active": True,
                        "contentfilterprofiles": "__default__",
                        "waf_active": True,
                        "limit_ids": [],
                        "isnew": True,
                    },
                    {
                        "name": "acl-waf",
                        "match": "/acl-waf/",
                        "acl_profile": "__default__",
                        "acl_active": True,
                        "contentfilterprofiles": "__default__",
                        "waf_active": True,
                        "limit_ids": [],
                        "isnew": True,
                    },
                    {
                        "name": "waf",
                        "match": "/waf/",
                        "acl_profile": "__default__",
                        "acl_active": False,
                        "contentfilterprofiles": "__default__",
                        "waf_active": True,
                        "limit_ids": [],
                        "isnew": True,
                    },
                    {
                        "name": "waf-short-headers",
                        "match": "/waf-short-headers/",
                        "acl_profile": "__default__",
                        "acl_active": False,
                        "contentfilterprofiles": "e2e000000002",
                        "waf_active": True,
                        "limit_ids": [],
                        "isnew": True,
                    },
                    {
                        "name": "nofilter",
                        "match": "/nofilter/",
                        "acl_profile": "__default__",
                        "acl_active": False,
                        "contentfilterprofiles": "__default__",
                        "waf_active": False,
                        "limit_ids": [],
                    },
                ],
            }
        ]
        return url_map


@pytest.fixture(scope="class")
def urlmap_config(cli, acl, api_config):
    cli.revert_and_enable()
    # Add ACL entry
    default_acl = cli.empty_acl()
    default_acl[0]["force_deny"].append("all")
    default_acl.append(UrlMapHelper.acl_passthrough_json())
    cli.call(
        f"doc update {BaseHelper.TEST_CONFIG_NAME} {api_config['acl_setting']} /dev/stdin", inputjson=default_acl
    )
    # Add waf profile entry
    wafpolicy = cli.call(f"doc get {BaseHelper.TEST_CONFIG_NAME} wafpolicies")
    wafpolicy.append(UrlMapHelper.waf_short_headers_json())
    cli.call(
        f"doc update {BaseHelper.TEST_CONFIG_NAME} {api_config['wafpolicies']} /dev/stdin", inputjson=wafpolicy
    )
    # Add urlmap entry URLMAP
    cli.call(f"doc update {BaseHelper.TEST_CONFIG_NAME} {api_config['url_map']} /dev/stdin", inputjson=UrlMapHelper.url_map_json())
    cli.publish_and_apply()
