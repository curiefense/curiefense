import pytest
import time
from e2e.core.base_helpers import cli, target, section, BaseHelper
from e2e.helpers.rl_helper import ratelimit_config, RateLimitHelper


@pytest.mark.usefixtures('api_setup', 'ratelimit_config')
@pytest.mark.rate_limit_tests
@pytest.mark.all_modules
class TestRateLimit:
    def test_ratelimit_scope_include(self, target, section):
        # rate limit: max 3 requests within 10 seconds
        param = {section: {"include": "true"}}
        for i in range(1, 4):
            assert target.is_reachable(
                f"/scope-{section}/include/{i}", **param
            ), f"Request #{i} for {section} should be allowed"
        assert not target.is_reachable(
            f"/scope-{section}/include/4", **param
        ), f"Request #4 for {section} should be blocked by the rate limit"
        time.sleep(10)
        assert target.is_reachable(
            f"/scope-{section}/include/5", **param
        ), f"Request #5 for {section} should be allowed"

    def test_ratelimit_scope_include_exclude(self, target, section):
        # rate limit: max 3 requests within 10 seconds
        param = {section: {"include": "true", "exclude": "true"}}
        for i in range(1, 5):
            assert target.is_reachable(
                f"/scope-{section}/include-exclude/{i}", **param
            ), f"Request #{i} for {section} should be allowed"

    def test_ratelimit_scope_exclude(self, target, section):
        # rate limit: max 3 requests within 10 seconds
        param = {section: {"exclude": "true"}}
        for i in range(1, 5):
            assert target.is_reachable(
                f"/scope-{section}/exclude/{i}", **param
            ), f"Request #{i} for {section} should be allowed"

    def test_ratelimit_scope_path_include(self, target):
        # rate limit: max 3 requests within 10 seconds
        for i in range(1, 4):
            assert target.is_reachable(
                f"/scope-path/include/{i}"
            ), f"Request #{i} for path should be allowed"
        assert not target.is_reachable(
            "/scope-path/include/4"
        ), "Request #4 for path should be blocked by the rate limit"
        time.sleep(10)
        assert target.is_reachable(
            "/scope-path/include/5"
        ), "Request #5 for path should be allowed"

    def test_ratelimit_scope_path_include_exclude(self, target):
        # rate limit: max 3 requests within 10 seconds
        for i in range(1, 5):
            assert target.is_reachable(
                f"/scope-path/include/exclude/{i}"
            ), f"Request #{i} for path should be allowed"

    def test_ratelimit_scope_uri_include(self, target):
        # rate limit: max 3 requests within 10 seconds
        for i in range(1, 4):
            assert target.is_reachable(
                f"/scope-uri/include/{i}"
            ), f"Request #{i} for uri should be allowed"
        assert not target.is_reachable(
            "/scope-uri/include/4"
        ), "Request #4 for uri should be blocked by the rate limit"
        time.sleep(10)
        assert target.is_reachable(
            "/scope-uri/include/5"
        ), "Request #5 for uri should be allowed"

    def test_ratelimit_scope_uri_include_exclude(self, target):
        # rate limit: max 3 requests within 10 seconds
        for i in range(1, 5):
            assert target.is_reachable(
                f"/scope-uri/include/exclude/{i}"
            ), f"Request #{i} for uri should be allowed"

    # Todo : research IP testing
    def test_ratelimit_scope_ipv4_include(self, target):
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-ipv4-include/included", srcip=BaseHelper.IP4_US
            ), f"Request #{i} for included ipv4 should be allowed"
        assert not target.is_reachable(
            "/scope-ipv4-include/included", srcip=BaseHelper.IP4_US
        ), "Request #4 for included ipv4 should be denied"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-ipv4-include/not-included", srcip=BaseHelper.IP4_JP
            ), f"Request #{i} for non included ipv4 should be allowed"

    # Todo : research IP testing
    def test_ratelimit_scope_ipv4_exclude(self, target):
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-ipv4-exclude/excluded", srcip=BaseHelper.IP4_US
            ), f"Request #{i} for excluded ipv4 should be allowed"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-ipv4-exclude/not-excluded", srcip=BaseHelper.IP4_JP
            ), f"Request #{i} for non excluded ipv4 should be allowed"
        assert not target.is_reachable(
            "/scope-ipv4-exclude/not-excluded", srcip=BaseHelper.IP4_JP
        ), "Request #4 for non excluded ipv4 should be denied"

    # Todo : research IP testing
    def test_ratelimit_scope_country_include(self, target):
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-country-include/included", srcip=BaseHelper.IP4_US
            ), f"Request #{i} for included country should be allowed"
        assert not target.is_reachable(
            "/scope-country-include/included", srcip=BaseHelper.IP4_US
        ), "Request #4 for included country should be denied"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-country-include/not-included", srcip=BaseHelper.IP4_JP
            ), f"Request #{i} for non included country should be allowed"

    # Todo : research IP testing
    def test_ratelimit_scope_country_exclude(self, target):
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-country-exclude/excluded", srcip=BaseHelper.IP4_US
            ), f"Request #{i} for excluded country should be allowed"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-country-exclude/not-excluded", srcip=BaseHelper.IP4_JP
            ), f"Request #{i} for non excluded country should be allowed"
        assert not target.is_reachable(
            "/scope-country-exclude/not-excluded", srcip=BaseHelper.IP4_JP
        ), "Request #4 for non excluded country should be denied"

    # Todo : research IP testing
    def test_ratelimit_scope_company_include(self, target):
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-company-include/included", srcip=BaseHelper.IP4_CLOUDFLARE
            ), f"Request #{i} for included company should be allowed"
        assert not target.is_reachable(
            "/scope-company-include/included", srcip=BaseHelper.IP4_CLOUDFLARE
        ), "Request #4 for included company should be denied"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-company-include/not-included", srcip=BaseHelper.IP4_US
            ), f"Request #{i} for non included company should be allowed"

    # Todo : research IP testing
    def test_ratelimit_scope_company_exclude(self, target):
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-company-exclude/excluded", srcip=BaseHelper.IP4_CLOUDFLARE
            ), f"Request #{i} for excluded company should be allowed"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-company-exclude/not-excluded", srcip=BaseHelper.IP4_US
            ), f"Request #{i} for non excluded company should be allowed"
        assert not target.is_reachable(
            "/scope-company-exclude/not-excluded", srcip=BaseHelper.IP4_US
        ), "Request #4 for non excluded company should be denied"

    # Todo : research IP testing
    def test_ratelimit_scope_provider_include(self, target):
        # "provider" means "asn"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-provider-include/included", srcip=BaseHelper.IP4_US
            ), f"Request #{i} for included provider should be allowed"
        assert not target.is_reachable(
            "/scope-provider-include/included", srcip=BaseHelper.IP4_US
        ), "Request #4 for included provider should be denied"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-provider-include/not-included", srcip=BaseHelper.IP4_JP
            ), f"Request #{i} for non included provider should be allowed"

    # Todo : research IP testing
    def test_ratelimit_scope_provider_exclude(self, target):
        # "provider" means "asn"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-provider-exclude/excluded", srcip=BaseHelper.IP4_US
            ), f"Request #{i} for excluded provider should be allowed"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-provider-exclude/not-excluded", srcip=BaseHelper.IP4_JP
            ), f"Request #{i} for non excluded provider should be allowed"
        assert not target.is_reachable(
            "/scope-provider-exclude/not-excluded", srcip=BaseHelper.IP4_JP
        ), "Request #4 for non excluded provider should be denied"

    def test_ratelimit_scope_method_include(self, target):
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-method-include/included"
            ), f"Request #{i} for included method should be allowed"
        assert not target.is_reachable(
            "/scope-method-include/included"
        ), "Request #4 for included method should be denied"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-method-include/not-included", method="HEAD"
            ), f"Request #{i} for non included method should be allowed"

    def test_ratelimit_scope_method_exclude(self, target):
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-method-exclude/excluded"
            ), f"Request #{i} for excluded method should be allowed"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-method-exclude/not-excluded", method="HEAD"
            ), f"Request #{i} for non excluded method should be allowed"
        assert not target.is_reachable(
            "/scope-method-exclude/not-excluded", method="HEAD"
        ), "Request #4 for non excluded method should be denied"

    def test_ratelimit_scope_query_include(self, target):
        # if "QUERY" is a substring of the query, rate limiting applies
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-query-include/included?QUERY"
            ), f"Request #{i} for included query should be allowed"
        assert not target.is_reachable(
            "/scope-query-include/included?QUERY"
        ), "Request #4 for included query should be denied"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-query-include/not-included?SOMETHINGELSE"
            ), f"Request #{i} for non included query should be allowed"

    def test_ratelimit_scope_query_exclude(self, target):
        # if "QUERY" is a substring of the query, rate limiting does not apply
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-query-exclude/excluded?QUERY"
            ), f"Request #{i} for excluded query should be allowed"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-query-exclude/not-excluded?SOMETHINGELSE"
            ), f"Request #{i} for non excluded query should be allowed"
        assert not target.is_reachable(
            "/scope-query-exclude/not-excluded?SOMETHINGELSE"
        ), "Request #4 for non excluded query should be denied"

    def test_ratelimit_scope_authority_include(self, target):
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-authority-include/included"
            ), f"Request #{i} for included authority should be allowed"
        assert not target.is_reachable(
            "/scope-authority-include/included"
        ), "Request #4 for included authority should be denied"
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-other-authority-include/not-included"
            ), f"Request #{i} for non included authority should be allowed"

    def test_ratelimit_scope_authority_exclude(self, target):
        for i in range(1, 5):
            assert target.is_reachable(
                "/scope-authority-exclude/excluded"
            ), f"Request #{i} for excluded authority should be allowed"
        for i in range(1, 4):
            assert target.is_reachable(
                "/scope-other-authority-exclude/not-excluded"
            ), f"Request #{i} for non excluded authority should be allowed"
        assert not target.is_reachable(
            "/scope-other-authority-exclude/not-excluded"
        ), "Request #4 for non excluded authority should be denied"

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_countby_section(self, target, section):
        param1 = {section: {"countby": "1"}}
        param2 = {section: {"countby": "2"}}
        RateLimitHelper.ratelimit_countby_helper(target, section, param1, param2)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_countby_ipv4(self, target):
        param1 = {"srcip": BaseHelper.IP4_US}
        param2 = {"srcip": BaseHelper.IP4_JP}
        RateLimitHelper.ratelimit_countby_helper(target, "ipv4", param1, param2)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_countby_ipv6(self, target):
        param1 = {"srcip": BaseHelper.IP6_1}
        param2 = {"srcip": BaseHelper.IP6_2}
        RateLimitHelper.ratelimit_countby_helper(target, "ipv6", param1, param2)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_countby_provider(self, target):
        # "provider" means "asn"
        param1 = {"srcip": BaseHelper.IP4_US}
        param2 = {"srcip": BaseHelper.IP4_JP}
        RateLimitHelper.ratelimit_countby_helper(target, "provider", param1, param2)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_countby_uri(self, target):
        param1 = {}
        param2 = {}
        RateLimitHelper.ratelimit_countby_helper(target, "uri", param1, param2, nocount=True)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_countby_path(self, target):
        param1 = {}
        param2 = {}
        RateLimitHelper.ratelimit_countby_helper(target, "path", param1, param2, nocount=True)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_countby_query(self, target):
        param1 = {"suffix": "?QUERY-1"}
        param2 = {"suffix": "?QUERY-2"}
        RateLimitHelper.ratelimit_countby_helper(target, "query", param1, param2)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_countby_method(self, target):
        param1 = {"method": "HEAD"}
        param2 = {"method": "GET"}
        RateLimitHelper.ratelimit_countby_helper(target, "method", param1, param2)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_countby_company(self, target):
        param1 = {"srcip": BaseHelper.IP4_US}
        param2 = {"srcip": BaseHelper.IP4_JP}
        RateLimitHelper.ratelimit_countby_helper(target, "company", param1, param2)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_countby_country(self, target):
        param1 = {"srcip": BaseHelper.IP4_US}
        param2 = {"srcip": BaseHelper.IP4_JP}
        RateLimitHelper.ratelimit_countby_helper(target, "country", param1, param2)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_countby_authority(self, target):
        param1 = {"headers": {"Host": "authority-1"}}
        param2 = {"headers": {"Host": "authority-2"}}
        RateLimitHelper.ratelimit_countby_helper(target, "authority", param1, param2)

    def test_ratelimit_countby2_section(self, target, section):
        param1 = {section: {"countby1": "1"}}
        param2 = {section: {"countby2": "1"}}
        param12 = {section: {"countby1": "1", "countby2": "1"}}
        for i in range(1, 4):
            assert target.is_reachable(
                f"/countby2-{section}/1/{i}", **param1
            ), f"Request #{i} with {section} countby 1 should be allowed"
            assert target.is_reachable(
                f"/countby2-{section}/2/{i}", **param2
            ), f"Request #{i} with {section} countby 2 should be allowed"
            assert target.is_reachable(
                f"/countby2-{section}/2/{i}", **param12
            ), f"Request #{i} with {section} countby 1&2 should be allowed"
        assert target.is_reachable(
            f"/countby2-{section}/2/4", **param1
        ), f"Request #4 with {section} countby 1 should not be blocked"
        assert target.is_reachable(
            f"/countby2-{section}/2/4", **param2
        ), f"Request #4 with {section} countby 2 should not be blocked"
        assert not target.is_reachable(
            f"/countby2-{section}/2/4", **param12
        ), f"Request #4 with {section} countby 1&2 should be blocked"
        time.sleep(10)
        assert target.is_reachable(
            f"/countby2-{section}/2/5", **param1
        ), f"Request #5 with {section} countby 1 should be allowed"
        assert target.is_reachable(
            f"/countby2-{section}/2/5", **param2
        ), f"Request #5 with {section} countby 2 should be allowed"
        assert target.is_reachable(
            f"/countby2-{section}/2/5", **param12
        ), f"Request #5 with {section} countby 1&2 should be allowed"

    def test_ratelimit_countby_2sections(self, target, section):
        # condition: have countby set for 2 sections
        othersection = {"headers": "params", "cookies": "headers", "params": "cookies"}[
            section
        ]
        param1 = {section: {"countby": "1"}}
        param2 = {othersection: {"countby": "1"}}
        param12 = {section: {"countby": "1"}, othersection: {"countby": "1"}}
        for i in range(1, 4):
            assert target.is_reachable(
                f"/countby-{section}-{othersection}/1/{i}", **param1
            ), f"Request #{i} with {section} countby 1 should be allowed"
            assert target.is_reachable(
                f"/countby-{section}-{othersection}/2/{i}", **param2
            ), f"Request #{i} with {section} countby 2 should be allowed"
            assert target.is_reachable(
                f"/countby-{section}-{othersection}/2/{i}", **param12
            ), f"Request #{i} with {section} countby 1&2 should be allowed"
        assert target.is_reachable(
            f"/countby-{section}-{othersection}/2/4", **param1
        ), f"Request #4 with {section} countby 1 should not be blocked"
        assert target.is_reachable(
            f"/countby-{section}-{othersection}/2/4", **param2
        ), f"Request #4 with {section} countby 2 should not be blocked"
        assert not target.is_reachable(
            f"/countby-{section}-{othersection}/2/4", **param12
        ), f"Request #4 with {section} countby 1&2 should be blocked"
        time.sleep(10)
        assert target.is_reachable(
            f"/countby-{section}-{othersection}/2/5", **param1
        ), f"Request #5 with {section} countby 1 should be allowed"
        assert target.is_reachable(
            f"/countby-{section}-{othersection}/2/5", **param2
        ), f"Request #5 with {section} countby 2 should be allowed"
        assert target.is_reachable(
            f"/countby-{section}-{othersection}/2/5", **param12
        ), f"Request #5 with {section} countby 1&2 should be allowed"

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_event_section(self, target, section):
        params = [{section: {"event": f"{i}"}} for i in range(1, 5)]
        RateLimitHelper.ratelimit_event_param_helper(target, section, params)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_event_ipv4(self, target):
        params = [{"srcip": f"199.0.0.{i}"} for i in range(1, 5)]
        RateLimitHelper.ratelimit_event_param_helper(target, "ipv4", params)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_event_ipv6(self, target):
        params = [
            {"srcip": f"0000:0000:0000:0000:0000:0000:0000:000{i}"} for i in range(1, 5)
        ]
        RateLimitHelper.ratelimit_event_param_helper(target, "ipv6", params)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_event_provider(self, target):
        # "provider" means "asn"
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.ratelimit_event_param_helper(target, "provider", params)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_event_uri(self, target):
        # URI is different for each query, nothing more needs changing
        params = [{"suffix": f"{i}"} for i in range(1, 5)]
        RateLimitHelper.ratelimit_event_param_helper(target, "uri", params)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_event_path(self, target):
        # Path is different for each query, nothing more needs changing
        params = [{"suffix": f"{i}"} for i in range(1, 5)]
        RateLimitHelper.ratelimit_event_param_helper(target, "path", params)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_event_query(self, target):
        params = [{"suffix": f"?QUERY-{i}"} for i in range(1, 5)]
        RateLimitHelper.ratelimit_event_param_helper(target, "query", params)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_event_method(self, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.ratelimit_event_param_helper(target, "method", params)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_event_company(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.ratelimit_event_param_helper(
            target,
            "company",
            params,
        )

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_event_country(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.ratelimit_event_param_helper(target, "country", params)

    # Todo : Try to change test - asserion is hidden
    def test_ratelimit_event_authority(self, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(1, 5)]
        RateLimitHelper.ratelimit_event_param_helper(target, "authority", params)