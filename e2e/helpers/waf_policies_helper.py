import pytest
from e2e.core.base_helpers import cli, target, BaseHelper
from e2e.helpers.acl_helper import acl
import time


class WafPoliciesHelper:

    @staticmethod
    def waf_param_constraints():
        waf_param_constraints_json = {
            "names": [
                {
                    "key": "name-norestrict",
                    "reg": "[v]+[a]{1}l?u*e",
                    "restrict": False,
                    "exclusions": {"100140": 1},
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
                    "exclusions": {"100140": 1},
                },
                {
                    "key": "reg[e]x{1}-restrict",
                    "reg": "[v]+[a]{1}l?u*e",
                    "restrict": True,
                    "exclusions": {},
                },
            ],
        }
        return waf_param_constraints_json


@pytest.fixture(
    scope="session", params=[True, False], ids=["ignore_alphanum", "no_ignore_alphanum"]
)
def ignore_alphanum(request):
    return request.param


@pytest.fixture(scope="class")
def wafparam_config(cli, ignore_alphanum):
    cli.revert_and_enable()
    # Apply WAF_PARAM_CONSTRAINTS
    wafpolicy = cli.call(f"doc get {BaseHelper.TEST_CONFIG_NAME} wafpolicies")
    for k in ("args", "headers", "cookies"):
        wafpolicy[0][k] = WafPoliciesHelper.waf_param_constraints()
    wafpolicy[0]["ignore_alphanum"] = ignore_alphanum
    cli.call(
        f"doc update {BaseHelper.TEST_CONFIG_NAME} wafpolicies /dev/stdin", inputjson=wafpolicy
    )

    cli.publish_and_apply()


@pytest.fixture(scope="function", params=["name", "regex"])
def name_regex(request):
    return request.param


@pytest.fixture(scope="function", params=["restrict", "norestrict"])
def restrict(request):
    return request.param

