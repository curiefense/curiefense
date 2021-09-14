import pytest
from e2e.core.base_helper import cli, target, BaseHelper
from e2e.helpers.acl_helper import acl
import time


class TagRulesHelper:

    @staticmethod
    def tag_rules_json():
        test_tagrules = {
            "id": "e2e000000000",
            "name": "e2e test tag rules",
            "source": "self-managed",
            "mdate": "2020-11-22T00:00:00.000Z",
            "notes": "E2E test tag rules",
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
                            ["path", "/e2e-tagrules-path/", "annotation"],
                            ["query", "e2e=value", "annotation"],
                            ["uri", "/e2e-tagrules-uri", "annotation"],
                            ["ip", BaseHelper.IP6_1, "annotation"],
                            ["ip", BaseHelper.IP4_US, "annotation"],
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
        return test_tagrules


@pytest.fixture(scope="session", params=[True, False], ids=["active", "inactive"])
def active(request):
    return request.param


@pytest.fixture(scope="class")
def tagrules_config(cli, acl, active, api_config):
    cli.revert_and_enable()
    acl.set_acl({"force_deny": "e2e-test", "bypass": "all"})
    # Apply TEST_TAGRULES
    TagRulesHelper.tag_rules_json()["active"] = active
    # 'updating' wafpolicies with a list containing a single entry adds this
    # entry, without removing pre-existing ones.
    cli.call(
        f"doc update {BaseHelper.TEST_CONFIG_NAME} {api_config['tag_rules']} /dev/stdin", inputjson=[TagRulesHelper.tag_rules_json()]
    )
    cli.publish_and_apply()