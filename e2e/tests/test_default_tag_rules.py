import pytest
from e2e.core.base_helper import cli, target, section, BaseHelper, log_fixture
from e2e.helpers.tag_rule_default_helper import  tagrules_config, active, TagRulesDefaultHelper
from e2e.helpers.acl_helper import acl


@pytest.mark.usefixtures('api_setup', 'tagrules_config', 'active')
@pytest.mark.gen_tag_rule
@pytest.mark.all_modules

class TestTagRuleDefault:

    #Cookie section | default action
    def test_cookie_with_default_action(self,target):
        params={"cookies":{"e2e": "value"}}
        TagRulesDefaultHelper.tag_rules_action_503(target,"/e2e-tagrule-cookie",params)