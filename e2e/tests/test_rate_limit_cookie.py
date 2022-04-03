import pytest
from e2e.core.base_helper import cli, target, section, log_fixture
from e2e.helpers.rl_helper import RateLimitHelper
from e2e.helpers.rl_cookie_helper import ratelimit_cookie_config


@pytest.mark.usefixtures("api_setup", "ratelimit_cookie_config")
@pytest.mark.all_modules
@pytest.mark.rate_limit_tests
@pytest.mark.rate_limit_cookie_tests
class TestRateLimitCookie:

    #  Action: 503  | Event: Cookie | Count: Ip
    def test_503_action_event_cookie_count_by_ip(self, target):
        params = [
            {"cookies": {"cookie_event_503_ip": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_503_action_cookie_count_by_ip", 3, 3, params
        )

    #  Action: 503  | Event: Cookie | Count: Path
    def test_503_action_event_cookie_count_by_path(self, target):
        params = [
            {"cookies": {"cookie_event_503_path": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_503_action_cookie_count_by_path", 3, 3, params
        )

    #  Action: 503  | Event: Cookie | Count: Uri
    def test_503_action_event_cookie_count_by_uri(self, target):
        params = [
            {"cookies": {"cookie_event_503_uri": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_503_action_cookie_count_by_uri", 3, 3, params
        )

    #  Action: 503  | Event: Cookie | Count: Asn
    def test_503_action_event_cookie_count_by_asn(self, target):
        params = [
            {"cookies": {"cookie_event_503_asn": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_503_action_cookie_count_by_asn", 3, 3, params
        )

    #  Action: 503  | Event: Cookie | Count: Query
    def test_503_action_event_cookie_count_by_query(self, target):
        params = [
            {"cookies": {"cookie_event_503_query": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_503_action_cookie_count_by_query", 3, 3, params
        )

    #  Action: 503  | Event: Cookie | Count: Method
    def test_503_action_event_cookie_count_by_method(self, target):
        params = [
            {"cookies": {"cookie_event_503_method": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_503_action_cookie_count_by_method", 3, 3, params
        )

    #  Action: 503  | Event: Cookie | Count: Company
    def test_503_action_event_cookie_count_by_company(self, target):
        params = [
            {"cookies": {"cookie_event_503_company": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_503_action_cookie_count_by_company", 3, 3, params
        )

    #  Action: 503  | Event: Cookie | Count: Country
    def test_503_action_event_cookie_count_by_country(self, target):
        params = [
            {"cookies": {"cookie_event_503_country": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_503_action_cookie_count_by_country", 3, 3, params
        )

    #  Action: 503  | Event: Cookie | Count: Authority
    def test_503_action_event_cookie_count_by_authority(self, target):
        params = [
            {"cookies": {"cookie_event_503_authority": f"val-{i}"}}
            for i in range(3 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_503_action_cookie_count_by_authority", 3, 3, params
        )

    #  Action: Tag only  | Event: Cookie | Count: Ip
    def test_tag_only_action_event_cookie_count_by_ip(self, target, log_fixture):
        params = [
            {"cookies": {"cookie_event_tag_ip": f"val-{i}"}} for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_tag_cookie_ip",
            "tags",
            "test-tag-cookie-ip",
            1,
            params,
        )

    #  Action: Tag only  | Event: Cookie | Count: Uri
    def test_tag_only_action_event_cookie_count_by_uri(self, target, log_fixture):
        params = [
            {"cookies": {"cookie_event_tag_uri": f"val-{i}"}} for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_tag_cookie_uri",
            "tags",
            "test-tag-cookie-uri",
            1,
            params,
        )

    #  Action: Tag only  | Event: Cookie | Count: Asn
    def test_tag_only_action_event_cookie_count_by_asn(self, target, log_fixture):
        params = [
            {"cookies": {"cookie_event_tag_asn": f"val-{i}"}} for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_tag_cookie_asn",
            "tags",
            "test-tag-cookie-asn",
            1,
            params,
        )

    #  Action: Tag only  | Event: Cookie | Count: Path
    def test_tag_only_action_event_cookie_count_by_path(self, target, log_fixture):
        params = [
            {"cookies": {"cookie_event_tag_path": f"val-{i}"}} for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_tag_cookie_path",
            "tags",
            "test-tag-cookie-path",
            1,
            params,
        )

    #  Action: Tag only  | Event: Cookie | Count: Query
    def test_tag_only_action_event_cookie_count_by_query(self, target, log_fixture):
        params = [
            {"cookies": {"cookie_event_tag_query": f"val-{i}"}} for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_tag_cookie_query",
            "tags",
            "test-tag-cookie-query",
            1,
            params,
        )

    #  Action: Tag only  | Event: Cookie | Count: Method
    def test_tag_only_action_event_cookie_count_by_method(self, target, log_fixture):
        params = [
            {"cookies": {"cookie_event_tag_method": f"val-{i}"}} for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_tag_cookie_method",
            "tags",
            "test-tag-cookie-method",
            1,
            params,
        )

    #  Action: Tag only  | Event: Cookie | Count: Company
    def test_tag_only_action_event_cookie_count_by_company(self, target, log_fixture):
        params = [
            {"cookies": {"cookie_event_tag_company": f"val-{i}"}} for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_tag_cookie_company",
            "tags",
            "test-tag-cookie-company",
            1,
            params,
        )

    #  Action: Tag only  | Event: Cookie | Count: Country
    def test_tag_only_action_event_cookie_count_by_country(self, target, log_fixture):
        params = [
            {"cookies": {"cookie_event_tag_country": f"val-{i}"}} for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_tag_cookie_country",
            "tags",
            "test-tag-cookie-country",
            1,
            params,
        )

    #  Action: Tag only  | Event: Cookie | Count: Authority
    def test_tag_only_action_event_cookie_count_by_authority(self, target, log_fixture):
        params = [
            {"cookies": {"cookie_event_tag_authority": f"val-{i}"}}
            for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_tag_cookie_authority",
            "tags",
            "test-tag-cookie-authority",
            1,
            params,
        )

    #  Action: Response  | Event: Cookie | Count: Ip
    def test_response_action_event_cookie_count_by_ip(self, cli, target):
        params = [
            {"cookies": {"cookie_event_res_ip": f"val-{i}"}} for i in range(5 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target, "test_res_cookie_ip", "response_body_ip", "302", 3, 5, params
        )

    #  Action: Response  | Event: Cookie | Count: Uri
    def test_response_action_event_cookie_count_by_uri(self, cli, target):
        params = [
            {"cookies": {"cookie_event_res_uri": f"val-{i}"}} for i in range(5 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target, "test_res_cookie_uri", "response_body_uri", "503", 3, 5, params
        )

    #  Action: Response  | Event: Cookie | Count: Query
    def test_response_action_event_cookie_count_by_query(self, cli, target):
        params = [
            {"cookies": {"cookie_event_res_query": f"val-{i}"}} for i in range(5 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target, "test_res_cookie_query", "response_body_query", "302", 3, 5, params
        )

    #  Action: Response  | Event: Cookie | Count: Path
    def test_response_action_event_cookie_count_by_path(self, cli, target):
        params = [
            {"cookies": {"cookie_event_res_path": f"val-{i}"}} for i in range(5 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target, "test_res_cookie_path", "response_body_path", "302", 3, 5, params
        )

    #  Action: Response  | Event: Cookie | Count: Asn
    def test_response_action_event_cookie_count_by_asn(self, cli, target):
        params = [
            {"cookies": {"cookie_event_res_asn": f"val-{i}"}} for i in range(5 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target, "test_res_cookie_asn", "response_body_asn", "302", 3, 5, params
        )

    #  Action: Response  | Event: Cookie | Count: Method
    def test_response_action_event_cookie_count_by_method(self, cli, target):
        params = [
            {"cookies": {"cookie_event_res_method": f"val-{i}"}} for i in range(5 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target,
            "test_res_cookie_method",
            "response_body_method",
            "302",
            3,
            5,
            params,
        )

    #  Action: Response  | Event: Cookie | Count: Company
    def test_response_action_event_cookie_count_by_company(self, cli, target):
        params = [
            {"cookies": {"cookie_event_res_company": f"val-{i}"}} for i in range(5 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target,
            "test_res_cookie_company",
            "response_body_company",
            "302",
            3,
            5,
            params,
        )

    #  Action: Response  | Event: Cookie | Count: Country
    def test_response_action_event_cookie_count_by_country(self, cli, target):
        params = [
            {"cookies": {"cookie_event_res_country": f"val-{i}"}} for i in range(5 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target,
            "test_res_cookie_country",
            "response_body_country",
            "302",
            3,
            5,
            params,
        )

    #  Action: Response  | Event: Cookie | Count: Authority
    def test_response_action_event_cookie_count_by_authority(self, cli, target):
        params = [
            {"cookies": {"cookie_event_res_authority": f"val-{i}"}}
            for i in range(2 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target,
            "test_res_cookie_authority",
            "response_body_authority",
            "302",
            4,
            2,
            params,
        )

    #  Action: Challenge  | Event: Cookie | Count: Ip
    def test_challenge_action_event_cookie_count_by_ip(self, cli, target):
        params = [
            {"cookies": {"cookie_event_chl_ip": f"val-{i}"}} for i in range(5 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_challenge_cookie_ip", 4, 5, params
        )

    #  Action: Challenge  | Event: Cookie | Count: Uri
    def test_challenge_action_event_cookie_count_by_uri(self, cli, target):
        params = [
            {"cookies": {"cookie_event_chl_uri": f"val-{i}"}} for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_challenge_cookie_uri", 4, 2, params
        )

    #  Action: Challenge  | Event: Cookie | Count: Query
    def test_challenge_action_event_cookie_count_by_query(self, cli, target):
        params = [
            {"cookies": {"cookie_event_chl_query": f"val-{i}"}} for i in range(5 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_challenge_cookie_query", 4, 5, params
        )

    #  Action: Challenge  | Event: Cookie | Count: Path
    def test_challenge_action_event_cookie_count_by_path(self, cli, target):
        params = [
            {"cookies": {"cookie_event_chl_path": f"val-{i}"}} for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_challenge_cookie_path", 4, 2, params
        )

    #  Action: Challenge  | Event: Cookie | Count: Asn
    def test_challenge_action_event_cookie_count_by_asn(self, cli, target):
        params = [
            {"cookies": {"cookie_event_chl_asn": f"val-{i}"}} for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_challenge_cookie_asn", 4, 2, params
        )

    #  Action: Challenge  | Event: Cookie | Count: Method
    def test_challenge_action_event_cookie_count_by_method(self, cli, target):
        params = [
            {"cookies": {"cookie_event_chl_method": f"val-{i}"}} for i in range(5 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_challenge_cookie_method", 4, 5, params
        )

    #  Action: Challenge  | Event: Cookie | Count: Company
    def test_challenge_action_event_cookie_count_by_company(self, cli, target):
        params = [
            {"cookies": {"cookie_event_chl_company": f"val-{i}"}} for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_challenge_cookie_company", 4, 2, params
        )

    #  Action: Challenge  | Event: Cookie | Count: Country
    def test_challenge_action_event_cookie_count_by_country(self, cli, target):
        params = [
            {"cookies": {"cookie_event_chl_country": f"val-{i}"}} for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_challenge_cookie_country", 4, 2, params
        )

    #  Action: Challenge  | Event: Cookie | Count: Authority
    def test_challenge_action_event_cookie_count_by_authority(self, cli, target):
        params = [
            {"cookies": {"cookie_event_chl_authority": f"val-{i}"}}
            for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_challenge_cookie_authority", 4, 2, params
        )

    #  Action: Redirect  | Event: Cookie | Count: Ip
    def test_redirect_action_event_cookie_count_by_ip(self, cli, target):
        params = [
            {"cookies": {"cookie_event_red_ip": f"val-{i}"}} for i in range(2 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target, "test_redirect_cookie_ip", "200", 4, 2, "https://yahoo.com", params
        )

    #  Action: Redirect  | Event: Cookie | Count: Uri
    def test_redirect_action_event_cookie_count_by_uri(self, cli, target):
        params = [
            {"cookies": {"cookie_event_red_uri": f"val-{i}"}} for i in range(2 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target, "test_redirect_cookie_uri", "200", 4, 2, "https://yahoo.com", params
        )

    #  Action: Redirect  | Event: Cookie | Count: Query
    def test_redirect_action_event_cookie_count_by_query(self, cli, target):
        params = [
            {"cookies": {"cookie_event_red_query": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target,
            "test_redirect_cookie_query",
            "200",
            5,
            3,
            "https://yahoo.com",
            params,
        )

    #  Action: Redirect  | Event: Cookie | Count: Path
    def test_redirect_action_event_cookie_count_by_path(self, cli, target):
        params = [
            {"cookies": {"cookie_event_red_path": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target,
            "test_redirect_cookie_path",
            "200",
            4,
            3,
            "https://yahoo.com",
            params,
        )

    #  Action: Redirect  | Event: Cookie | Count: Asn
    def test_redirect_action_event_cookie_count_by_asn(self, cli, target):
        params = [
            {"cookies": {"cookie_event_red_asn": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target, "test_redirect_cookie_asn", "200", 3, 3, "https://yahoo.com", params
        )

    #  Action: Redirect  | Event: Cookie | Count: Method
    def test_redirect_action_event_cookie_count_by_method(self, cli, target):
        params = [
            {"cookies": {"cookie_event_red_method": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target,
            "test_redirect_cookie_method",
            "200",
            3,
            3,
            "https://yahoo.com",
            params,
        )

    #  Action: Redirect  | Event: Cookie | Count: Company
    def test_redirect_action_event_cookie_count_by_company(self, cli, target):
        params = [
            {"cookies": {"cookie_event_red_company": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target,
            "test_redirect_cookie_company",
            "200",
            3,
            3,
            "https://yahoo.com",
            params,
        )

    #  Action: Redirect  | Event: Cookie | Count: Country
    def test_redirect_action_event_cookie_count_by_country(self, cli, target):
        params = [
            {"cookies": {"cookie_event_red_country": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target,
            "test_redirect_cookie_country",
            "200",
            3,
            3,
            "https://yahoo.com",
            params,
        )

    #  Action: Redirect  | Event: Cookie | Count: Authority
    def test_redirect_action_event_cookie_count_by_authority(self, cli, target):
        params = [
            {"cookies": {"cookie_event_red_authority": f"val-{i}"}}
            for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target,
            "test_redirect_cookie_authority",
            "200",
            3,
            3,
            "https://yahoo.com",
            params,
        )

    #  Action: Redirect | Subaction: 503 | Event: Cookie | Count: Ip
    def test_ban_action_event_cookie_subaction_503_count_by_ip(self, cli, target):
        params = [
            {"cookies": {"cookie_event_ban_503_ip": f"val-{i}"}} for i in range(4 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_ban_cookie_sub_503_count_by_ip", 4, 4, params
        )

    #  Action: Redirect | Subaction: 503 | Event: Cookie | Count: Uri
    def test_ban_action_event_cookie_subaction_503_count_by_uri(self, cli, target):
        params = [
            {"cookies": {"cookie_event_ban_503_uri": f"val-{i}"}} for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_ban_cookie_sub_503_count_by_uri", 4, 2, params
        )

    #  Action: Redirect | Subaction: 503 | Event: Cookie | Count: Query
    def test_ban_action_event_cookie_subaction_503_count_by_query(self, cli, target):
        params = [
            {"cookies": {"cookie_event_ban_503_query": f"val-{i}"}}
            for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_ban_cookie_sub_503_count_by_query", 4, 2, params
        )

    #  Action: Redirect | Subaction: 503 | Event: Cookie | Count: Path
    def test_ban_action_event_cookie_subaction_503_count_by_path(self, cli, target):
        params = [
            {"cookies": {"cookie_event_ban_503_path": f"val-{i}"}} for i in range(4 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_ban_cookie_sub_503_count_by_path", 4, 4, params
        )

    #  Action: Redirect | Subaction: 503 | Event: Cookie | Count: Asn
    def test_ban_action_event_cookie_subaction_503_count_by_asn(self, cli, target):
        params = [
            {"cookies": {"cookie_event_ban_503_asn": f"val-{i}"}} for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_ban_cookie_sub_503_count_by_asn", 4, 2, params
        )

    #  Action: Redirect | Subaction: 503 | Event: Cookie | Count: Method
    def test_ban_action_event_cookie_subaction_503_count_by_method(self, cli, target):
        params = [
            {"cookies": {"cookie_event_ban_503_method": f"val-{i}"}}
            for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_ban_cookie_sub_503_count_by_method", 4, 2, params
        )

    #  Action: Redirect | Subaction: 503 | Event: Cookie | Count: Company
    def test_ban_action_event_cookie_subaction_503_count_by_company(self, cli, target):
        params = [
            {"cookies": {"cookie_event_ban_503_company": f"val-{i}"}}
            for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_ban_cookie_sub_503_count_by_company", 4, 2, params
        )

    #  Action: Redirect | Subaction: 503 | Event: Cookie | Count: Country
    def test_ban_action_event_cookie_subaction_503_count_by_country(self, cli, target):
        params = [
            {"cookies": {"cookie_event_ban_503_country": f"val-{i}"}}
            for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_ban_cookie_sub_503_count_by_country", 4, 2, params
        )

    #  Action: Redirect | Subaction: 503 | Event: Cookie | Count: Authority
    def test_ban_action_event_cookie_subaction_503_count_by_authority(
        self, cli, target
    ):
        params = [
            {"cookies": {"cookie_event_ban_503_authority": f"val-{i}"}}
            for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limits_action_503_with_params(
            target, "test_ban_cookie_sub_503_count_by_authority", 4, 2, params
        )

    #  Action: Redirect | Subaction: Challenge | Event: Cookie | Count: Ip
    def test_ban_action_event_cookie_subaction_challenge_count_by_ip(self, cli, target):
        params = [
            {"cookies": {"cookie_event_ban_chl_ip": f"val-{i}"}} for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_ban_cookie_sub_chl_count_by_ip", 6, 2, params
        )

    #  Action: Redirect | Subaction: Challenge | Event: Cookie | Count: Uri
    def test_ban_action_event_cookie_subaction_challenge_count_by_uri(
        self, cli, target
    ):
        params = [
            {"cookies": {"cookie_event_ban_chl_uri": f"val-{i}"}} for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_ban_cookie_sub_chl_count_by_uri", 6, 2, params
        )

    #  Action: Redirect | Subaction: Challenge | Event: Cookie | Count: Query
    def test_ban_action_event_cookie_subaction_challenge_count_by_query(
        self, cli, target
    ):
        params = [
            {"cookies": {"cookie_event_ban_chl_query": f"val-{i}"}}
            for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_ban_cookie_sub_chl_count_by_query", 6, 2, params
        )

    #  Action: Redirect | Subaction: Challenge | Event: Cookie | Count: Path
    def test_ban_action_event_cookie_subaction_challenge_count_by_path(
        self, cli, target
    ):
        params = [
            {"cookies": {"cookie_event_ban_chl_path": f"val-{i}"}} for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_ban_cookie_sub_chl_count_by_path", 6, 2, params
        )

    #  Action: Redirect | Subaction: Challenge | Event: Cookie | Count: Asn
    def test_ban_action_event_cookie_subaction_challenge_count_by_asn(
        self, cli, target
    ):
        params = [
            {"cookies": {"cookie_event_ban_chl_asn": f"val-{i}"}} for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_ban_cookie_sub_chl_count_by_asn", 6, 2, params
        )

    #  Action: Redirect | Subaction: Challenge | Event: Cookie | Count: Method
    def test_ban_action_event_cookie_subaction_challenge_count_by_method(
        self, cli, target
    ):
        params = [
            {"cookies": {"cookie_event_ban_chl_method": f"val-{i}"}}
            for i in range(5 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_ban_cookie_sub_chl_count_by_method", 6, 5, params
        )

    #  Action: Redirect | Subaction: Challenge | Event: Cookie | Count: Company
    def test_ban_action_event_cookie_subaction_challenge_count_by_company(
        self, cli, target
    ):
        params = [
            {"cookies": {"cookie_event_ban_chl_company": f"val-{i}"}}
            for i in range(5 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_ban_cookie_sub_chl_count_by_company", 6, 5, params
        )

    #  Action: Redirect | Subaction: Challenge | Event: Cookie | Count: Country
    def test_ban_action_event_cookie_subaction_challenge_count_by_country(
        self, cli, target
    ):
        params = [
            {"cookies": {"cookie_event_ban_chl_country": f"val-{i}"}}
            for i in range(5 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_ban_cookie_sub_chl_count_by_country", 6, 5, params
        )

    #  Action: Redirect | Subaction: Challenge | Event: Cookie | Count: Authority
    def test_ban_action_event_cookie_subaction_challenge_count_by_authority(
        self, cli, target
    ):
        params = [
            {"cookies": {"cookie_event_ban_chl_authority": f"val-{i}"}}
            for i in range(2 + 2)
        ]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(
            target, "test_ban_cookie_sub_chl_count_by_authority", 5, 2, params
        )

    #  Action: Redirect | Subaction: Tag only | Event: Cookie | Count: Ip
    def test_ban_action_event_cookie_subaction_tag_only_count_by_ip(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_tag_ip": f"val-{i}"}} for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_ban_cookie_sub_tag_count_by_ip",
            "tags",
            "test-ban-cookie-sub-tag-count-by-ip",
            1,
            params,
        )

    #  Action: Redirect | Subaction: Tag only | Event: Cookie | Count: Uri
    def test_ban_action_event_cookie_subaction_tag_only_count_by_uri(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_tag_uri": f"val-{i}"}} for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_ban_cookie_sub_tag_count_by_uri",
            "tags",
            "test-ban-cookie-sub-tag-count-by-uri",
            1,
            params,
        )

    #  Action: Redirect | Subaction: Tag only | Event: Cookie | Count: Query
    def test_ban_action_event_cookie_subaction_tag_only_count_by_query(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_tag_query": f"val-{i}"}}
            for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_ban_cookie_sub_tag_count_by_query",
            "tags",
            "test-ban-cookie-sub-tag-count-by-query",
            1,
            params,
        )

    #  Action: Redirect | Subaction: Tag only | Event: Cookie | Count: Path
    def test_ban_action_event_cookie_subaction_tag_only_count_by_path(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_tag_path": f"val-{i}"}} for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_ban_cookie_sub_tag_count_by_path",
            "tags",
            "test-ban-cookie-sub-tag-count-by-path",
            1,
            params,
        )

    #  Action: Redirect | Subaction: Tag only | Event: Cookie | Count: Asn
    def test_ban_action_event_cookie_subaction_tag_only_count_by_asn(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_tag_asn": f"val-{i}"}} for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_ban_cookie_sub_tag_count_by_asn",
            "tags",
            "test-ban-cookie-sub-tag-count-by-asn",
            1,
            params,
        )

    #  Action: Redirect | Subaction: Tag only | Event: Cookie | Count: Method
    def test_ban_action_event_cookie_subaction_tag_only_count_by_method(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_tag_method": f"val-{i}"}}
            for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_ban_cookie_sub_tag_count_by_method",
            "tags",
            "test-ban-cookie-sub-tag-count-by-method",
            1,
            params,
        )

    #  Action: Redirect | Subaction: Tag only | Event: Cookie | Count: Company
    def test_ban_action_event_cookie_subaction_tag_only_count_by_company(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_tag_company": f"val-{i}"}}
            for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_ban_cookie_sub_tag_count_by_company",
            "tags",
            "test-ban-cookie-sub-tag-count-by-company",
            1,
            params,
        )

    #  Action: Redirect | Subaction: Tag only | Event: Cookie | Count: Country
    def test_ban_action_event_cookie_subaction_tag_only_count_by_country(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_tag_country": f"val-{i}"}}
            for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_ban_cookie_sub_tag_count_by_country",
            "tags",
            "test-ban-cookie-sub-tag-count-by-country",
            1,
            params,
        )

    #  Action: Redirect | Subaction: Tag only | Event: Cookie | Count: Authority
    def test_ban_action_event_cookie_subaction_tag_only_count_by_authority(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_tag_authority": f"val-{i}"}}
            for i in range(1 + 2)
        ]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(
            log_fixture,
            target,
            "test_ban_cookie_sub_tag_count_by_authority",
            "tags",
            "test-ban-cookie-sub-tag-count-by-authority",
            1,
            params,
        )

    #  Action: Redirect | Subaction: Response | Event: Cookie | Count: Ip
    def test_ban_action_event_cookie_subaction_response_count_by_ip(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_res_ip": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target,
            "test_ban_cookie_sub_response_count_by_ip",
            "response_body_ip",
            "503",
            4,
            3,
            params,
        )

    #  Action: Redirect | Subaction: Response | Event: Cookie | Count: Uri
    def test_ban_action_event_cookie_subaction_response_count_by_uri(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_res_uri": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target,
            "test_ban_cookie_sub_response_count_by_uri",
            "response_body_uri",
            "503",
            4,
            3,
            params,
        )

    #  Action: Redirect | Subaction: Response | Event: Cookie | Count: Query
    def test_ban_action_event_cookie_subaction_response_count_by_query(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_res_query": f"val-{i}"}}
            for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target,
            "test_ban_cookie_sub_response_count_by_query",
            "response_body_query",
            "503",
            4,
            3,
            params,
        )

    #  Action: Redirect | Subaction: Response | Event: Cookie | Count: Path
    def test_ban_action_event_cookie_subaction_response_count_by_path(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_res_path": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target,
            "test_ban_cookie_sub_response_count_by_path",
            "response_body_path",
            "503",
            4,
            3,
            params,
        )

    #  Action: Redirect | Subaction: Response | Event: Cookie | Count: Asn
    def test_ban_action_event_cookie_subaction_response_count_by_asn(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_res_asn": f"val-{i}"}} for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target,
            "test_ban_cookie_sub_response_count_by_asn",
            "response_body_asn",
            "503",
            4,
            3,
            params,
        )

    #  Action: Redirect | Subaction: Response | Event: Cookie | Count: Method
    def test_ban_action_event_cookie_subaction_response_count_by_method(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_res_method": f"val-{i}"}}
            for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target,
            "test_ban_cookie_sub_response_count_by_method",
            "response_body_method",
            "503",
            4,
            3,
            params,
        )

    #  Action: Redirect | Subaction: Response | Event: Cookie | Count: Company
    def test_ban_action_event_cookie_subaction_response_count_by_company(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_res_company": f"val-{i}"}}
            for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target,
            "test_ban_cookie_sub_response_count_by_company",
            "response_body_company",
            "503",
            4,
            3,
            params,
        )

    #  Action: Redirect | Subaction: Response | Event: Cookie | Count: Country
    def test_ban_action_event_cookie_subaction_response_count_by_country(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_res_country": f"val-{i}"}}
            for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target,
            "test_ban_cookie_sub_response_count_by_country",
            "response_body_country",
            "503",
            4,
            3,
            params,
        )

    #  Action: Redirect | Subaction: Response | Event: Cookie | Count: Authority
    def test_ban_action_event_cookie_subaction_response_count_by_authority(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_res_authority": f"val-{i}"}}
            for i in range(3 + 2)
        ]
        RateLimitHelper.check_rl_response_action_with_params(
            target,
            "test_ban_cookie_sub_response_count_by_authority",
            "response_body_authority",
            "503",
            4,
            3,
            params,
        )

    #  Action: Redirect | Subaction: Redirect | Event: Cookie | Count: Ip
    def test_ban_action_event_cookie_subaction_redirect_count_by_ip(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_red_ip": f"val-{i}"}} for i in range(4 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target,
            "test_ban_cookie_sub_redirect_count_by_ip",
            "200",
            4,
            4,
            "https://google.com",
            params,
        )

    #  Action: Redirect | Subaction: Redirect | Event: Cookie | Count: Asn
    def test_ban_action_event_cookie_subaction_redirect_count_by_asn(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_red_asn": f"val-{i}"}} for i in range(4 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target,
            "test_ban_cookie_sub_redirect_count_by_asn",
            "200",
            4,
            4,
            "https://google.com",
            params,
        )

    #  Action: Redirect | Subaction: Redirect | Event: Cookie | Count: Country
    def test_ban_action_event_cookie_subaction_redirect_count_by_country(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_red_country": f"val-{i}"}}
            for i in range(4 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target,
            "test_ban_cookie_sub_redirect_count_by_country",
            "200",
            4,
            4,
            "https://google.com",
            params,
        )

    #  Action: Redirect | Subaction: Redirect | Event: Cookie | Count: Uri
    def test_ban_action_event_cookie_subaction_redirect_count_by_uri(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_red_uri": f"val-{i}"}} for i in range(4 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target,
            "test_ban_cookie_sub_redirect_count_by_uri",
            "200",
            4,
            4,
            "https://google.com",
            params,
        )

    #  Action: Redirect | Subaction: Redirect | Event: Cookie | Count: Query
    def test_ban_action_event_cookie_subaction_redirect_count_by_query(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_red_query": f"val-{i}"}}
            for i in range(4 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target,
            "test_ban_cookie_sub_redirect_count_by_query",
            "200",
            4,
            4,
            "https://google.com",
            params,
        )

    #  Action: Redirect | Subaction: Redirect | Event: Cookie | Count: Path
    def test_ban_action_event_cookie_subaction_redirect_count_by_path(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_red_path": f"val-{i}"}} for i in range(4 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target,
            "test_ban_cookie_sub_redirect_count_by_path",
            "200",
            4,
            4,
            "https://google.com",
            params,
        )

    #  Action: Redirect | Subaction: Redirect | Event: Cookie | Count: Method
    def test_ban_action_event_cookie_subaction_redirect_count_by_method(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_red_method": f"val-{i}"}}
            for i in range(4 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target,
            "test_ban_cookie_sub_redirect_count_by_method",
            "200",
            4,
            4,
            "https://google.com",
            params,
        )

    #  Action: Redirect | Subaction: Redirect | Event: Cookie | Count: Company
    def test_ban_action_event_cookie_subaction_redirect_count_by_company(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_red_company": f"val-{i}"}}
            for i in range(4 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target,
            "test_ban_cookie_sub_redirect_count_by_company",
            "200",
            4,
            4,
            "https://google.com",
            params,
        )

    #  Action: Redirect | Subaction: Redirect | Event: Cookie | Count: Authority
    def test_ban_action_event_cookie_subaction_redirect_count_by_authority(
        self, cli, target, log_fixture
    ):
        params = [
            {"cookies": {"cookie_event_ban_red_authority": f"val-{i}"}}
            for i in range(4 + 2)
        ]
        RateLimitHelper.check_rl_redirect_action_with_params(
            target,
            "test_ban_cookie_sub_redirect_count_by_authority",
            "200",
            4,
            4,
            "https://google.com",
            params,
        )
