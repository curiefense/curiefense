import pytest
from e2e.core.base_helper import target, cli, BaseHelper
from e2e.helpers.acl_helper import acl


@pytest.mark.usefixtures('api_setup')
@pytest.mark.acl_tests
@pytest.mark.all_modules
class TestACL:

    def test_enforce_deny_all(self, acl, target):
        acl.reset_and_set_acl({"force_deny": "all"})
        assert not target.is_reachable("/deny-all")

    def test_passthrough_all(self, acl, target):
        acl.reset_and_set_acl({"deny": "all", "passthrough": "all"})
        assert target.is_reachable("/deny-passthrough-all")

    def test_allow_bot_all(self, acl, target):
        acl.reset_and_set_acl({"allow_bot": "all"})
        assert not target.is_reachable(
            "/", headers={"Long-Header": "not_alphanum" * 1000}
        )
        assert target.is_reachable()

    def test_deny_bot_all(self, acl, target):
        acl.reset_and_set_acl({"deny_bot": "all"})
        res = target.query(path="/")
        assert res.status_code == 247
        assert ";;window.rbzns={bereshit:" in res.text

    def test_allow_all(self, acl, target):
        acl.reset_and_set_acl({"allow": "all", "deny": "all"})
        assert not target.is_reachable(
            "/", headers={"Long-Header": "not_alphanum" * 1500}
        )
        assert target.is_reachable()

    def test_deny_all(self, acl, target):
        acl.reset_and_set_acl({"deny": "all"})
        assert not target.is_reachable("/deny-all")

    def test_ip_asn(self, acl, target):
        acl.reset_and_set_acl({"deny": "asn:1239"})
        assert not target.is_reachable("/acl-asn", srcip=BaseHelper.IP4_US)
        assert target.is_reachable("/")

    def test_ipv4(self, acl, target):
        acl.reset_and_set_acl({"deny": "ip:199-0-0-1"})
        assert not target.is_reachable("/acl-ipv4", srcip=BaseHelper.IP4_US)
        assert target.is_reachable("/")

    def test_geo(self, acl, target):
        acl.reset_and_set_acl({"deny": "geo:united-states"})
        assert not target.is_reachable("/acl-geo", srcip=BaseHelper.IP4_US)
        assert target.is_reachable("/acl-geo", srcip=BaseHelper.IP4_JP)
        assert target.is_reachable("/")

    def test_ipv6(self, acl, target):
        acl.reset_and_set_acl({"deny": "ip:0000:0000:0000:0000:0000:0000:0000:0001"})
        assert not target.is_reachable("/acl-ipv6", srcip=BaseHelper.IP6_1)
        assert target.is_reachable("/")