import pytest
from e2e.core.base_helper import cli, target, BaseHelper, TargetHelper,LogHelper,log_fixture
from e2e.helpers.acl_helper import acl, ACLHelper

class TagRulesDefaultHelper:

    @staticmethod
    def tag_rules_action_503_with_params(target,path, params):
        response = target.query(f"{path}",**params)
        assert response.status_code == 503


    @staticmethod
    def tag_rules_action_503(target,path,**kwargs):
        response= target.query(f"{path}")
        assert response.status_code == 503


    @staticmethod
    def gen_tag_rule():
        default_tagrules = []
        def add_tag_rule(name, action_ext=None, subaction_ext=None, param_ext=None, **kwargs):
            tagruleid = BaseHelper.generate_random_mixed_string(12)
            relation = ['AND', 'OR']
            section_relation = ['AND', 'OR']
            if subaction_ext is None:
                subaction_ext = {}
            if action_ext is None:
                action_ext = {}
            if param_ext is None:
                param_ext = {}

            default_tagrules.append(
                {
                    "id": tagruleid,
                    "name": name,
                    "notes": "test",
                    "tags": ["test-503"],
                    "rule": {
                        "sections": [
                            {
                                "relation": section_relation[1],
                                "entries": [
                                    ["cookies", kwargs.get("cookies", ["e2e","value"])],
                                    ["headers", kwargs.get("headers", ["e2e", "value"])],
                                    ["method", kwargs.get("method", "(PUT|POST)"), "annotation"],
                                    ["path", kwargs.get("path","/e2e-tagrules-path/"), "annotation"],
                                    ["query", kwargs.get("query","e2e=value"), "annotation"],
                                    ["uri", kwargs.get("uri","/e2e-tagrules-uri"), "annotation"],
                                    ["ip", kwargs.get("ip",BaseHelper.IP6_1),
                                      "annotation"],
                                    ["ip", kwargs.get("ip",BaseHelper.IP4_US), "annotation"],
                                    ["country",kwargs.get("country","jp"), "annotation",],
                                    ["asn", kwargs.get("asn","13335"), "annotation"],
                                ]

                            },
                            {
                                "relation": section_relation[0],
                                "entries": [
                                    ["headers", kwargs.get("headers", ["e2e", "value"]), "annotation"],
                                    ["ip", kwargs.get("ip", BaseHelper.IP4_US),"annotation"]
                                ]
                            }
                        ],

                        "relation": relation[1]
                    },
                    "action": {
                        "type": kwargs.get("action", "monitor"),
                        "params": {
                            "action": {
                                "type": kwargs.get("subaction", "default"),
                                "params": kwargs.get("subaction_params", {}),
                                **subaction_ext
                            },
                            **param_ext
                        },
                        **action_ext
                    },
                    "active": True,
                    "mdate": "2020-11-22T00:00:00.000Z",
                    "source": "self-managed"
                }
            )

        add_tag_rule(
            name="Cookie with 503 action",
            action_ext=({"type": "default"}),
            cookies=["e2e","test"]
        )
        add_tag_rule(
            name="Header with 503 action",
            action_ext=({"type":"default"}),
            headers=["e2e","test"]
        )
        add_tag_rule(
            name="Method with 503 action",
            action_ext=({"type":"default"}),
            method="(PUT|POST)"
        )
        add_tag_rule(
            name="Query with 503 action",
            action_ext=({"type":"default"}),
            query="e2e=tests"
        )
        add_tag_rule(
            name= "Country with 503 action",
            action_ext=({"type":"default"}),
            path="/e2e-country"
        )
        add_tag_rule(
            name="ASN with 503 action",
            action_ext=({"type":"default"}),
            path="/e2e-asn"
        )


        return default_tagrules



@pytest.fixture(scope="session", params=[True, False], ids=["active", "inactive"])
def active(request):
    return request.param

@pytest.fixture(scope="class")
def tagrules_config(cli, active, api_config):
    cli.revert_and_enable()
    tag_rules = cli.call(f"doc get {BaseHelper.TEST_CONFIG_NAME} {api_config['tag_rules']}")
    # Apply TEST_TAGRULES
    newrules = TagRulesDefaultHelper.gen_tag_rule()
    TagRulesDefaultHelper.gen_tag_rule()[0]["active"] = active
    tag_rules.extend(newrules)
    cli.call(
        f"doc update {BaseHelper.TEST_CONFIG_NAME} {api_config['tag_rules']} /dev/stdin", inputjson=tag_rules
    )
    cli.publish_and_apply()
