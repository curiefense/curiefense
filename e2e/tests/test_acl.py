import pytest
from e2e.core.base_helper import target, cli, BaseHelper
from e2e.helpers.acl_helper import acl


@pytest.mark.usefixtures('api_setup')
@pytest.mark.acl_tests
@pytest.mark.all_modules
class TestACL:
    def test_enforce_deny_all(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl(self.data["ACL1"]["set_acl"])
        cli.publish_and_apply()
        # acl.reset_and_set_acl(self.data["ACL1"]["set_acl"])
        assert not target.is_reachable(self.data["ACL1"]["is_reachable"])

    def test_bypass_all(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl(self.data["ACL2"]["set_acl"])
        cli.publish_and_apply()
        # acl.reset_and_set_acl({"deny": "all", "bypass": "all"})
        assert target.is_reachable(self.data["ACL2"]["is_reachable"])

    def test_allow_bot_all(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl(self.data["ACL3"]["set_acl"])
        cli.publish_and_apply()
        # acl.reset_and_set_acl({"allow_bot": "all"})
        assert not target.is_reachable(
            self.data["ACL3"]["path"],
            headers={self.data["ACL3"]["header_name"]: self.data["ACL3"]["header_value"] * 1500}
        )
        assert target.is_reachable()

    def test_deny_bot_all(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl(self.data["ACL4"]["set_acl"])
        cli.publish_and_apply()
        # acl.reset_and_set_acl({"deny_bot": "all"})
        res = target.query(path=self.data["ACL4"]["path"])
        assert res.status_code == 247
        assert self.data["ACL4"]["response_text"] in res.text

    def test_allow_all(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl(self.data["ACL5"]["set_acl"])
        cli.publish_and_apply()
        # acl.reset_and_set_acl({"allow": "all", "deny": "all"})
        assert not target.is_reachable(
            self.data["ACL5"]["path"],
            headers={self.data["ACL5"]["header_name"]: self.data["ACL5"]["header_value"] * 1500}
        )
        assert target.is_reachable()

    def test_deny_all(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl(self.data["ACL6"]["set_acl"])
        cli.publish_and_apply()
        # acl.reset_and_set_acl({"deny": "all"})
        assert not target.is_reachable(self.data["ACL6"]["is_reachable"])

    def test_ip_asn(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl(self.data["ACL7"]["set_acl"])
        cli.publish_and_apply()
        # acl.reset_and_set_acl({"deny": "asn:1239"})
        assert not target.is_reachable(self.data["ACL7"]["is_reachable"], srcip=BaseHelper.IP4_US)
        assert target.is_reachable("/")

    def test_ipv4(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl(self.data["ACL8"]["set_acl"])
        cli.publish_and_apply()
        # acl.reset_and_set_acl({"deny": "ip:199-0-0-1"})
        assert not target.is_reachable(self.data["ACL8"]["is_reachable"], srcip=BaseHelper.IP4_US)
        assert target.is_reachable("/")

    def test_geo(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl(self.data["ACL9"]["set_acl"])
        cli.publish_and_apply()
        # acl.reset_and_set_acl({"deny": "geo:united-states"})
        assert not target.is_reachable(self.data["ACL9"]["is_reachable"], srcip=BaseHelper.IP4_US)
        assert target.is_reachable(self.data["ACL9"]["is_reachable"], srcip=BaseHelper.IP4_JP)
        assert target.is_reachable("/")

    def test_ipv6(self, acl, target, cli):
        acl.reset_acl_to_default_values()
        acl.set_acl(self.data["ACL10"]["set_acl"])
        cli.publish_and_apply()
        # acl.reset_and_set_acl({"deny": "ip:0000:0000:0000:0000:0000:0000:0000:0001"})
        assert not target.is_reachable(self.data["ACL10"]["is_reachable"], srcip=BaseHelper.IP6_1)
        assert target.is_reachable("/")

