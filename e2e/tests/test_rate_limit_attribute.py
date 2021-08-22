import pytest
from e2e.core.base_helper import cli, target, section, log_fixture, BaseHelper
from e2e.helpers.rl_helper import RateLimitHelper
from e2e.helpers.rl_attribute_helper import ratelimit_attribute_config


@pytest.mark.usefixtures('api_setup', 'ratelimit_attribute_config')
@pytest.mark.all_modules
@pytest.mark.rate_limit_tests
@pytest.mark.rate_limit_attribute_tests
class TestRateLimitAttribute:

    #  Action: 503  | Event: Attribute-Path | Count: Ip
    def test_503_action_event_path_attribute_count_by_ip(self, target):
        RateLimitHelper.check_503_response_change_path(target, "test_503_action_path_attribute_count_by_ip/", 3, 8)

    #  Action: 503  | Event: Attribute-Path | Count: Asn
    def test_503_action_event_path_attribute_count_by_asn(self, target):
        RateLimitHelper.check_503_response_change_path(target, "test_503_action_path_attribute_count_by_asn/", 3, 8)

    #  Action: 503  | Event: Attribute-Path | Count: Path
    def test_503_action_event_path_attribute_count_by_path(self, target):
        RateLimitHelper.check_503_response_change_path(target, "test_503_action_path_attribute_count_by_path/", 3, 8)

    #  Action: 503  | Event: Attribute-Path | Count: Method
    def test_503_action_event_path_attribute_count_by_method(self, target):
        RateLimitHelper.check_503_response_change_path(target, "test_503_action_path_attribute_count_by_method/", 3, 8)

    #  Action: 503  | Event: Attribute-Path | Count: Company
    def test_503_action_event_path_attribute_count_by_company(self, target):
        RateLimitHelper.check_503_response_change_path(target, "test_503_action_path_attribute_count_by_company/", 3, 8)

    #  Action: 503  | Event: Attribute-Path | Count: Country
    def test_503_action_event_path_attribute_count_by_country(self, target):
        RateLimitHelper.check_503_response_change_path(target, "test_503_action_path_attribute_count_by_country/", 3, 8)

    #  Action: 503  | Event: Attribute-Path | Count: Authority
    def test_503_action_event_path_attribute_count_by_authority(self, target):
        RateLimitHelper.check_503_response_change_path(target, "test_503_action_path_attribute_count_by_authority/",
                                                       3, 8)

    #  Action: 503  | Event: Attribute-Uri | Count: Ip
    def test_503_action_event_uri_attribute_count_by_ip(self, target):
        RateLimitHelper.check_503_response_change_path(target, "test_503_action_uri_attribute_count_by_ip/", 3, 8)

    #  Action: 503  | Event: Attribute-Uri | Count: Asn
    def test_503_action_event_uri_attribute_count_by_asn(self, target):
        RateLimitHelper.check_503_response_change_path(target, "test_503_action_uri_attribute_count_by_asn/", 3, 8)

    #  Action: 503  | Event: Attribute-Uri | Count: Path
    def test_503_action_event_uri_attribute_count_by_path(self, target):
        RateLimitHelper.check_503_response_change_path(target, "test_503_action_uri_attribute_count_by_path/", 3, 8)

    #  Action: 503  | Event: Attribute-Uri | Count: Method
    def test_503_action_event_uri_attribute_count_by_method(self, target):
        RateLimitHelper.check_503_response_change_path(target, "test_503_action_uri_attribute_count_by_method/", 3, 8)

    #  Action: 503  | Event: Attribute-Uri | Count: Company
    def test_503_action_event_uri_attribute_count_by_company(self, target):
        RateLimitHelper.check_503_response_change_path(target, "test_503_action_uri_attribute_count_by_company/", 3, 8)

    #  Action: 503  | Event: Attribute-Uri | Count: Country
    def test_503_action_event_uri_attribute_count_by_country(self, target):
        RateLimitHelper.check_503_response_change_path(target, "test_503_action_uri_attribute_count_by_country/", 3, 8)

    #  Action: 503  | Event: Attribute-Uri | Count: Authority
    def test_503_action_event_uri_attribute_count_by_authority(self, target):
        RateLimitHelper.check_503_response_change_path(target, "test_503_action_uri_attribute_count_by_authority/", 3,
                                                       8)

    #  Action: 503  | Event: Attribute-Query | Count: Ip
    def test_503_action_event_query_attribute_count_by_ip(self, target):
        params = [{"suffix": f"?QUERY-{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_query_attribute_count_by_ip/",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-Query | Count: Asn
    def test_503_action_event_query_attribute_count_by_asn(self, target):
        params = [{"suffix": f"?QUERY-{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_query_attribute_count_by_asn",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-Query | Count: Method
    def test_503_action_event_query_attribute_count_by_method(self, target):
        params = [{"suffix": f"?QUERY-{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_query_attribute_count_by_method",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-Query | Count: Company
    def test_503_action_event_query_attribute_count_by_company(self, target):
        params = [{"suffix": f"?QUERY-{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_query_attribute_count_by_company",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-Query | Count: Country
    def test_503_action_event_query_attribute_count_by_country(self, target):
        params = [{"suffix": f"?QUERY-{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_query_attribute_count_by_country",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-Query | Count: Authority
    def test_503_action_event_query_attribute_count_by_authority(self, target):
        params = [{"suffix": f"?QUERY-{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_query_attribute_count_by_authority",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-ipv4 | Count: Path
    def test_503_action_event_ipv4_attribute_count_by_path(self, target):
        params = [{"srcip": f"199.0.0.{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_ipv4_attribute_count_by_path",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-ipv4 | Count: Uri
    def test_503_action_event_ipv4_attribute_count_by_uri(self, target):
        params = [{"srcip": f"199.0.0.{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_ipv4_attribute_count_by_uri",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-ipv4 | Count: Query
    def test_503_action_event_ipv4_attribute_count_by_query(self, target):
        params = [{"srcip": f"199.0.0.{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_ipv4_attribute_count_by_query",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-ipv4 | Count: Method
    def test_503_action_event_ipv4_attribute_count_by_method(self, target):
        params = [{"srcip": f"199.0.0.{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_ipv4_attribute_count_by_method",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-ipv4 | Count: Asn
    def test_503_action_event_ipv4_attribute_count_by_asn(self, target):
        params = [{"srcip": f"199.0.0.{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_ipv4_attribute_count_by_asn",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-ipv4 | Count: Country
    def test_503_action_event_ipv4_attribute_count_by_country(self, target):
        params = [{"srcip": f"199.0.0.{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_ipv4_attribute_count_by_country",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-ipv4 | Count: Company
    def test_503_action_event_ipv4_attribute_count_by_company(self, target):
        params = [{"srcip": f"199.0.0.{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_ipv4_attribute_count_by_company",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-ipv4 | Count: Authority
    def test_503_action_event_ipv4_attribute_count_by_authority(self, target):
        params = [{"srcip": f"199.0.0.{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_ipv4_attribute_count_by_authority",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-ipv6 | Count: Path
    def test_503_action_event_ipv6_attribute_count_by_path(self, target):
        params = [{"srcip": f"0000:0000:0000:0000:0000:0000:0000:000{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_ipv6_attribute_count_by_path",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-ipv6 | Count: Method
    def test_503_action_event_ipv6_attribute_count_by_method(self, target):
        params = [{"srcip": f"0000:0000:0000:0000:0000:0000:0000:000{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_ipv6_attribute_count_by_method",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-ipv6 | Count: Query
    def test_503_action_event_ipv6_attribute_count_by_query(self, target):
        params = [{"srcip": f"0000:0000:0000:0000:0000:0000:0000:000{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_ipv6_attribute_count_by_query",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-ipv6 | Count: Authority
    def test_503_action_event_ipv6_attribute_count_by_authority(self, target):
        params = [{"srcip": f"0000:0000:0000:0000:0000:0000:0000:000{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_ipv6_attribute_count_by_authority",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-ipv6 | Count: Uri
    def test_503_action_event_ipv6_attribute_count_by_uri(self, target):
        params = [{"srcip": f"0000:0000:0000:0000:0000:0000:0000:000{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_ipv6_attribute_count_by_uri",
                                                                 3, 8, params)

    #  Action: 503  | Event: Attribute-asn | Count: Path
    def test_503_action_event_asn_attribute_count_by_path(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_asn_attribute_count_by_path",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-asn | Count: Uri
    def test_503_action_event_asn_attribute_count_by_uri(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_asn_attribute_count_by_uri",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-asn | Count: Query
    def test_503_action_event_asn_attribute_count_by_query(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_asn_attribute_count_by_query",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-asn | Count: Method
    def test_503_action_event_asn_attribute_count_by_method(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_asn_attribute_count_by_method",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-asn | Count: Authority
    def test_503_action_event_asn_attribute_count_by_authority(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_asn_attribute_count_by_authority",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-company | Count: Path
    def test_503_action_event_company_attribute_count_by_path(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_company_attribute_count_by_path",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-company | Count: Uri
    def test_503_action_event_company_attribute_count_by_uri(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_company_attribute_count_by_uri",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-company | Count: Query
    def test_503_action_event_company_attribute_count_by_query(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_company_attribute_count_by_query",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-company | Count: Method
    def test_503_action_event_company_attribute_count_by_method(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_company_attribute_count_by_method",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-company | Count: Authority
    def test_503_action_event_company_attribute_count_by_authority(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_company_attribute_count_by_authority",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-country | Count: Path
    def test_503_action_event_country_attribute_count_by_path(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_country_attribute_count_by_path",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-country | Count: Uri
    def test_503_action_event_country_attribute_count_by_uri(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_country_attribute_count_by_uri",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-country | Count: Query
    def test_503_action_event_country_attribute_count_by_query(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_country_attribute_count_by_query",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-country | Count: Method
    def test_503_action_event_country_attribute_count_by_method(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_country_attribute_count_by_method",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-country | Count: Authority
    def test_503_action_event_country_attribute_count_by_authority(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_country_attribute_count_by_authority",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-method | Count: Ip
    def test_503_action_event_method_attribute_count_by_ip(self, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_method_attribute_count_by_ip",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-method | Count: Path
    def test_503_action_event_method_attribute_count_by_path(self, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_method_attribute_count_by_path",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-method | Count: Uri
    def test_503_action_event_method_attribute_count_by_uri(self, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_method_attribute_count_by_uri",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-method | Count: Ip
    def test_503_action_event_method_attribute_count_by_query(self, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_method_attribute_count_by_query",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-method | Count: Ip
    def test_503_action_event_method_attribute_count_by_company(self, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_method_attribute_count_by_company",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-method | Count: Ip
    def test_503_action_event_method_attribute_count_by_country(self, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_method_attribute_count_by_country",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-method | Count: Asn
    def test_503_action_event_method_attribute_count_by_asn(self, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_method_attribute_count_by_asn",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-method | Count: Authority
    def test_503_action_event_method_attribute_count_by_authority(self, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_method_attribute_count_by_authority",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-authority | Count: Ip
    def test_503_action_event_authority_attribute_count_by_ip(self, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_authority_attribute_count_by_ip",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-authority | Count: Path
    def test_503_action_event_authority_attribute_count_by_path(self, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_authority_attribute_count_by_path",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-authority | Count: Uri
    def test_503_action_event_authority_attribute_count_by_uri(self, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_authority_attribute_count_by_uri",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-authority | Count: Query
    def test_503_action_event_authority_attribute_count_by_query(self, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_authority_attribute_count_by_query",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-authority | Count: Method
    def test_503_action_event_authority_attribute_count_by_method(self, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_authority_attribute_count_by_method",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-authority | Count: Asn
    def test_503_action_event_authority_attribute_count_by_asn(self, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_authority_attribute_count_by_asn",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-authority | Count: Company
    def test_503_action_event_authority_attribute_count_by_company(self, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_authority_attribute_count_by_company",
                                                                 3, 2, params)

    #  Action: 503  | Event: Attribute-authority | Count: Country
    def test_503_action_event_authority_attribute_count_by_country(self, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_503_action_authority_attribute_count_by_country",
                                                                 3, 2, params)

    #  Action: Challenge  | Event: Attribute-ip | Count: Path
    def test_challenge_action_event_ip_attribute_count_by_path(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_challenge_action_ip_attribute_count_by_path",
                                                                      3, 2, params)

    #  Action: Challenge  | Event: Attribute-ip | Count: Uri
    def test_challenge_action_event_ip_attribute_count_by_uri(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_challenge_action_ip_attribute_count_by_uri",
                                                                      3, 2, params)

    #  Action: Challenge  | Event: Attribute-ip | Count: Query
    def test_challenge_action_event_ip_attribute_count_by_query(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_challenge_action_ip_attribute_count_by_query",
                                                                      3, 2, params)

    #  Action: Challenge  | Event: Attribute-ip | Count: Method
    def test_challenge_action_event_ip_attribute_count_by_method(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_challenge_action_ip_attribute_count_by_method",
                                                                      3, 2, params)

    #  Action: Challenge  | Event: Attribute-ip | Count: Authority
    def test_challenge_action_event_ip_attribute_count_by_authority(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_challenge_action_ip_attribute_count_by_authority",
                                                                      3, 2, params)

    #  Action: Challenge  | Event: Attribute-asn | Count: Path
    def test_challenge_action_event_asn_attribute_count_by_path(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_chl_action_asn_attr_count_by_path",
                                                                      3, 2, params)

    #  Action: Challenge  | Event: Attribute-asn | Count: Uri
    def test_challenge_action_event_asn_attribute_count_by_uri(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_chl_action_asn_attr_count_by_uri",
                                                                      3, 2, params)

    #  Action: Challenge  | Event: Attribute-asn | Count: Query
    def test_challenge_action_event_asn_attribute_count_by_query(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_chl_action_asn_attr_count_by_query",
                                                                      3, 2, params)

    #  Action: Challenge  | Event: Attribute-asn | Count: Method
    def test_challenge_action_event_asn_attribute_count_by_method(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_chl_action_asn_attr_count_by_method",
                                                                      3, 2, params)

    #  Action: Challenge  | Event: Attribute-asn | Count: Authority
    def test_challenge_action_event_asn_attribute_count_by_authority(self, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_chl_action_asn_attr_count_by_auth",
                                                                      3, 2, params)

    #  Action: Tag only  | Event: Attribute-ip | Count: Path
    def test_tag_action_event_ip_attribute_count_by_path(self, target, log_fixture):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_action_ip_attr_count_by_path",
                                                                     "tags",
                                                                     "test-tag-action-ip-attr-count-by-path",
                                                                     1, params)

    #  Action: Tag only  | Event: Attribute-ip | Count: Method
    def test_tag_action_event_ip_attribute_count_by_method(self, target, log_fixture):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_action_ip_attr_count_by_method",
                                                                     "tags",
                                                                     "test-tag-action-ip-attr-count-by-method",
                                                                     1, params)

    #  Action: Tag only  | Event: Attribute-country | Count: Query
    def test_tag_action_event_country_attribute_count_by_query(self, target, log_fixture):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_action_country_attr_count_by_query",
                                                                     "tags",
                                                                     "test-tag-action-country-attr-count-by-query",
                                                                     1, params)

    #  Action: Tag only  | Event: Attribute-country | Count: Authority
    def test_tag_action_event_country_attribute_count_by_authority(self, target, log_fixture):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_action_country_attr_count_by_authority",
                                                                     "tags",
                                                                     "test-tag-action-country-attr-count-by-authority",
                                                                     1, params)

    #  Action: Response  | Event: Attribute - ip | Count: Path
    def test_response_action_event_ip_attribute_count_by_path(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_response_action_ip_attr_count_by_path",
                                                             "response_body_path_attr_ip", "302", 5, 2, params)

    #  Action: Response  | Event: Attribute - Method | Count: Country
    def test_response_action_event_method_attribute_count_by_country(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rl_response_action_with_params(target,
                                                             "test_response_action_method_attr_count_by_country",
                                                             "response_body_path_attr_method", "503", 5, 2, params)

    #  Action: Redirect  | Event: Attribute - Ip | Count: Path
    def test_redirect_action_event_ip_attribute_count_by_path(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_action_ip_attr_count_by_path",
                                                             "200", 2, 2, "https://yahoo.com", params)

    #  Action: Redirect  | Event: Attribute - method | Count: Company
    def test_redirect_action_event_method_attribute_count_by_company(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rl_redirect_action_with_params(target,
                                                             "test_redirect_action_method_attr_count_by_company",
                                                             "200", 2, 2, "https://yahoo.com", params)

    #  Action: Ban | Subaction: 503 | Event: Attribute - Ip | Count: path
    def test_ban_action_event_ip_subaction_503_count_by_path(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_action_by_ip_sub_503_count_path", 4,
                                                                 2, params)

    #  Action: Ban | Subaction: 503 | Event: Attribute - Ip | Count: Query
    def test_ban_action_event_ip_subaction_503_count_by_query(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_action_by_ip_sub_503_count_query",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event: Attribute - Ip | Count: Uri
    def test_ban_action_event_ip_subaction_503_count_by_uri(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_action_by_ip_sub_503_count_uri",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event: Attribute - Ip | Count: Method
    def test_ban_action_event_ip_subaction_503_count_by_method(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_action_by_ip_sub_503_count_method",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event: Attribute - Ip | Count: Authority
    def test_ban_action_event_ip_subaction_503_count_by_authority(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_ip_sub_503_count_authority",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute -  Asn | Count: Uri
    def test_ban_action_event_asn_subaction_503_count_by_uri(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_provider_sub_503_count_uri",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute -  Asn | Count: Path
    def test_ban_action_event_asn_subaction_503_count_by_path(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_provider_sub_503_count_path",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute -  Asn | Count: Query
    def test_ban_action_event_asn_subaction_503_count_by_query(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_provider_sub_503_count_query",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute -  Asn | Count: Method
    def test_ban_action_event_asn_subaction_503_count_by_method(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_provider_sub_503_count_method",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute -  Asn | Count: Authority
    def test_ban_action_event_asn_subaction_503_count_by_authority(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_provider_sub_503_count_authority",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute -  Uri | Count: Ip
    def test_ban_action_event_uri_subaction_503_count_by_ip(self, cli, target):
        RateLimitHelper.check_503_response_change_path(target, "test_ban_action_by_uri_sub_503_count_ip", 4, 2)

    #  Action: Ban | Subaction: 503 | Event:Attribute -  Uri | Count: Asn
    def test_ban_action_event_uri_subaction_503_count_by_asn(self, cli, target):
        RateLimitHelper.check_503_response_change_path(target, "test_ban_action_by_uri_sub_503_count_asn", 4, 2)

    #  Action: Ban | Subaction: 503 | Event:Attribute -  Uri | Count: Method
    def test_ban_action_event_uri_subaction_503_count_by_method(self, cli, target):
        RateLimitHelper.check_503_response_change_path(target, "test_ban_action_by_uri_sub_503_count_method", 4, 2)

    #  Action: Ban | Subaction: 503 | Event:Attribute -  Uri | Count: Country
    def test_ban_action_event_uri_subaction_503_count_by_country(self, cli, target):
        RateLimitHelper.check_503_response_change_path(target, "test_ban_action_by_uri_sub_503_count_country", 4, 2)

    #  Action: Ban | Subaction: 503 | Event:Attribute -  Uri | Count: Company
    def test_ban_action_event_uri_subaction_503_count_by_company(self, cli, target):
        RateLimitHelper.check_503_response_change_path(target, "test_ban_action_by_uri_sub_503_count_company", 4, 2)

    #  Action: Ban | Subaction: 503 | Event:Attribute -  Uri | Count: Authority
    def test_ban_action_event_uri_subaction_503_count_by_authority(self, cli, target):
        RateLimitHelper.check_503_response_change_path(target, "test_ban_action_by_uri_sub_503_count_authority", 4, 2)

    #  Action: Ban | Subaction: 503 | Event:Attribute -  Path | Count: Ip
    def test_ban_action_event_path_subaction_503_count_by_ip(self, cli, target):
        RateLimitHelper.check_503_response_change_path(target, "test_ban_action_by_path_sub_503_count_ip", 4, 2)

    #  Action: Ban | Subaction: 503 | Event:Attribute -  Path | Count: Asn
    def test_ban_action_event_path_subaction_503_count_by_asn(self, cli, target):
        RateLimitHelper.check_503_response_change_path(target, "test_ban_action_by_path_sub_503_count_asn", 4, 2)

    #  Action: Ban | Subaction: 503 | Event:Attribute -  Path | Count: Method
    def test_ban_action_event_path_subaction_503_count_by_method(self, cli, target):
        RateLimitHelper.check_503_response_change_path(target, "test_ban_action_by_path_sub_503_count_method", 4, 2)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Path | Count: Country
    def test_ban_action_event_path_subaction_503_count_by_country(self, cli, target):
        RateLimitHelper.check_503_response_change_path(target, "test_ban_action_by_path_sub_503_count_country", 4, 2)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Path | Count: Company
    def test_ban_action_event_path_subaction_503_count_by_company(self, cli, target):
        RateLimitHelper.check_503_response_change_path(target, "test_ban_action_by_path_sub_503_count_company", 4, 2)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Path | Count: Authority
    def test_ban_action_event_path_subaction_503_count_by_authority(self, cli, target):
        RateLimitHelper.check_503_response_change_path(target, "test_ban_action_by_path_sub_503_count_authority", 4, 2)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Query | Count: Ip
    def test_ban_action_event_query_subaction_503_count_by_ip(self, cli, target):
        params = [{"params": {"an_action_by_query_sub_503_count_ip": f"val-{i}"}} for i in range(2 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_query_sub_503_count_ip", 4, 2,
                                                                 params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Query | Count: Asn
    def test_ban_action_event_query_subaction_503_count_by_asn(self, cli, target):
        params = [{"params": {"an_action_by_query_sub_503_count_ip": f"val-{i}"}} for i in range(2 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_query_sub_503_count_asn", 4, 2,
                                                                 params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Query | Count: Method
    def test_ban_action_event_query_subaction_503_count_by_method(self, cli, target):
        params = [{"params": {"an_action_by_query_sub_503_count_ip": f"val-{i}"}} for i in range(2 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_query_sub_503_count_method", 4, 2,
                                                                 params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Query | Count: Country
    def test_ban_action_event_query_subaction_503_count_by_country(self, cli, target):
        params = [{"params": {"an_action_by_query_sub_503_count_ip": f"val-{i}"}} for i in range(2 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_query_sub_503_count_country", 4, 2,
                                                                 params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Query | Count: Company
    def test_ban_action_event_query_subaction_503_count_by_company(self, cli, target):
        params = [{"params": {"an_action_by_query_sub_503_count_ip": f"val-{i}"}} for i in range(2 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_query_sub_503_count_company",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Query | Count: Authority
    def test_ban_action_event_query_subaction_503_count_by_authority(self, cli, target):
        params = [{"params": {"an_action_by_query_sub_503_count_ip": f"val-{i}"}} for i in range(2 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_query_sub_503_count_authority",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Method | Count: Ip
    def test_ban_action_event_method_subaction_503_count_by_ip(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_method_sub_503_count_ip", 4, 2,
                                                                 params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Method | Count: Asn
    def test_ban_action_event_method_subaction_503_count_by_asn(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_method_sub_503_count_asn", 4, 2,
                                                                 params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Method | Count: Uri
    def test_ban_action_event_method_subaction_503_count_by_uri(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_method_sub_503_count_uri", 4,
                                                                 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Method | Count: Path
    def test_ban_action_event_method_subaction_503_count_by_path(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_method_sub_503_count_path", 4,
                                                                 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Method | Count: Query
    def test_ban_action_event_method_subaction_503_count_by_query(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_method_sub_503_count_query",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Method | Count: Country
    def test_ban_action_event_method_subaction_503_count_by_country(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_method_sub_503_count_country", 4,
                                                                 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Method | Count: Company
    def test_ban_action_event_method_subaction_503_count_by_company(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_method_sub_503_count_company", 4,
                                                                 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Method | Count: Authority
    def test_ban_action_event_method_subaction_503_count_by_authority(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_method_sub_503_count_authority",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Company | Count: uri
    def test_ban_action_event_company_subaction_503_count_by_uri(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_company_sub_503_count_uri",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Company | Count: path
    def test_ban_action_event_company_subaction_503_count_by_path(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_company_sub_503_count_path",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Company | Count: query
    def test_ban_action_event_company_subaction_503_count_by_query(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_company_sub_503_count_query",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Company | Count: method
    def test_ban_action_event_company_subaction_503_count_by_method(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_company_sub_503_count_method",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Company | Count: authority
    def test_ban_action_event_company_subaction_503_count_by_authority(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_company_sub_503_count_authority",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Country | Count: uri
    def test_ban_action_event_country_subaction_503_count_by_uri(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_country_sub_503_count_uri",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Country | Count: path
    def test_ban_action_event_country_subaction_503_count_by_path(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_country_sub_503_count_path",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Country | Count: query
    def test_ban_action_event_country_subaction_503_count_by_query(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_country_sub_503_count_query",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Country | Count: method
    def test_ban_action_event_country_subaction_503_count_by_method(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_country_sub_503_count_method",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Country | Count: authority
    def test_ban_action_event_country_subaction_503_count_by_authority(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_country_sub_503_count_authority",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: ip
    def test_ban_action_event_authority_subaction_503_count_by_ip(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_authority_sub_503_count_ip",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: path
    def test_ban_action_event_authority_subaction_503_count_by_path(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_authority_sub_503_count_path",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: uri
    def test_ban_action_event_authority_subaction_503_count_by_uri(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_authority_sub_503_count_uri",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: query
    def test_ban_action_event_authority_subaction_503_count_by_query(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_authority_sub_503_count_query",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: method
    def test_ban_action_event_authority_subaction_503_count_by_method(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_authority_sub_503_count_method",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: Asn
    def test_ban_action_event_authority_subaction_503_count_by_asn(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_authority_sub_503_count_asn",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: country
    def test_ban_action_event_authority_subaction_503_count_by_country(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_authority_sub_503_count_country",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: company
    def test_ban_action_event_authority_subaction_503_count_by_company(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target,
                                                                 "test_ban_action_by_authority_sub_503_count_company",
                                                                 4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Company | Count: uri
    def test_ban_action_event_company_subaction_503_count_by_uri(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_company_sub_503_count_uri",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Company | Count: path
    def test_ban_action_event_company_subaction_503_count_by_path(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_company_sub_503_count_path",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Company | Count: query
    def test_ban_action_event_company_subaction_503_count_by_query(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_company_sub_503_count_query",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Company | Count: method
    def test_ban_action_event_company_subaction_503_count_by_method(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_company_sub_503_count_method",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Company | Count: authority
    def test_ban_action_event_company_subaction_503_count_by_authority(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_company_sub_503_count_authority",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Country | Count: uri
    def test_ban_action_event_country_subaction_503_count_by_uri(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_country_sub_503_count_uri",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Country | Count: path
    def test_ban_action_event_country_subaction_503_count_by_path(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_country_sub_503_count_path",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Country | Count: query
    def test_ban_action_event_country_subaction_503_count_by_query(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_country_sub_503_count_query",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Country | Count: method
    def test_ban_action_event_country_subaction_503_count_by_method(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_country_sub_503_count_method",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Country | Count: authority
    def test_ban_action_event_country_subaction_503_count_by_authority(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_country_sub_503_count_authority",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: ip
    def test_ban_action_event_authority_subaction_503_count_by_ip(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_authority_sub_503_count_ip",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: path
    def test_ban_action_event_authority_subaction_503_count_by_path(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_authority_sub_503_count_path",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: uri
    def test_ban_action_event_authority_subaction_503_count_by_uri(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_authority_sub_503_count_uri",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: query
    def test_ban_action_event_authority_subaction_503_count_by_query(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_authority_sub_503_count_query",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: method
    def test_ban_action_event_authority_subaction_503_count_by_method(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_authority_sub_503_count_method",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: Asn
    def test_ban_action_event_authority_subaction_503_count_by_asn(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_authority_sub_503_count_asn",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: country
    def test_ban_action_event_authority_subaction_503_count_by_country(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_authority_sub_503_count_country",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: 503 | Event:Attribute - Authority | Count: company
    def test_ban_action_event_authority_subaction_503_count_by_company(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_for_geo_attr(target,
                                                                  "test_ban_action_by_authority_sub_503_count_company",
                                                                  4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute -  Ip | Count: Path
    def test_ban_action_event_ip_subaction_challenge_count_by_path(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_ip_sub_chl_count_path",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute -  Ip | Count: Uri
    def test_ban_action_event_ip_subaction_challenge_count_by_uri(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_ip_sub_chl_count_uri",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute -  Ip | Count: Query
    def test_ban_action_event_ip_subaction_challenge_count_by_query(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_ip_sub_chl_count_query",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute -  Ip | Count: Method
    def test_ban_action_event_ip_subaction_challenge_count_by_method(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_ip_sub_chl_count_method",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute -  Ip | Count: Authority
    def test_ban_action_event_ip_subaction_challenge_count_by_authority(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_ip_sub_chl_count_authority",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute -  Asn | Count: Uri
    def test_ban_action_event_asn_subaction_challenge_count_by_uri(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_provider_sub_chl_count_uri",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute -  Asn | Count: Path
    def test_ban_action_event_asn_subaction_challenge_count_by_path(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_provider_sub_chl_count_path",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Asn | Count: Query
    def test_ban_action_event_asn_subaction_challenge_count_by_query(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_provider_sub_chl_count_query",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Asn | Count: Method
    def test_ban_action_event_asn_subaction_challenge_count_by_method(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_provider_sub_chl_count_method",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Asn | Count: Authority
    def test_ban_action_event_asn_subaction_challenge_count_by_authority(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_provider_sub_chl_count_authority",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Uri | Count: Ip
    def test_ban_action_event_uri_subaction_challenge_count_by_ip(self, cli, target):
        RateLimitHelper.check_challenge_response_change_path(target, "test_ban_action_by_uri_sub_chl_count_ip", 4, 2)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Uri | Count: Asn
    def test_ban_action_event_uri_subaction_challenge_count_by_asn(self, cli, target):
        RateLimitHelper.check_challenge_response_change_path(target, "test_ban_action_by_uri_sub_chl_count_asn", 4, 2)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Uri | Count: Method
    def test_ban_action_event_uri_subaction_challenge_count_by_method(self, cli, target):
        RateLimitHelper.check_challenge_response_change_path(target, "test_ban_action_by_uri_sub_chl_count_method",
                                                             4, 2)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Uri | Count: Company
    def test_ban_action_event_uri_subaction_challenge_count_by_company(self, cli, target):
        RateLimitHelper.check_challenge_response_change_path(target,
                                                             "test_ban_action_by_uri_sub_chl_count_company", 4, 2)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Uri | Count: Authority
    def test_ban_action_event_uri_subaction_challenge_count_by_authority(self, cli, target):
        RateLimitHelper.check_challenge_response_change_path(target,
                                                             "test_ban_action_by_uri_sub_chl_count_authority", 4, 2)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Path | Count: Ip
    def test_ban_action_event_path_subaction_challenge_count_by_ip(self, cli, target):
        RateLimitHelper.check_challenge_response_change_path(target, "test_ban_action_by_path_sub_chl_count_ip", 4, 2)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Path | Count: Asn
    def test_ban_action_event_path_subaction_challenge_count_by_asn(self, cli, target):
        RateLimitHelper.check_challenge_response_change_path(target, "test_ban_action_by_path_sub_chl_count_asn", 4, 2)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Path | Count: Method
    def test_ban_action_event_path_subaction_challenge_count_by_method(self, cli, target):
        RateLimitHelper.check_challenge_response_change_path(target, "test_ban_action_by_path_sub_chl_count_method",
                                                             4, 2)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Path | Count: Company
    def test_ban_action_event_path_subaction_challenge_count_by_company(self, cli, target):
        RateLimitHelper.check_challenge_response_change_path(target,
                                                             "test_ban_action_by_path_sub_chl_count_company", 4, 2)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Path | Count: Authority
    def test_ban_action_event_path_subaction_challenge_count_by_authority(self, cli, target):
        RateLimitHelper.check_challenge_response_change_path(target,
                                                             "test_ban_action_by_path_sub_chl_count_authority", 4, 2)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Query | Count: Ip
    def test_ban_action_event_query_subaction_challenge_count_by_ip(self, cli, target):
        params = [{"suffix": f"?QUERY-{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_query_sub_chl_count_ip",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Query | Count: Asn
    def test_ban_action_event_query_subaction_challenge_count_by_asn(self, cli, target):
        params = [{"suffix": f"?QUERY-{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_query_sub_chl_count_asn",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Query | Count: Method
    def test_ban_action_event_query_subaction_challenge_count_by_method(self, cli, target):
        params = [{"suffix": f"?QUERY-{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_query_sub_chl_count_method",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Query | Count: Company
    def test_ban_action_event_query_subaction_challenge_count_by_company(self, cli, target):
        params = [{"suffix": f"?QUERY-{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_query_sub_chl_count_company",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Query | Count: Authority
    def test_ban_action_event_query_subaction_challenge_count_by_authority(self, cli, target):
        params = [{"suffix": f"?QUERY-{i}"} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_query_sub_chl_count_auth",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Method | Count: Ip
    def test_ban_action_event_method_subaction_challenge_count_by_ip(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_method_sub_chl_count_ip", 4,
                                                                      2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Method | Count: Uri
    def test_ban_action_event_method_subaction_challenge_count_by_uri(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_method_sub_chl_count_uri",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Method | Count: Path
    def test_ban_action_event_method_subaction_challenge_count_by_path(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_method_sub_chl_count_path",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Method | Count: Query
    def test_ban_action_event_method_subaction_challenge_count_by_query(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_method_sub_chl_count_query",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Method | Count: Asn
    def test_ban_action_event_method_subaction_challenge_count_by_asn(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_method_sub_chl_count_asn",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Method | Count: Company
    def test_ban_action_event_method_subaction_challenge_count_by_company(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_method_sub_chl_count_comp",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Method | Count: Country
    def test_ban_action_event_method_subaction_challenge_count_by_country(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_method_sub_chl_count_country",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Method | Count: Authority
    def test_ban_action_event_method_subaction_challenge_count_by_authority(self, cli, target):
        params = [{"method": m} for m in ("GET", "HEAD", "POST", "PUT")]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_method_sub_chl_count_auth",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Company | Count: Uri
    def test_ban_action_event_company_subaction_challenge_count_by_uri(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_company_sub_chl_count_uri",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Company | Count: Path
    def test_ban_action_event_company_subaction_challenge_count_by_path(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_company_sub_chl_count_path",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Company | Count: Query
    def test_ban_action_event_company_subaction_challenge_count_by_query(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_company_sub_chl_count_query",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Company | Count: Method
    def test_ban_action_event_company_subaction_challenge_count_by_method(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_company_sub_chl_count_method",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Company | Count: Authority
    def test_ban_action_event_company_subaction_challenge_count_by_authority(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_company_sub_chl_count_auth",
                                                                      4, 2, params)

        #  Action: Ban | Subaction: Challenge | Event:Attribute - Country | Count: Uri

    def test_ban_action_event_country_subaction_challenge_count_by_uri(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_country_sub_chl_count_uri",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Country | Count: Path
    def test_ban_action_event_country_subaction_challenge_count_by_path(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_country_sub_chl_count_path",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Country | Count: Query
    def test_ban_action_event_country_subaction_challenge_count_by_query(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_country_sub_chl_count_query",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Country | Count: Method
    def test_ban_action_event_country_subaction_challenge_count_by_method(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_country_sub_chl_count_method",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Country | Count: Authority
    def test_ban_action_event_country_subaction_challenge_count_by_authority(self, cli, target):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_country_sub_chl_count_auth",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Authority | Count: Ip
    def test_ban_action_event_authority_subaction_challenge_count_by_ip(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_auth_sub_chl_count_ip",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Authority | Count: Uri
    def test_ban_action_event_authority_subaction_challenge_count_by_uri(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_auth_sub_chl_count_uri",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Authority | Count: Query
    def test_ban_action_event_authority_subaction_challenge_count_by_query(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_auth_sub_chl_count_query",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Authority | Count: Path
    def test_ban_action_event_authority_subaction_challenge_count_by_path(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_auth_sub_chl_count_path",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Authority | Count: Method
    def test_ban_action_event_authority_subaction_challenge_count_by_method(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_auth_sub_chl_count_method",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Authority | Count: Asn
    def test_ban_action_event_authority_subaction_challenge_count_by_asn(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_auth_sub_chl_count_asn",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Authority | Count: Country
    def test_ban_action_event_authority_subaction_challenge_count_by_country(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_auth_sub_chl_count_country",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Challenge | Event:Attribute - Authority | Count: Company
    def test_ban_action_event_authority_subaction_challenge_count_by_company(self, cli, target):
        params = [{"headers": {"Host": f"authority-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_action_by_auth_sub_chl_count_company",
                                                                      4, 2, params)

    #  Action: Ban | Subaction: Tag only | Event:Attribute -  Ip | Count: Path
    def test_ban_action_event_ip_subaction_tag_count_by_path(self, cli, target, log_fixture):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_action_by_ip_sub_tag_count_path",
                                                                     "tags",
                                                                     "test-ban-action-by-ip-sub-tag-count-path",
                                                                     1, params)

    #  Action: Ban | Subaction: Tag only | Event:Attribute -  Asn | Count: Uri
    def test_ban_action_event_asn_subaction_tag_count_by_uri(self, cli, target, log_fixture):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_action_by_provider_sub_tag_count_uri",
                                                                     "tags",
                                                                     "test-ban-action-by-provider-sub-tag-count-uri",
                                                                     1, params)

    #  Action: Ban | Subaction: Response| Event:Attribute -  Ip | Count: Path
    def test_ban_action_event_ip_subaction_response_count_by_path(self, cli, target, log_fixture):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_action_by_ip_sub_res_count_path",
                                                             "response_attr_ip_by_path", "503", 3, 2, params)

    #  Action: Ban | Subaction: Response | Event:Attribute -  Asn | Count: Uri
    def test_ban_action_event_asn_subaction_response_count_by_uri(self, cli, target, log_fixture):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rl_response_action_with_params(target,
                                                             "test_ban_action_by_provider_sub_res_count_uri",
                                                             "response_attr_ip_by_uri",
                                                             "503", 3, 2, params)

    #  Action: Ban | Subaction: Redirect| Event:Attribute -  Ip | Count: Path
    def test_ban_action_event_ip_subaction_redirect_count_by_path(self, cli, target, log_fixture):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_action_by_ip_sub_red_count_path",
                                                             "200", 3, 2, "https://google.com", params)

    #  Action: Ban | Subaction: Redirect | Event:Attribute -  Asn | Count: Uri
    def test_ban_action_event_asn_subaction_redirect_count_by_uri(self, cli, target, log_fixture):
        params = [{"srcip": ip} for ip in (BaseHelper.IP4_US, BaseHelper.IP4_JP,
                                           BaseHelper.IP4_CLOUDFLARE, BaseHelper.IP4_ORANGE)]
        RateLimitHelper.check_rl_redirect_action_with_params(target,
                                                             "test_ban_action_by_provider_sub_red_count_uri",
                                                             "200", 3, 2, "https://google.com", params)
