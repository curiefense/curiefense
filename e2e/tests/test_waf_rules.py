import pytest
from e2e.core.base_helpers import cli, target, section
from e2e.helpers.acl_helper import acl
from e2e.helpers.waf_policies_helper import wafparam_config, ignore_alphanum, restrict
from e2e.helpers.waf_rules_helper import wafrules


@pytest.mark.usefixtures('api_setup')
@pytest.mark.waf_rules_tests
@pytest.mark.all_modules
class TestWafRules:
    def test_wafsig(self, wafparam_config, target, section, wafrules, ignore_alphanum):
        ruleid, rulestr = wafrules
        has_nonalpha = "." in rulestr
        if ignore_alphanum and not has_nonalpha:
            assert target.is_reachable(
                f"/wafsig-{section}", **{section: {"key": rulestr}}
            ), f"Unreachable despite ignore_alphanum=True for rule {ruleid}"
        else:
            assert not target.is_reachable(
                f"/wafsig-{section}", **{section: {"key": rulestr}}
            ), f"Reachable despite matching rule {ruleid}"