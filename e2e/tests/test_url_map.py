import pytest
from e2e.core.base_helper import cli, target
from e2e.helpers.url_map_helper import urlmap_config
from e2e.helpers.acl_helper import acl


@pytest.mark.usefixtures('api_setup', 'urlmap_config')
@pytest.mark.all_modules
@pytest.mark.url_map_tests
class TestUrlMap:
    def test_nofilter(self, target):
        assert target.is_reachable("/nofilter/")
        assert target.is_reachable(
            "/nofilter/", headers={"Long-header": "Overlong_header" * 100}
        )

    def test_waffilter(self, target):
        assert target.is_reachable("/waf/")
        assert not target.is_reachable(
            "/waf/", headers={"Long-header": "Overlong_header" * 100}
        )

    def test_aclfilter(self, target):
        assert not target.is_reachable("/acl/")
        assert not target.is_reachable(
            "/acl/", headers={"Long-header": "Overlong_header" * 100}
        )

    def test_nondefault_aclfilter_passthrough(self, target):
        assert target.is_reachable("/acl-passthrough/")
        assert target.is_reachable(
            "/acl-passthrough/", headers={"Long-header": "Overlong_header" * 100}
        )

    def test_aclwaffilter(self, target):
        assert not target.is_reachable("/acl-waf/")
        assert not target.is_reachable(
            "/acl/", headers={"Long-header": "Overlong_header" * 100}
        )

    def test_nondefault_wafpolicy_short_headers(self, target):
        assert target.is_reachable(
            "/waf-short-headers/", headers={"Short-header": "0123456789" * 5}
        )
        assert not target.is_reachable(
            "/waf-short-headers/", headers={"Long-header": "0123456789" * 5 + "A"}
        )
