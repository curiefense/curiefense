import pytest
from e2e.core.base_helper import cli, target, section, log_fixture
from e2e.helpers.rl_helper import RateLimitHelper
from e2e.helpers.rl_argument_helper import ratelimit_argument_config


@pytest.mark.usefixtures('api_setup', 'ratelimit_argument_config')
@pytest.mark.all_modules
@pytest.mark.rate_limit_tests
@pytest.mark.rate_limit_argument_tests
class TestRateLimitArgument:

    #  Action: 503  | Event: Argument | Count: Ip
    def test_503_action_event_argument_count_by_ip(self, target):
        params = [{"params": {"arg_event_503_ip": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_argument_count_by_ip", 2,
                                                                 3, params)

    #  Action: 503  | Event: Argument | Count: Path
    def test_503_action_event_argument_count_by_path(self, target):
        params = [{"params": {"arg_event_503_path": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_argument_count_by_path",
                                                                 2, 3, params)

    #  Action: 503  | Event: Argument | Count: Asn
    def test_503_action_event_argument_count_by_asn(self, target):
        params = [{"params": {"arg_event_503_asn": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_argument_count_by_asn", 2,
                                                                 3, params)

    #  Action: 503  | Event: Argument | Count: Method
    def test_503_action_event_argument_count_by_method(self, target):
        params = [{"params": {"arg_event_503_method": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_argument_count_by_method",
                                                                 2, 3, params)

    #  Action: 503  | Event: Argument | Count: Company
    def test_503_action_event_argument_count_by_company(self, target):
        params = [{"params": {"arg_event_503_company": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_argument_count_by_company",
                                                                 2, 3, params)

    #  Action: 503  | Event: Argument | Count: Country
    def test_503_action_event_argument_count_by_country(self, target):
        params = [{"params": {"arg_event_503_country": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_argument_count_by_country",
                                                                 2, 3, params)

    #  Action: 503  | Event: Argument | Count: Authority
    def test_503_action_event_argument_count_by_authority(self, target):
        params = [{"params": {"arg_event_503_authority": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_503_action_argument_count_by_authority",
                                                                 2, 3, params)

    #  Action: Tag only  | Event: Argument | Count: Ip
    def test_tag_only_action_event_argument_count_by_ip(self, target, log_fixture):
        params = [{"params": {"arg_event_tag_ip": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_argument_ip", "tags",
                                                                     "test-tag-argument-ip", 1, params)

    #  Action: Tag only  | Event: Argument | Count: Asn
    def test_tag_only_action_event_argument_count_by_asn(self, target, log_fixture):
        params = [{"params": {"arg_event_tag_asn": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_argument_asn",
                                                                     "tags", "test-tag-argument-asn", 1, params)

    #  Action: Tag only  | Event: Argument | Count: Path
    def test_tag_only_action_event_argument_count_by_path(self, target, log_fixture):
        params = [{"params": {"arg_event_tag_path": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_argument_path",
                                                                     "tags", "test-tag-argument-path", 1, params)

    #  Action: Tag only  | Event: Argument | Count: Method
    def test_tag_only_action_event_argument_count_by_method(self, target, log_fixture):
        params = [{"params": {"arg_event_tag_method": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_argument_method", "tags",
                                                                     "test-tag-argument-method", 1, params)

    #  Action: Tag only  | Event: Argument | Count: Company
    def test_tag_only_action_event_argument_count_by_company(self, target, log_fixture):
        params = [{"params": {"arg_event_tag_company": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_argument_company", "tags",
                                                                     "test-tag-argument-company", 1, params)

    #  Action: Tag only  | Event: Argument | Count: Country
    def test_tag_only_action_event_argument_count_by_country(self, target, log_fixture):
        params = [{"params": {"arg_event_tag_country": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_argument_country", "tags",
                                                                     "test-tag-argument-country", 1, params)

    #  Action: Tag only  | Event: Argument | Count: Authority
    def test_tag_only_action_event_argument_count_by_authority(self, target, log_fixture):
        params = [{"params": {"arg_event_tag_authority": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_tag_argument_authority", "tags",
                                                                     "test-tag-argument-authority", 1, params)

    #  Action: Response  | Event: Argument | Count: Ip
    def test_response_action_event_argument_count_by_ip(self, cli, target):
        params = [{"params": {"arg_event_res_ip": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_argument_ip",
                                                             "response_body_ip", "302", 3, 5, params)

    #  Action: Response  | Event: Argument | Count: Path
    def test_response_action_event_argument_count_by_path(self, cli, target):
        params = [{"params": {"arg_event_res_path": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_argument_path",
                                                             "response_body_path", "302", 3, 5, params)

    #  Action: Response  | Event: Argument | Count: Asn
    def test_response_action_event_argument_count_by_asn(self, cli, target):
        params = [{"params": {"arg_event_res_asn": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_argument_asn",
                                                             "response_body_asn", "302", 3, 5, params)

    #  Action: Response  | Event: Argument | Count: Method
    def test_response_action_event_argument_count_by_method(self, cli, target):
        params = [{"params": {"arg_event_res_method": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_argument_method",
                                                             "response_body_method", "302", 3, 5, params)

    #  Action: Response  | Event: Argument | Count: Company
    def test_response_action_event_argument_count_by_company(self, cli, target):
        params = [{"params": {"arg_event_res_company": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_argument_company",
                                                             "response_body_company", "302", 3, 5, params)

    #  Action: Response  | Event: Argument | Count: Country
    def test_response_action_event_argument_count_by_country(self, cli, target):
        params = [{"params": {"arg_event_res_country": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_argument_country",
                                                             "response_body_country", "302", 3, 5, params)

    #  Action: Response  | Event: Argument | Count: Authority
    def test_response_action_event_argument_count_by_authority(self, cli, target):
        params = [{"params": {"arg_event_res_authority": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_res_argument_authority",
                                                             "response_body_authority", "302", 3, 5, params)

    #  Action: Challenge  | Event: Argument | Count: Ip
    def test_challenge_action_event_argument_count_by_ip(self, cli, target):
        params = [{"params": {"arg_event_chl_ip": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_argument_ip", 4, 5,
                                                                      params)

    #  Action: Challenge  | Event: Argument | Count: Path
    def test_challenge_action_event_argument_count_by_path(self, cli, target):
        params = [{"params": {"arg_event_chl_path": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_argument_path", 4, 5,
                                                                      params)

    #  Action: Challenge  | Event: Argument | Count: Asn
    def test_challenge_action_event_argument_count_by_asn(self, cli, target):
        params = [{"params": {"arg_event_chl_asn": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_argument_asn", 4, 5,
                                                                      params)

    #  Action: Challenge  | Event: Argument | Count: Method
    def test_challenge_action_event_argument_count_by_method(self, cli, target):
        params = [{"params": {"arg_event_chl_method": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_argument_method", 4, 5,
                                                                      params)

    #  Action: Challenge  | Event: Argument | Count: Company
    def test_challenge_action_event_argument_count_by_company(self, cli, target):
        params = [{"params": {"arg_event_chl_company": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_argument_company", 4, 5,
                                                                      params)

    #  Action: Challenge  | Event: Argument | Count: Country
    def test_challenge_action_event_argument_count_by_country(self, cli, target):
        params = [{"params": {"arg_event_chl_country": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_argument_country", 4, 5,
                                                                      params)

    #  Action: Challenge  | Event: Argument | Count: Authority
    def test_challenge_action_event_argument_count_by_authority(self, cli, target):
        params = [{"params": {"arg_event_chl_authority": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target, "test_challenge_argument_authority", 4, 5,
                                                                      params)

    #  Action: Redirect  | Event: Argument | Count: Ip
    def test_redirect_action_event_argument_count_by_ip(self, cli, target):
        params = [{"params": {"arg_event_red_ip": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_argument_ip", "200", 3, 3,
                                                             "https://yahoo.com", params)

    #  Action: Redirect  | Event: Argument | Count: Path
    def test_redirect_action_event_argument_count_by_path(self, cli, target):
        params = [{"params": {"arg_event_red_path": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_argument_path", "200", 3, 3,
                                                             "https://yahoo.com", params)

    #  Action: Redirect  | Event: Argument | Count: Asn
    def test_redirect_action_event_argument_count_by_asn(self, cli, target):
        params = [{"params": {"arg_event_red_asn": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_argument_asn", "200", 3, 3,
                                                             "https://yahoo.com", params)

    #  Action: Redirect  | Event: Argument | Count: Method
    def test_redirect_action_event_argument_count_by_method(self, cli, target):
        params = [{"params": {"arg_event_red_method": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_argument_method", "200", 3, 3,
                                                             "https://yahoo.com", params)

    #  Action: Redirect  | Event: Argument | Count: Company
    def test_redirect_action_event_argument_count_by_company(self, cli, target):
        params = [{"params": {"arg_event_red_company": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_argument_company", "200", 3,
                                                             3, "https://yahoo.com", params)

    #  Action: Redirect  | Event: Argument | Count: Country
    def test_redirect_action_event_argument_count_by_country(self, cli, target):
        params = [{"params": {"arg_event_red_country": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_argument_country", "200", 3,
                                                             3, "https://yahoo.com", params)

    #  Action: Redirect  | Event: Argument | Count: Authority
    def test_redirect_action_event_argument_count_by_authority(self, cli, target):
        params = [{"params": {"arg_event_red_authority": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_redirect_argument_authority", "200", 3,
                                                             3, "https://yahoo.com", params)

    #  Action: Ban | Subaction: 503 | Event: Argument | Count: Ip
    def test_ban_action_event_argument_subaction_503_count_by_ip(self, cli, target):
        params = [{"params": {"arg_event_ban_503_ip": f"val-{i}"}} for i in range(4 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_argument_sub_503_count_by_ip", 4, 4,
                                                                 params)

    #  Action: Ban | Subaction: 503 | Event: Argument | Count: Path
    def test_ban_action_event_argument_subaction_503_count_by_path(self, cli, target):
        params = [{"params": {"arg_event_ban_503_path": f"val-{i}"}} for i in range(4 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_argument_sub_503_count_by_path", 4,
                                                                 4, params)

    #  Action: Ban | Subaction: 503 | Event: Argument | Count: Asn
    def test_ban_action_event_argument_subaction_503_count_by_asn(self, cli, target):
        params = [{"params": {"arg_event_ban_503_asn": f"val-{i}"}} for i in range(4 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_argument_sub_503_count_by_asn", 4,
                                                                 4, params)

    #  Action: Ban | Subaction: 503 | Event: Argument | Count: Method
    def test_ban_action_event_argument_subaction_503_count_by_method(self, cli, target):
        params = [{"params": {"arg_event_ban_503_method": f"val-{i}"}} for i in range(4 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_argument_sub_503_count_by_method", 4,
                                                                 4, params)

    #  Action: Ban | Subaction: 503 | Event: Argument | Count: Company
    def test_ban_action_event_argument_subaction_503_count_by_company(self, cli, target):
        params = [{"params": {"arg_event_ban_503_company": f"val-{i}"}} for i in range(4 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_argument_sub_503_count_by_company",
                                                                 4, 4, params)

    #  Action: Ban | Subaction: 503 | Event: Argument | Count: Country
    def test_ban_action_event_argument_subaction_503_count_by_country(self, cli, target):
        params = [{"params": {"arg_event_ban_503_country": f"val-{i}"}} for i in range(4 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_argument_sub_503_count_by_country",
                                                                 4, 4, params)

    #  Action: Ban | Subaction: 503 | Event: Argument | Count: Authority
    def test_ban_action_event_argument_subaction_503_count_by_authority(self, cli, target):
        params = [{"params": {"arg_event_ban_503_authority": f"val-{i}"}} for i in range(4 + 2)]
        RateLimitHelper.check_rate_limits_action_503_with_params(target, "test_ban_argument_sub_503_count_by_authority",
                                                                 4, 4, params)

    #  Action: Ban | Subaction: Challenge | Event: Argument | Count: Ip
    def test_ban_action_event_argument_subaction_challenge_count_by_ip(self, cli, target):
        params = [{"params": {"arg_event_ban_chl_ip": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_argument_sub_chl_count_by_ip", 6, 5,
                                                                      params)

    #  Action: Ban | Subaction: Challenge | Event: Argument | Count: Path
    def test_ban_action_event_argument_subaction_challenge_count_by_path(self, cli, target):
        params = [{"params": {"arg_event_ban_chl_path": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_argument_sub_chl_count_by_path", 6, 5,
                                                                      params)

    #  Action: Ban | Subaction: Challenge | Event: Argument | Count: Asn
    def test_ban_action_event_argument_subaction_challenge_count_by_asn(self, cli, target):
        params = [{"params": {"arg_event_ban_chl_asn": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_argument_sub_chl_count_by_asn", 6, 5,
                                                                      params)

    #  Action: Ban | Subaction: Challenge | Event: Argument | Count: Method
    def test_ban_action_event_argument_subaction_challenge_count_by_method(self, cli, target):
        params = [{"params": {"arg_event_ban_chl_method": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_argument_sub_chl_count_by_method", 6, 5,
                                                                      params)

    #  Action: Ban | Subaction: Challenge | Event: Argument | Count: Company
    def test_ban_action_event_argument_subaction_challenge_count_by_company(self, cli, target):
        params = [{"params": {"arg_event_ban_chl_company": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_argument_sub_chl_count_by_company",
                                                                      6, 5, params)

    #  Action: Ban | Subaction: Challenge | Event: Argument | Count: Country
    def test_ban_action_event_argument_subaction_challenge_count_by_country(self, cli, target):
        params = [{"params": {"arg_event_ban_chl_country": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_argument_sub_chl_count_by_country", 6,
                                                                      5, params)

    #  Action: Ban | Subaction: Challenge | Event: Argument | Count: Authority
    def test_ban_action_event_argument_subaction_challenge_count_by_authority(self, cli, target):
        params = [{"params": {"arg_event_ban_chl_authority": f"val-{i}"}} for i in range(5 + 2)]
        RateLimitHelper.check_rate_limit_challenge_action_with_params(target,
                                                                      "test_ban_argument_sub_chl_count_by_authority",
                                                                      6, 5, params)

    #  Action: Ban | Subaction: Tag only | Event: Argument | Count: Ip
    def test_ban_action_event_argument_subaction_tag_only_count_by_ip(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_tag_ip": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_argument_sub_tag_count_by_ip",
                                                                     "tags", "test-ban-argument-sub-tag-count-by-ip", 1,
                                                                     params)

    #  Action: Ban | Subaction: Tag only | Event: Argument | Count: Path
    def test_ban_action_event_argument_subaction_tag_only_count_by_path(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_tag_path": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_argument_sub_tag_count_by_path",
                                                                     "tags", "test-ban-argument-sub-tag-count-by-path",
                                                                     1, params)

    #  Action: Ban | Subaction: Tag only | Event: Argument | Count: Asn
    def test_ban_action_event_argument_subaction_tag_only_count_by_asn(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_tag_asn": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_argument_sub_tag_count_by_asn", "tags",
                                                                     "test-ban-argument-sub-tag-count-by-asn", 1,
                                                                     params)

    #  Action: Ban | Subaction: Tag only | Event: Argument | Count: Method
    def test_ban_action_event_argument_subaction_tag_only_count_by_method(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_tag_method": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_argument_sub_tag_count_by_method",
                                                                     "tags",
                                                                     "test-ban-argument-sub-tag-count-by-method", 1,
                                                                     params)

    #  Action: Ban | Subaction: Tag only | Event: Argument | Count: Company
    def test_ban_action_event_argument_subaction_tag_only_count_by_company(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_tag_company": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_argument_sub_tag_count_by_company",
                                                                     "tags",
                                                                     "test-ban-argument-sub-tag-count-by-company", 1,
                                                                     params)

    #  Action: Ban | Subaction: Tag only | Event: Argument | Count: Country
    def test_ban_action_event_argument_subaction_tag_only_count_by_country(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_tag_country": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_argument_sub_tag_count_by_country",
                                                                     "tags",
                                                                     "test-ban-argument-sub-tag-count-by-country", 1,
                                                                     params)

    #  Action: Ban | Subaction: Tag only | Event: Argument | Count: Authority
    def test_ban_action_event_argument_subaction_tag_only_count_by_authority(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_tag_authority": f"val-{i}"}} for i in range(1 + 2)]
        RateLimitHelper.check_rl_action_tag_with_pattern_with_params(log_fixture, target,
                                                                     "test_ban_argument_sub_tag_count_by_authority",
                                                                     "tags",
                                                                     "test-ban-argument-sub-tag-count-by-authority",
                                                                     1, params)

    #  Action: Ban | Subaction: Response | Event: Argument | Count: Ip
    def test_ban_action_event_argument_subaction_response_count_by_ip(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_res_ip": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_argument_sub_response_count_by_ip",
                                                             "response_body_ip", "503", 4, 3, params)

    #  Action: Ban | Subaction: Response | Event: Argument | Count: Path
    def test_ban_action_event_argument_subaction_response_count_by_path(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_res_path": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_argument_sub_response_count_by_path",
                                                             "response_body_path", "503", 4, 3, params)

    #  Action: Ban | Subaction: Response | Event: Argument | Count: Asn
    def test_ban_action_event_argument_subaction_response_count_by_asn(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_res_asn": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_argument_sub_response_count_by_asn",
                                                             "response_body_asn", "503", 4, 3, params)

    #  Action: Ban | Subaction: Response | Event: Argument | Count: Method
    def test_ban_action_event_argument_subaction_response_count_by_method(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_res_method": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_argument_sub_response_count_by_method",
                                                             "response_body_method", "503", 4, 3, params)

    #  Action: Ban | Subaction: Response | Event: Argument | Count: Company
    def test_ban_action_event_argument_subaction_response_count_by_company(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_res_company": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_argument_sub_response_count_by_company",
                                                             "response_body_company", "503", 4, 3, params)

    #  Action: Ban | Subaction: Response | Event: Argument | Count: Country
    def test_ban_action_event_argument_subaction_response_count_by_country(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_res_country": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target, "test_ban_argument_sub_response_count_by_country",
                                                             "response_body_country", "503", 4, 3, params)

    #  Action: Ban | Subaction: Response | Event: Argument | Count: Authority
    def test_ban_action_event_argument_subaction_response_count_by_authority(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_res_authority": f"val-{i}"}} for i in range(3 + 2)]
        RateLimitHelper.check_rl_response_action_with_params(target,
                                                             "test_ban_argument_sub_response_count_by_authority",
                                                             "response_body_authority", "503", 4, 3, params)

    #  Action: Ban | Subaction: Redirect | Event: Argument | Count: Ip
    def test_ban_action_event_argument_subaction_redirect_count_by_ip(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_red_ip": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_argument_sub_redirect_count_by_ip",
                                                             "200", 6, 10, "https://google.com", params)

    #  Action: Ban | Subaction: Redirect | Event: Argument | Count: Asn
    def test_ban_action_event_argument_subaction_redirect_count_by_asn(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_red_asn": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_argument_sub_redirect_count_by_asn",
                                                             "200", 6, 10, "https://google.com", params)

    #  Action: Ban | Subaction: Redirect | Event: Argument | Count: Country
    def test_ban_action_event_argument_subaction_redirect_count_by_country(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_red_country": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_argument_sub_redirect_count_by_country",
                                                             "200", 6, 10, "https://google.com", params)

    #  Action: Ban | Subaction: Redirect | Event: Argument | Count: Path
    def test_ban_action_event_argument_subaction_redirect_count_by_path(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_red_path": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_argument_sub_redirect_count_by_path",
                                                             "200", 6, 10, "https://google.com", params)

    #  Action: Ban | Subaction: Redirect | Event: Argument | Count: Method
    def test_ban_action_event_argument_subaction_redirect_count_by_method(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_red_method": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_argument_sub_redirect_count_by_method",
                                                             "200", 6, 10, "https://google.com", params)

    #  Action: Ban | Subaction: Redirect | Event: Argument | Count: Company
    def test_ban_action_event_argument_subaction_redirect_count_by_company(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_red_company": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target, "test_ban_argument_sub_redirect_count_by_company",
                                                             "200", 6, 10, "https://google.com", params)

    #  Action: Ban | Subaction: Redirect | Event: Argument | Count: Authority
    def test_ban_action_event_argument_subaction_redirect_count_by_authority(self, cli, target, log_fixture):
        params = [{"params": {"arg_event_ban_red_authority": f"val-{i}"}} for i in range(10 + 2)]
        RateLimitHelper.check_rl_redirect_action_with_params(target,
                                                             "test_ban_argument_sub_redirect_count_by_authority",
                                                             "200", 6, 10, "https://google.com", params)
