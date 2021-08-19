import pytest
from e2e.core.base_helper import cli, target, section, log_fixture
from e2e.helpers.rl_helper import RateLimitHelper
from e2e.helpers.rl_header_helper import ratelimit_header_config


@pytest.mark.usefixtures('api_setup', 'ratelimit_header_config')
@pytest.mark.all_modules
@pytest.mark.rate_limit_tests
@pytest.mark.rate_limit_header_tests
class TestRateLimitHeader:

    #  Action: 503  | Event: Header | Count: Ip
    def test_503_action_event_header_count_by_ip(self, target):
        params = [{"headers": {"header_event_503_ip": f"val-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_header_count_by_ip", 3,
                                                                 8, params)

    #  Action: 503  | Event: Header | Count: Path
    def test_503_action_event_header_count_by_path(self, target):
        params = [{"headers": {"header_event_503_path": f"val-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_header_count_by_path",
                                                                 3, 8, params)

    #  Action: 503  | Event: Header | Count: Uri
    def test_503_action_event_header_count_by_uri(self, target):
        params = [{"headers": {"header_event_503_uri": f"val-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_header_count_by_uri", 3,
                                                                 8, params)

    #  Action: 503  | Event: Header | Count: Asn
    def test_503_action_event_header_count_by_asn(self, target):
        params = [{"headers": {"header_event_503_asn": f"val-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_header_count_by_asn", 3,
                                                                 8, params)

    #  Action: 503  | Event: Header | Count: Query
    def test_503_action_event_header_count_by_query(self, target):
        params = [{"headers": {"header_event_503_query": f"val-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_header_count_by_query",
                                                                 3, 8, params)

    #  Action: 503  | Event: Header | Count: Method
    def test_503_action_event_header_count_by_method(self, target):
        params = [{"headers": {"header_event_503_method": f"val-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_header_count_by_method",
                                                                 3, 8, params)

    #  Action: 503  | Event: Header | Count: Company
    def test_503_action_event_header_count_by_company(self, target):
        params = [{"headers": {"header_event_503_company": f"val-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_header_count_by_company",
                                                                 3, 8, params)

    #  Action: 503  | Event: Header | Count: Country
    def test_503_action_event_header_count_by_country(self, target):
        params = [{"headers": {"header_event_503_country": f"val-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_header_count_by_country",
                                                                 3, 8, params)

    #  Action: 503  | Event: Header | Count: Authority
    def test_503_action_event_header_count_by_authority(self, target):
        params = [{"headers": {"header_event_503_authority": f"val-{i}"}} for i in range(8 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_header_count_by_authority",
                                                                 3, 8, params)

    #  Action: Tag only  | Event: Header | Count: Ip
    def test_tag_only_action_event_header_count_by_ip(self, target, log_fixture):
        params = [{"headers": {"header_event_tag_ip": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_header_ip", "tags",
                                                                     "test-tag-header-ip", 1, params)

    #  Action: Tag only  | Event: Header | Count: Uri
    def test_tag_only_action_event_header_count_by_uri(self, target, log_fixture):
        params = [{"headers": {"header_event_tag_uri": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_header_uri",
                                                                     "tags", "test-tag-header-uri", 1, params)

    #  Action: Tag only  | Event: Header | Count: Asn
    def test_tag_only_action_event_header_count_by_asn(self, target, log_fixture):
        params = [{"headers": {"header_event_tag_asn": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_header_asn",
                                                                     "tags", "test-tag-header-asn", 1, params)

    #  Action: Tag only  | Event: Header | Count: Path
    def test_tag_only_action_event_header_count_by_path(self, target, log_fixture):
        params = [{"headers": {"header_event_tag_path": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_header_path",
                                                                     "tags", "test-tag-header-path", 1, params)

    #  Action: Tag only  | Event: Header | Count: Query
    def test_tag_only_action_event_header_count_by_query(self, target, log_fixture):
        params = [{"headers": {"header_event_tag_query": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_header_query",
                                                                     "tags", "test-tag-header-query", 1, params)

    #  Action: Tag only  | Event: Header | Count: Method
    def test_tag_only_action_event_header_count_by_method(self, target, log_fixture):
        params = [{"headers": {"header_event_tag_method": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_header_method",
                                                                     "tags", "test-tag-header-method", 1, params)

    #  Action: Tag only  | Event: Header | Count: Company
    def test_tag_only_action_event_header_count_by_company(self, target, log_fixture):
        params = [{"headers": {"header_event_tag_company": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_header_company",
                                                                     "tags", "test-tag-header-company", 1, params)

    #  Action: Tag only  | Event: Header | Count: Country
    def test_tag_only_action_event_header_count_by_country(self, target, log_fixture):
        params = [{"headers": {"header_event_tag_country": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_header_country",
                                                                     "tags", "test-tag-header-country", 1, params)

    #  Action: Tag only  | Event: Header | Count: Authority
    def test_tag_only_action_event_header_count_by_authority(self, target, log_fixture):
        params = [{"headers": {"header_event_tag_authority": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_header_authority",
                                                                     "tags", "test-tag-header-authority", 1, params)

    #  Action: Response  | Event: Header | Count: Ip
    def test_response_action_event_header_count_by_ip(self, cli, target):
        params = [{"headers": {"header_event_res_ip": f"val-{i}"}} for i in range(15 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_header_ip",
                                                             "response_body_ip", "302", 9, 15, params)

    #  Action: Response  | Event: Header | Count: Uri
    def test_response_action_event_header_count_by_uri(self, cli, target):
        params = [{"headers": {"header_event_res_uri": f"val-{i}"}} for i in range(15 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_header_uri",
                                                             "response_body_uri", "503", 9, 15, params)

    #  Action: Response  | Event: Header | Count: Query
    def test_response_action_event_header_count_by_query(self, cli, target):
        params = [{"headers": {"header_event_res_query": f"val-{i}"}} for i in range(15 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_header_query",
                                                             "response_body_query", "302", 9, 15, params)

    #  Action: Response  | Event: Header | Count: Path
    def test_response_action_event_header_count_by_path(self, cli, target):
        params = [{"headers": {"header_event_res_path": f"val-{i}"}} for i in range(15 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_header_path",
                                                             "response_body_path", "302", 9, 15, params)

    #  Action: Response  | Event: Header | Count: Asn
    def test_response_action_event_header_count_by_asn(self, cli, target):
        params = [{"headers": {"header_event_res_asn": f"val-{i}"}} for i in range(15 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_header_asn",
                                                             "response_body_asn", "302", 9, 15, params)

    #  Action: Response  | Event: Header | Count: Method
    def test_response_action_event_header_count_by_method(self, cli, target):
        params = [{"headers": {"header_event_res_method": f"val-{i}"}} for i in range(15 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_header_method",
                                                             "response_body_method", "302", 9, 15, params)

    #  Action: Response  | Event: Header | Count: Company
    def test_response_action_event_header_count_by_company(self, cli, target):
        params = [{"headers": {"header_event_res_company": f"val-{i}"}} for i in range(15 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_header_company",
                                                             "response_body_company", "302", 9, 15, params)

    #  Action: Response  | Event: Header | Count: Country
    def test_response_action_event_header_count_by_country(self, cli, target):
        params = [{"headers": {"header_event_res_country": f"val-{i}"}} for i in range(15 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_header_country",
                                                             "response_body_country", "302", 9, 15, params)

    #  Action: Response  | Event: Header | Count: Authority
    def test_response_action_event_header_count_by_authority(self, cli, target):
        params = [{"headers": {"header_event_res_authority": f"val-{i}"}} for i in range(15 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_header_authority",
                                                             "response_body_authority", "302", 9, 15, params)

    #  Action: Challenge  | Event: Header | Count: Ip
    def test_challenge_action_event_header_count_by_ip(self, cli, target):
        params = [{"headers": {"header_event_chl_ip": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_header_ip", 4, 5,
                                                                      params)

    #  Action: Challenge  | Event: Header | Count: Uri
    def test_challenge_action_event_header_count_by_uri(self, cli, target):
        params = [{"headers": {"header_event_chl_uri": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_header_uri", 4, 5,
                                                                      params)

    #  Action: Challenge  | Event: Header | Count: Query
    def test_challenge_action_event_header_count_by_query(self, cli, target):
        params = [{"headers": {"header_event_chl_query": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_header_query", 4, 5,
                                                                      params)

    #  Action: Challenge  | Event: Header | Count: Path
    def test_challenge_action_event_header_count_by_path(self, cli, target):
        params = [{"headers": {"header_event_chl_path": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_header_path", 4, 5,
                                                                      params)

    #  Action: Challenge  | Event: Header | Count: Asn
    def test_challenge_action_event_header_count_by_asn(self, cli, target):
        params = [{"headers": {"header_event_chl_asn": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_header_asn", 4, 5,
                                                                      params)

    #  Action: Challenge  | Event: Header | Count: Method
    def test_challenge_action_event_header_count_by_method(self, cli, target):
        params = [{"headers": {"header_event_chl_method": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_header_method", 4, 5,
                                                                      params)

    #  Action: Challenge  | Event: Header | Count: Company
    def test_challenge_action_event_header_count_by_company(self, cli, target):
        params = [{"headers": {"header_event_chl_company": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_header_company", 4, 5,
                                                                      params)

    #  Action: Challenge  | Event: Header | Count: Country
    def test_challenge_action_event_header_count_by_country(self, cli, target):
        params = [{"headers": {"header_event_chl_country": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_header_country", 4, 5,
                                                                      params)

    #  Action: Challenge  | Event: Header | Count: Authority
    def test_challenge_action_event_header_count_by_authority(self, cli, target):
        params = [{"headers": {"header_event_chl_authority": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_header_authority", 4, 5,
                                                                      params)

    #  Action: Redirect  | Event: Header | Count: Ip
    def test_redirect_action_event_header_count_by_ip(self, cli, target):
        params = [{"headers": {"header_event_red_ip": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_header_ip", "200", 3, 3,
                                                             "https://yahoo.com", params)

    #  Action: Redirect  | Event: Header | Count: Uri
    def test_redirect_action_event_header_count_by_uri(self, cli, target):
        params = [{"headers": {"header_event_red_uri": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_header_uri", "200", 3, 3,
                                                             "https://yahoo.com", params)

    #  Action: Redirect  | Event: Header | Count: Query
    def test_redirect_action_event_header_count_by_query(self, cli, target):
        params = [{"headers": {"header_event_red_query": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_header_query", "200", 3, 3,
                                                             "https://yahoo.com", params)

    #  Action: Redirect  | Event: Header | Count: Path
    def test_redirect_action_event_header_count_by_path(self, cli, target):
        params = [{"headers": {"header_event_red_path": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_header_path", "200", 3, 3,
                                                             "https://yahoo.com", params)

    #  Action: Redirect  | Event: Header | Count: Asn
    def test_redirect_action_event_header_count_by_asn(self, cli, target):
        params = [{"headers": {"header_event_red_asn": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_header_asn", "200", 3, 3,
                                                             "https://yahoo.com", params)

    #  Action: Redirect  | Event: Header | Count: Method
    def test_redirect_action_event_header_count_by_method(self, cli, target):
        params = [{"headers": {"header_event_red_method": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_header_method", "200", 3, 3,
                                                             "https://yahoo.com", params)

    #  Action: Redirect  | Event: Header | Count: Company
    def test_redirect_action_event_header_count_by_company(self, cli, target):
        params = [{"headers": {"header_event_red_company": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_header_company", "200", 3,
                                                             3, "https://yahoo.com", params)

    #  Action: Redirect  | Event: Header | Count: Country
    def test_redirect_action_event_header_count_by_country(self, cli, target):
        params = [{"headers": {"header_event_red_country": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_header_country", "200", 3,
                                                             3, "https://yahoo.com", params)

    #  Action: Redirect  | Event: Header | Count: Authority
    def test_redirect_action_event_header_count_by_authority(self, cli, target):
        params = [{"headers": {"header_event_red_authority": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_header_authority", "200", 3,
                                                             3, "https://yahoo.com", params)

    #  Action: Ban | Subaction: 503 | Event: Header | Count: Ip
    def test_ban_action_event_header_subaction_503_count_by_ip(self, cli, target):
        params = [{"headers": {"header_event_ban_503_ip": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_header_sub_503_count_by_ip", 8, 10,
                                                                 params)

    #  Action: Ban | Subaction: 503 | Event: Header | Count: Uri
    def test_ban_action_event_header_subaction_503_count_by_uri(self, cli, target):
        params = [{"headers": {"header_event_ban_503_uri": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_header_sub_503_count_by_uri", 8, 10,
                                                                 params)

    #  Action: Ban | Subaction: 503 | Event: Header | Count: Query
    def test_ban_action_event_header_subaction_503_count_by_query(self, cli, target):
        params = [{"headers": {"header_event_ban_503_query": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_header_sub_503_count_by_query", 8,
                                                                 10, params)

    #  Action: Ban | Subaction: 503 | Event: Header | Count: Path
    def test_ban_action_event_header_subaction_503_count_by_path(self, cli, target):
        params = [{"headers": {"header_event_ban_503_path": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_header_sub_503_count_by_path", 8,
                                                                 10, params)

    #  Action: Ban | Subaction: 503 | Event: Header | Count: Asn
    def test_ban_action_event_header_subaction_503_count_by_asn(self, cli, target):
        params = [{"headers": {"header_event_ban_503_asn": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_header_sub_503_count_by_asn", 8,
                                                                 10, params)

    #  Action: Ban | Subaction: 503 | Event: Header | Count: Method
    def test_ban_action_event_header_subaction_503_count_by_method(self, cli, target):
        params = [{"headers": {"header_event_ban_503_method": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_header_sub_503_count_by_method", 8,
                                                                 10, params)

    #  Action: Ban | Subaction: 503 | Event: Header | Count: Company
    def test_ban_action_event_header_subaction_503_count_by_company(self, cli, target):
        params = [{"headers": {"header_event_ban_503_company": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_header_sub_503_count_by_company", 8,
                                                                 10, params)

    #  Action: Ban | Subaction: 503 | Event: Header | Count: Country
    def test_ban_action_event_header_subaction_503_count_by_country(self, cli, target):
        params = [{"headers": {"header_event_ban_503_country": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_header_sub_503_count_by_country", 8,
                                                                 10, params)

    #  Action: Ban | Subaction: 503 | Event: Header | Count: Authority
    def test_ban_action_event_header_subaction_503_count_by_authority(self, cli, target):
        params = [{"headers": {"header_event_ban_503_authority": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_header_sub_503_count_by_authority",
                                                                 8, 10, params)

    #  Action: Ban | Subaction: Challenge | Event: Header | Count: Ip
    def test_ban_action_event_header_subaction_challenge_count_by_ip(self, cli, target):
        params = [{"headers": {"header_event_ban_chl_ip": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_header_sub_chl_count_by_ip", 6, 5,
                                                                      params)

    #  Action: Ban | Subaction: Challenge | Event: Header | Count: Uri
    def test_ban_action_event_header_subaction_challenge_count_by_uri(self, cli, target):
        params = [{"headers": {"header_event_ban_chl_uri": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_header_sub_chl_count_by_uri", 6, 5,
                                                                      params)

    #  Action: Ban | Subaction: Challenge | Event: Header | Count: Query
    def test_ban_action_event_header_subaction_challenge_count_by_query(self, cli, target):
        params = [{"headers": {"header_event_ban_chl_query": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_header_sub_chl_count_by_query", 6, 5,
                                                                      params)

    #  Action: Ban | Subaction: Challenge | Event: Header | Count: Path
    def test_ban_action_event_header_subaction_challenge_count_by_path(self, cli, target):
        params = [{"headers": {"header_event_ban_chl_path": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_header_sub_chl_count_by_path", 6, 5,
                                                                      params)

    #  Action: Ban | Subaction: Challenge | Event: Header | Count: Asn
    def test_ban_action_event_header_subaction_challenge_count_by_asn(self, cli, target):
        params = [{"headers": {"header_event_ban_chl_asn": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_header_sub_chl_count_by_asn", 6, 5,
                                                                      params)

    #  Action: Ban | Subaction: Challenge | Event: Header | Count: Method
    def test_ban_action_event_header_subaction_challenge_count_by_method(self, cli, target):
        params = [{"headers": {"header_event_ban_chl_method": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_header_sub_chl_count_by_method", 6, 5,
                                                                      params)

    #  Action: Ban | Subaction: Challenge | Event: Header | Count: Company
    def test_ban_action_event_header_subaction_challenge_count_by_company(self, cli, target):
        params = [{"headers": {"header_event_ban_chl_company": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_header_sub_chl_count_by_company", 6, 5,
                                                                      params)

    #  Action: Ban | Subaction: Challenge | Event: Header | Count: Country
    def test_ban_action_event_header_subaction_challenge_count_by_country(self, cli, target):
        params = [{"headers": {"header_event_ban_chl_country": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_header_sub_chl_count_by_country", 6, 5,
                                                                      params)

    #  Action: Ban | Subaction: Challenge | Event: Header | Count: Authority
    def test_ban_action_event_header_subaction_challenge_count_by_authority(self, cli, target):
        params = [{"headers": {"header_event_ban_chl_authority": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_header_sub_chl_count_by_authority",
                                                                      6, 5, params)

    #  Action: Ban | Subaction: Tag only | Event: Header | Count: Ip
    def test_ban_action_event_header_subaction_tag_only_count_by_ip(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_tag_ip": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_header_sub_tag_count_by_ip",
                                                                     "tags", "test-ban-header-sub-tag-count-by-ip", 1,
                                                                     params)

    #  Action: Ban | Subaction: Tag only | Event: Header | Count: Uri
    def test_ban_action_event_header_subaction_tag_only_count_by_uri(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_tag_uri": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_header_sub_tag_count_by_uri",
                                                                     "tags", "test-ban-header-sub-tag-count-by-uri", 1,
                                                                     params)

    #  Action: Ban | Subaction: Tag only | Event: Header | Count: Query
    def test_ban_action_event_header_subaction_tag_only_count_by_query(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_tag_query": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_header_sub_tag_count_by_query",
                                                                     "tags", "test-ban-header-sub-tag-count-by-query",
                                                                     1, params)

    #  Action: Ban | Subaction: Tag only | Event: Header | Count: Path
    def test_ban_action_event_header_subaction_tag_only_count_by_path(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_tag_path": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_header_sub_tag_count_by_path",
                                                                     "tags", "test-ban-header-sub-tag-count-by-path",
                                                                     1, params)

    #  Action: Ban | Subaction: Tag only | Event: Header | Count: Asn
    def test_ban_action_event_header_subaction_tag_only_count_by_asn(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_tag_asn": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_header_sub_tag_count_by_asn", "tags",
                                                                     "test-ban-header-sub-tag-count-by-asn", 1,
                                                                     params)

    #  Action: Ban | Subaction: Tag only | Event: Header | Count: Method
    def test_ban_action_event_header_subaction_tag_only_count_by_method(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_tag_method": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_header_sub_tag_count_by_method", "tags",
                                                                     "test-ban-header-sub-tag-count-by-method", 1,
                                                                     params)

    #  Action: Ban | Subaction: Tag only | Event: Header | Count: Company
    def test_ban_action_event_header_subaction_tag_only_count_by_company(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_tag_company": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_header_sub_tag_count_by_company", "tags",
                                                                     "test-ban-header-sub-tag-count-by-company", 1,
                                                                     params)

    def test_ban_action_event_header_subaction_tag_only_count_by_country(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_tag_country": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_header_sub_tag_count_by_country",
                                                                     "tags", "test-ban-header-sub-tag-count-by-country",
                                                                     1, params)

    #  Action: Ban | Subaction: Tag only | Event: Header | Count: Authority
    def test_ban_action_event_header_subaction_tag_only_count_by_authority(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_tag_authority": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_header_sub_tag_count_by_authority",
                                                                     "tags",
                                                                     "test-ban-header-sub-tag-count-by-authority",
                                                                     1, params)

    #  Action: Ban | Subaction: Response | Event: Header | Count: Ip
    def test_ban_action_event_header_subaction_response_count_by_ip(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_res_ip": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_header_sub_response_count_by_ip",
                                                             "response_body_ip", "503", 4, 3, params)

    #  Action: Ban | Subaction: Response | Event: Header | Count: Uri
    def test_ban_action_event_header_subaction_response_count_by_uri(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_res_uri": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_header_sub_response_count_by_uri",
                                                             "response_body_uri", "503", 4, 3, params)

    #  Action: Ban | Subaction: Response | Event: Header | Count: Query
    def test_ban_action_event_header_subaction_response_count_by_query(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_res_query": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_header_sub_response_count_by_query",
                                                             "response_body_query", "503", 4, 3, params)

    #  Action: Ban | Subaction: Response | Event: Header | Count: Path
    def test_ban_action_event_header_subaction_response_count_by_path(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_res_path": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_header_sub_response_count_by_path",
                                                             "response_body_path", "503", 4, 3, params)

    #  Action: Ban | Subaction: Response | Event: Header | Count: Asn
    def test_ban_action_event_header_subaction_response_count_by_asn(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_res_asn": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_header_sub_response_count_by_asn",
                                                             "response_body_asn", "503", 4, 3, params)

    #  Action: Ban | Subaction: Response | Event: Header | Count: Method
    def test_ban_action_event_header_subaction_response_count_by_method(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_res_method": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_header_sub_response_count_by_method",
                                                             "response_body_method", "503", 4, 3, params)

    #  Action: Ban | Subaction: Response | Event: Header | Count: Company
    def test_ban_action_event_header_subaction_response_count_by_company(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_res_company": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_header_sub_response_count_by_company",
                                                             "response_body_company", "503", 4, 3, params)

    #  Action: Ban | Subaction: Response | Event: Header | Count: Country
    def test_ban_action_event_header_subaction_response_count_by_country(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_res_country": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_header_sub_response_count_by_country",
                                                             "response_body_country", "503", 4, 3, params)

    #  Action: Ban | Subaction: Response | Event: Header | Count: Authority
    def test_ban_action_event_header_subaction_response_count_by_authority(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_res_authority": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_header_sub_response_count_by_authority",
                                                             "response_body_authority", "503", 4, 3, params)

    #  Action: Ban | Subaction: Redirect | Event: Header | Count: Ip
    def test_ban_action_event_header_subaction_redirect_count_by_ip(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_red_ip": f"val-{i}"}} for i in range(20 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_header_sub_redirect_count_by_ip",
                                                             "200", 11, 20, "https://google.com", params)

    #  Action: Ban | Subaction: Redirect | Event: Header | Count: Asn
    def test_ban_action_event_header_subaction_redirect_count_by_asn(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_red_asn": f"val-{i}"}} for i in range(20 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_header_sub_redirect_count_by_asn",
                                                             "200", 11, 20, "https://google.com", params)

    #  Action: Ban | Subaction: Redirect | Event: Header | Count: Country
    def test_ban_action_event_header_subaction_redirect_count_by_country(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_red_country": f"val-{i}"}} for i in range(20 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_header_sub_redirect_count_by_country",
                                                             "200", 11, 20, "https://google.com", params)

    #  Action: Ban | Subaction: Redirect | Event: Header | Count: Uri
    def test_ban_action_event_header_subaction_redirect_count_by_uri(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_red_uri": f"val-{i}"}} for i in range(20 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_header_sub_redirect_count_by_uri",
                                                             "200", 11, 20, "https://google.com", params)

    #  Action: Ban | Subaction: Redirect | Event: Header | Count: Path
    def test_ban_action_event_header_subaction_redirect_count_by_path(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_red_asn": f"val-{i}"}} for i in range(20 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_header_sub_redirect_count_by_path",
                                                             "200", 11, 20, "https://google.com", params)

    #  Action: Ban | Subaction: Redirect | Event: Header | Count: Query
    def test_ban_action_event_header_subaction_redirect_count_by_query(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_red_country": f"val-{i}"}} for i in range(20 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_header_sub_redirect_count_by_query",
                                                             "200", 11, 20, "https://google.com", params)

    #  Action: Ban | Subaction: Redirect | Event: Header | Count: Method
    def test_ban_action_event_header_subaction_redirect_count_by_method(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_red_method": f"val-{i}"}} for i in range(20 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_header_sub_redirect_count_by_method",
                                                             "200", 11, 20, "https://google.com", params)

    #  Action: Ban | Subaction: Redirect | Event: Header | Count: Company
    def test_ban_action_event_header_subaction_redirect_count_by_company(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_red_company": f"val-{i}"}} for i in range(20 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_header_sub_redirect_count_by_company",
                                                             "200", 11, 20, "https://google.com", params)

    #  Action: Ban | Subaction: Redirect | Event: Header | Count: Authority
    def test_ban_action_event_header_subaction_redirect_count_by_authority(self, cli, target, log_fixture):
        params = [{"headers": {"header_event_ban_red_authority": f"val-{i}"}} for i in range(20 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_header_sub_redirect_count_by_authority",
                                                             "200", 11, 20, "https://google.com", params)

