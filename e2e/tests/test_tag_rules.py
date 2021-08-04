import pytest
from e2e.core.base_helpers import cli, target, section, BaseHelper
from e2e.helpers.tag_rules_helper import tagrules_config, active, TagRulesHelper
from e2e.helpers.acl_helper import acl


@pytest.mark.usefixtures('api_setup', 'tagrules_config', 'active')
@pytest.mark.tag_rules_tests
@pytest.mark.all_modules
class TestTagRules:
    def test_cookies(self, target):
        assert (
            target.is_reachable("/e2e-tagrules-cookies", cookies={"e2e": "value"})
            is not active
        )
        assert (
            target.is_reachable("/e2e-tagrules-cookies", cookies={"e2e": "allowed"})
            is True
        )

    def test_headers(self, target):
        assert (
            target.is_reachable("/e2e-tagrules-headers", headers={"e2e": "value"})
            is not active
        )
        assert (
            target.is_reachable("/e2e-tagrules-headers", headers={"e2e": "allowed"})
            is True
        )

    def test_method(self, target):
        assert target.is_reachable("/e2e-tagrules-method-GET", method="GET") is True
        assert (
            target.is_reachable("/e2e-tagrules-method-POST", method="POST")
            is not active
        )
        assert (
            target.is_reachable("/e2e-tagrules-method-PUT", method="PUT") is not active
        )

    def test_path(self, target):
        assert target.is_reachable("/e2e-tagrules-path/") is not active
        assert target.is_reachable("/e2e-tagrules-valid-path/") is True

    def test_query(self, target):
        assert (
            target.is_reachable("/e2e-tagrules-query", params={"e2e": "value"})
            is not active
        )
        assert (
            target.is_reachable("/e2e-tagrules-query", params={"e2e": "allowed"})
            is True
        )

    def test_uri(self, target):
        assert target.is_reachable("/e2e-tagrules-uri") is not active
        assert target.is_reachable("/e2e-tagrules-allowed-uri") is True

    def test_ipv4(self, target):
        assert target.is_reachable("/tag-ipv4-1", srcip=BaseHelper.IP4_US) is not active
        assert target.is_reachable("/tag-ipv4-2", srcip=BaseHelper.IP4_ORANGE) is True

    def test_ipv6(self, target):
        assert target.is_reachable("/tag-ipv6-1", srcip=BaseHelper.IP6_1) is not active
        assert target.is_reachable("/tag-ipv6-2", srcip=BaseHelper.IP6_2) is True

    def test_country(self, target):
        # JP address (Softbank)
        assert target.is_reachable("/tag-country", srcip=BaseHelper.IP4_JP) is not active

    def test_asn(self, target):
        # ASN 13335
        assert target.is_reachable("/tag-asn", srcip=BaseHelper.IP4_CLOUDFLARE) is not active

    def test_and(self, target):
        assert (
            target.is_reachable("/e2e-and/", cookies={"e2e-and": "value"}) is not active
        )
        assert (
            target.is_reachable("/not-e2e-and/", cookies={"e2e-and": "value"}) is True
        )
        assert (
            target.is_reachable("/e2e-and/", cookies={"not-e2e-and": "value"}) is True
        )