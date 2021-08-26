import pytest
from e2e.core.base_helper import target, cli, BaseHelper
from e2e.helpers.acl_helper import acl


@pytest.mark.usefixtures('api_setup')
@pytest.mark.acl_tests
@pytest.mark.all_modules
class TestACL:

    def test_enforce_deny_all1(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl({"force_deny": "all"})
        cli.publish_and_apply()
        assert not target.is_reachable("/deny-all")

    def test_bypass_all(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl({"deny": "all", "bypass": "all"})
        cli.publish_and_apply()
        assert target.is_reachable("/deny-bypass-all")

    def test_allow_bot_all(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl({"allow_bot": "all"})
        cli.publish_and_apply()
        assert not target.is_reachable(
            "/allow_bot-all",
            headers={"Long-Header": "not_alphanum" * 1500}
        )
        assert target.is_reachable()

    def test_deny_bot_all(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl({"deny_bot": "all"})
        cli.publish_and_apply()
        res = target.query(path="/deny_bot-all")
        assert res.status_code == 247
        assert ";;window.rbzns={bereshit:" in res.text

    def test_allow_all(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl({"allow": "all", "deny": "all"})
        cli.publish_and_apply()
        assert not target.is_reachable(
            "/allow-deny-all",
            headers={"Long-Header": "not_alphanum" * 1500}
        )
        assert target.is_reachable()

    def test_deny_all(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl({"deny": "all"})
        cli.publish_and_apply()
        assert not target.is_reachable("/deny-all")

    def test_ip_asn(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl({"deny": "asn:1239"})
        cli.publish_and_apply()
        assert not target.is_reachable("/acl-asn", srcip=BaseHelper.IP4_US)
        assert target.is_reachable("/")

    def test_ipv4(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl({"deny": "ip:199-0-0-1"})
        cli.publish_and_apply()
        assert not target.is_reachable("/acl-ipv4", srcip=BaseHelper.IP4_US)
        assert target.is_reachable("/")

    def test_geo(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl({"deny": "geo:united-states"})
        cli.publish_and_apply()
        assert not target.is_reachable("/acl-geo", srcip=BaseHelper.IP4_US)
        assert target.is_reachable("/acl-geo", srcip=BaseHelper.IP4_JP)
        assert target.is_reachable("/")

    def test_ipv6(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl({"deny": "ip:0000:0000:0000:0000:0000:0000:0000:0001"})
        cli.publish_and_apply()
        assert not target.is_reachable("/acl-ipv6", srcip=BaseHelper.IP6_1)
        assert target.is_reachable("/")
