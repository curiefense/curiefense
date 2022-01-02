import pytest
from e2e.core.base_helper import cli, target, section, BaseHelper, log_fixture
from e2e.helpers.tag_rule_default_helper import  tagrules_config, active, TagRulesDefaultHelper
from e2e.helpers.acl_helper import acl


@pytest.mark.usefixtures('api_setup', 'tagrules_config', 'active')
# @pytest.mark.gen_tag_rule
@pytest.mark.all_modules

class TestTagRuleDefault:

    #Cookie section | default action
    def test_cookie_with_default_action(self,target):
        params={"cookies":{"e2e": "test"}}
        TagRulesDefaultHelper.tag_rules_action_503_with_params(target,"/e2e-tagrule-cookie",params)

    #Header section | default action
    def test_header_with_default_action(self,target):
        params={"headers":{"e2e":"test"}}
        TagRulesDefaultHelper.tag_rules_action_503_with_params(target,"/e2e-tagrules-header",params)

    #Method section | default action
    def test_method_with_default_action(self,target):
        params={"method":"POST","headers":{"Content-Length":"0"}}
        TagRulesDefaultHelper.tag_rules_action_503_with_params(target,"/e2e-tagrules-post",params)

    #IPV4 section | default action
    def test_ipv4_with_default_action(self,target):
        params= {"srcip":BaseHelper.IP4_US}
        TagRulesDefaultHelper.tag_rules_action_503_with_params(target,"/e2e-ip",params)

    #IPv6 section | default action
    def test_ipv6_with_default_action(self,target):
        params= {"srcip":BaseHelper.IP6_1}
        TagRulesDefaultHelper.tag_rules_action_503_with_params(target,"/e2e-ip",params)

    #Path section | default action
    def test_path_with_default_action(self,target):
        TagRulesDefaultHelper.tag_rules_action_503(target,"/e2e-tagrules-path/")

    #URI section | default action
    def test_uri_with_default_action(self,target):
        TagRulesDefaultHelper.tag_rules_action_503(target,"/e2e-tagrules-uri")

    #Query section | default action
    def test_query_with_default_action(self,target):
        query= "e2e=value"
        TagRulesDefaultHelper.tag_rules_action_503(target,f"/e2e-query?{query}")

    #Country section | default action
    def test_country_with_default_action(self,target):
        TagRulesDefaultHelper.tag_rules_action_503(target,"/e2e-country")

    #ASN section | default action
    def test_asn_with_default_sction(self,target):
        TagRulesDefaultHelper.tag_rules_action_503(target,"/e2e-asn")