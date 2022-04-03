import pytest
from e2e.core.base_helper import cli, target, section, BaseHelper, log_fixture
from e2e.helpers.tag_rule_monitor_helper import (
    tagrules_config,
    active,
    TagRulesMonitorHelper,
)
from e2e.helpers.acl_helper import acl


@pytest.mark.usefixtures("api_setup", "tagrules_config", "active")
# @pytest.mark.gen_tag_rule
@pytest.mark.all_modules
class TestTagRulesMonitor:
    # Cookies section | monitor action
    def test_cookies_with_tagonly_action(self, target):
        params = {"cookies": {"e2e": "value"}}
<<<<<<< HEAD
        TagRulesMonitorHelper.tag_rules_action_tagonly(
            target, "/e2e-tagrules-cookies", params
        )
=======
        TagRulesMonitorHelper.tag_rules_action_tagonly(target, "/e2e-tagrules-cookies", params)
>>>>>>> 5bb5355c4ae398cab2972092fc84856f27ddd848

    # Header section | monitor action
    def test_header_with_tagonly_action(self, target):
        params = {"headers": {"e2e": "value"}}
<<<<<<< HEAD
        TagRulesMonitorHelper.tag_rules_action_tagonly(
            target, "/e2e-tagrules-header", params
        )
=======
        TagRulesMonitorHelper.tag_rules_action_tagonly(target, "/e2e-tagrules-header", params)
>>>>>>> 5bb5355c4ae398cab2972092fc84856f27ddd848

    # Method section | monitor action
    def test_method_with_tagonly_action(self, target):
        params = {"method": "POST"}
<<<<<<< HEAD
        TagRulesMonitorHelper.tag_rules_action_tagonly(
            target, "/e2e-tagrules-method", params
        )
=======
        TagRulesMonitorHelper.tag_rules_action_tagonly(target, "/e2e-tagrules-method", params)
>>>>>>> 5bb5355c4ae398cab2972092fc84856f27ddd848

    # Path section | monitor action
    def test_path_with_tagonly_action(self, target):
        params = {"method": "GET"}
        TagRulesMonitorHelper.tag_rules_action_tagonly(target, "/notdefault/", params)

    # Query section | monitor action
    def test_query_with_tagonly_action(self, target):
        query = {"query": "e2e=test"}
        params = {"method": "GET"}
<<<<<<< HEAD
        TagRulesMonitorHelper.tag_rules_action_tagonly(
            target, f"/e2e-tagrule-query?{query}", params
        )
=======
        TagRulesMonitorHelper.tag_rules_action_tagonly(target, f"/e2e-tagrule-query?{query}", params)
>>>>>>> 5bb5355c4ae398cab2972092fc84856f27ddd848

    # URI section | monitor action
    def test_uri_with_tagonly_action(self, target):
        params = {"method": "GET"}
<<<<<<< HEAD
        TagRulesMonitorHelper.tag_rules_action_tagonly(
            target, "/e2e-nondefault", params
        )
=======
        TagRulesMonitorHelper.tag_rules_action_tagonly(target, "/e2e-nondefault", params)
>>>>>>> 5bb5355c4ae398cab2972092fc84856f27ddd848

    # IPV4 section | monitor action
    def test_ipv4_with_tagonly_action(self, target):
        params = {"srcip": BaseHelper.IP4_US}
<<<<<<< HEAD
        TagRulesMonitorHelper.tag_rules_action_tagonly(
            target, "/e2e-tagrules-ipv4", params
        )
=======
        TagRulesMonitorHelper.tag_rules_action_tagonly(target, "/e2e-tagrules-ipv4", params)
>>>>>>> 5bb5355c4ae398cab2972092fc84856f27ddd848

    # IPV6 section | monitor action
    def test_ipv6_with_tagonly_action(self, target):
        params = {"srcip": BaseHelper.IP6_1}
<<<<<<< HEAD
        TagRulesMonitorHelper.tag_rules_action_tagonly(
            target, "/e2e-tagrules-ipv6", params
        )
=======
        TagRulesMonitorHelper.tag_rules_action_tagonly(target, "/e2e-tagrules-ipv6", params)
>>>>>>> 5bb5355c4ae398cab2972092fc84856f27ddd848

    # Country section | monitor action
    def test_country_with_tagonly_action(self, target):
        params = {"method": "GET"}
<<<<<<< HEAD
        TagRulesMonitorHelper.tag_rules_action_tagonly(
            target, "/e2e-tagrules-country", params
        )
=======
        TagRulesMonitorHelper.tag_rules_action_tagonly(target, "/e2e-tagrules-country", params)
>>>>>>> 5bb5355c4ae398cab2972092fc84856f27ddd848

    # ASN section | monitor action
    def test_asn_with_tagonly_action(self, target):
        params = {"method": "GET"}
<<<<<<< HEAD
        TagRulesMonitorHelper.tag_rules_action_tagonly(
            target, "/e2e-tagrule-asn", params
        )
=======
        TagRulesMonitorHelper.tag_rules_action_tagonly(target, "/e2e-tagrule-asn", params)
>>>>>>> 5bb5355c4ae398cab2972092fc84856f27ddd848

    # Header and IP section | monitor action
    def test_header_and_ip_with_tagonly_action(self, target):
        params = {"headers": {"e2e": "value"}, "srcip": BaseHelper.IP4_US}
<<<<<<< HEAD
        TagRulesMonitorHelper.tag_rules_action_tagonly(
            target, "/e2e-tagrule-header-ip", params
        )
=======
        TagRulesMonitorHelper.tag_rules_action_tagonly(target, "/e2e-tagrule-header-ip", params)
>>>>>>> 5bb5355c4ae398cab2972092fc84856f27ddd848
