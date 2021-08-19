import pytest
from e2e.core.base_helper import cli, target, section, log_fixture
from e2e.helpers.rl_helper import RateLimitHelper
from e2e.helpers.rl_http_helper import ratelimit_http_config


@pytest.mark.usefixtures('api_setup', 'ratelimit_http_config')
@pytest.mark.all_modules
@pytest.mark.rate_limit_tests
@pytest.mark.rate_limit_http_request_tests
class TestRateLimitHttp:

    #  Action: 503  | Event: http | Count: Ip
    def test_503_action_event_http_request_count_by_ip(self, target):
        RateLimitHelper.check_503_response(target,
                                           "test_503_action_event_http_request_count_by_ip", 3, 8)

    #  Action: 503  | Event: http | Count: Path
    def test_503_action_event_http_request_count_by_path(self, target):
        RateLimitHelper.check_503_response(target,
                                           "test_503_action_event_http_request_count_by_path", 3, 8)

    #  Action: 503  | Event: http | Count: Uri
    def test_503_action_event_http_request_count_by_uri(self, target):
        RateLimitHelper.check_503_response(target,
                                           "test_503_action_event_http_request_count_by_uri", 3, 8)

    #  Action: 503  | Event: http | Count: Asn
    def test_503_action_event_http_request_count_by_asn(self, target):
        RateLimitHelper.check_503_response(target,
                                           "test_503_action_event_http_request_count_by_asn", 3, 8)

    #  Action: 503  | Event: http | Count: Query
    def test_503_action_event_http_request_count_by_query(self, target):
        RateLimitHelper.check_503_response(target,
                                           "test_503_action_event_http_request_count_by_query", 3, 8)

    #  Action: 503  | Event: http | Count: Method
    def test_503_action_event_http_request_count_by_method(self, target):
        RateLimitHelper.check_503_response(target,
                                           "test_503_action_event_http_request_count_by_method", 3, 8)

    #  Action: 503  | Event: http | Count: ompany
    def test_503_action_event_http_request_count_by_company(self, target):
        RateLimitHelper.check_503_response(target,
                                           "test_503_action_event_http_request_count_by_company", 3, 8)

    #  Action: 503  | Event: http | Count: Country
    def test_503_action_event_http_request_count_by_country(self, target):
        RateLimitHelper.check_503_response(target,
                                           "test_503_action_event_http_request_count_by_country", 3, 8)

    #  Action: 503  | Event: http | Count: Authority
    def test_503_action_event_http_request_count_by_authority(self, target):
        RateLimitHelper.check_503_response(target,
                                           "test_503_action_event_http_request_count_by_authority", 3, 8)

    #  Action: Tag only  | Event: http | Count: Ip
    def test_tag_only_action_event_http_request_count_by_ip(self, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_tag_only_http_ip",
                                                                       "tags", "test-tag-only-http-ip", 1)

    #  Action: Tag only  | Event: http | Count: Uri
    def test_tag_only_action_event_http_request_count_by_uri(self, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_tag_only_http_uri",
                                                                       "tags", "test-tag-only-http-uri", 1)

    #  Action: Tag only  | Event: http | Count: Asn
    def test_tag_only_action_event_http_request_count_by_asn(self, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_tag_only_http_asn",
                                                                       "tags", "test-tag-only-http-asn", 1)

    #  Action: Tag only  | Event: http | Count: Path
    def test_tag_only_action_event_http_request_count_by_path(self, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_tag_only_http_path",
                                                                       "tags", "test-tag-only-http-path", 1)

    #  Action: Tag only  | Event: http | Count: Query
    def test_tag_only_action_event_http_request_count_by_query(self, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_tag_only_http_query",
                                                                       "tags", "test-tag-only-http-query", 1)

    #  Action: Tag only  | Event: http | Count: Method
    def test_tag_only_action_event_http_request_count_by_method(self, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_tag_only_http_method",
                                                                       "tags", "test-tag-only-http-method", 1)

    #  Action: Tag only  | Event: http | Count: Company
    def test_tag_only_action_event_http_request_count_by_company(self, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_tag_only_http_company",
                                                                       "tags", "test-tag-only-http-company", 1)

    #  Action: Tag only  | Event: http | Count: Country
    def test_tag_only_action_event_http_request_count_by_country(self, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_tag_only_http_country",
                                                                       "tags", "test-tag-only-http-country", 1)

    #  Action: Tag only  | Event: http | Count: Authority
    def test_tag_only_action_event_http_request_count_by_authority(self, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_tag_only_http_authority",
                                                                       "tags", "test-tag-only-http-authority", 1)

    #  Action: Response | Event: http | Count: Ip
    def test_response_action_event_http_request_count_by_ip(self, cli, target):
        RateLimitHelper.check_rate_limits_response_action(target, "test_response_http_ip", "response_body_ip",
                                                          "302", 9, 15)

    #  Action: Response | Event: http | Count: Uri
    def test_response_action_event_http_request_count_by_uri(self, cli, target):
        RateLimitHelper.check_rate_limits_response_action(target, "test_response_http_uri", "response_body_uri",
                                                          "503", 9, 15)

    #  Action: Response | Event: http | Count: Query
    def test_response_action_event_http_request_count_by_query(self, cli, target):
        RateLimitHelper.check_rate_limits_response_action(target, "test_response_http_query", "response_body_query",
                                                          "302", 9, 15)

    #  Action: Response | Event: http | Count: Path
    def test_response_action_event_http_request_count_by_path(self, cli, target):
        RateLimitHelper.check_rate_limits_response_action(target, "test_response_http_path", "response_body_path",
                                                          "302", 9, 15)

    #  Action: Response | Event: http | Count: Asn
    def test_response_action_event_http_request_count_by_asn(self, cli, target):
        RateLimitHelper.check_rate_limits_response_action(target, "test_response_http_asn", "response_body_asn",
                                                          "302", 9, 15)

    #  Action: Response | Event: http | Count: Method
    def test_response_action_event_http_request_count_by_method(self, cli, target):
        RateLimitHelper.check_rate_limits_response_action(target, "test_response_http_method",
                                                          "response_body_method", "302", 9, 15)

    #  Action: Response | Event: http | Count: Company
    def test_response_action_event_http_request_count_by_company(self, cli, target):
        RateLimitHelper.check_rate_limits_response_action(target, "test_response_http_company",
                                                          "response_body_company", "302", 9, 15)

    #  Action: Response | Event: http | Count: Country
    def test_response_action_event_http_request_count_by_country(self, cli, target):
        RateLimitHelper.check_rate_limits_response_action(target, "test_response_http_country",
                                                          "response_body_country", "302", 9, 15)

    #  Action: Response | Event: http | Count: Authority
    def test_response_action_event_http_request_count_by_authority(self, cli, target):
        RateLimitHelper.check_rate_limits_response_action(target, "test_response_http_authority",
                                                          "response_body_authority", "302", 9, 15)

    #  Action: Challenge | Event: http | Count: Ip
    def test_challenge_action_event_http_request_count_by_ip(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target, "test_challenge_http_ip", 4, 5)

    #  Action: Challenge | Event: http | Count: Uri
    def test_challenge_action_event_http_request_count_by_uri(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target, "test_challenge_http_uri", 4, 5)

    #  Action: Challenge | Event: http | Count: Query
    def test_challenge_action_event_http_request_count_by_query(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target, "test_challenge_http_query", 4, 5)

    #  Action: Challenge | Event: http | Count: Path
    def test_challenge_action_event_http_request_count_by_path(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target, "test_challenge_http_path", 4, 5)

    #  Action: Challenge | Event: http | Count: Asn
    def test_challenge_action_event_http_request_count_by_asn(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target, "test_challenge_http_asn", 4, 5)

    #  Action: Challenge | Event: http | Count: Method
    def test_challenge_action_event_http_request_count_by_method(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target, "test_challenge_http_method", 4, 5)

    #  Action: Challenge | Event: http | Count: Company
    def test_challenge_action_event_http_request_count_by_company(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target, "test_challenge_http_company", 4, 5)

    #  Action: Challenge | Event: http | Count: Country
    def test_challenge_action_event_http_request_count_by_country(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target, "test_challenge_http_country", 4, 5)

    #  Action: Challenge | Event: http | Count: Authority
    def test_challenge_action_event_http_request_count_by_authority(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target, "test_challenge_http_authority", 4, 5)

    #  Action: Redirect | Event: http | Count: Ip
    def test_redirect_action_event_http_request_count_by_ip(self, cli, target):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_redirect_http_ip", "200", 3, 3,
                                                         "https://yahoo.com")

    #  Action: Redirect | Event: http | Count: Uri
    def test_redirect_action_event_http_request_count_by_uri(self, cli, target):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_redirect_http_uri", "200", 3, 3,
                                                         "https://yahoo.com")

    #  Action: Redirect | Event: http | Count: Query
    def test_redirect_action_event_http_request_count_by_query(self, cli, target):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_redirect_http_query", "200", 3, 3,
                                                         "https://yahoo.com")

    #  Action: Redirect | Event: http | Count: Path
    def test_redirect_action_event_http_request_count_by_path(self, cli, target):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_redirect_http_path", "200", 3, 3,
                                                         "https://yahoo.com")

    #  Action: Redirect | Event: http | Count: Asn
    def test_redirect_action_event_http_request_count_by_asn(self, cli, target):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_redirect_http_asn", "200", 3, 3,
                                                         "https://yahoo.com")

    #  Action: Redirect | Event: http | Count: Method
    def test_redirect_action_event_http_request_count_by_method(self, cli, target):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_redirect_http_method", "200", 3, 3,
                                                         "https://yahoo.com")

    #  Action: Redirect | Event: http | Count: Company
    def test_redirect_action_event_http_request_count_by_company(self, cli, target):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_redirect_http_company", "200", 3, 3,
                                                         "https://yahoo.com")

    #  Action: Redirect | Event: http | Count: Country
    def test_redirect_action_event_http_request_count_by_country(self, cli, target):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_redirect_http_country", "200", 3, 3,
                                                         "https://yahoo.com")

    #  Action: Redirect | Event: http | Count: Authority
    def test_redirect_action_event_http_request_count_by_authority(self, cli, target):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_redirect_http_authority", "200", 3, 3,
                                                         "https://yahoo.com")

    #  Action: Ban | Subaction: 503 | Event: http | Count: Ip
    def test_ban_action_event_http_request_subaction_503_count_by_ip(self, cli, target):
        RateLimitHelper.check_503_response(target, "test_ban_http_subaction_503_count_by_ip", 8, 10)

    #  Action: Ban | Subaction: 503 | Event: http | Count: Uri
    def test_ban_action_event_http_request_subaction_503_count_by_uri(self, cli, target):
        RateLimitHelper.check_503_response(target, "test_ban_http_subaction_503_count_by_uri", 8, 10)

    #  Action: Ban | Subaction: 503 | Event: http | Count: Query
    def test_ban_action_event_http_request_subaction_503_count_by_query(self, cli, target):
        RateLimitHelper.check_503_response(target, "test_ban_http_subaction_503_count_by_query", 8, 10)

    #  Action: Ban | Subaction: 503 | Event: http | Count: Path
    def test_ban_action_event_http_request_subaction_503_count_by_path(self, cli, target):
        RateLimitHelper.check_503_response(target, "test_ban_http_subaction_503_count_by_path", 8, 10)

    #  Action: Ban | Subaction: 503 | Event: http | Count: Asn
    def test_ban_action_event_http_request_subaction_503_count_by_asn(self, cli, target):
        RateLimitHelper.check_503_response(target, "test_ban_http_subaction_503_count_by_asn", 8, 10)

    #  Action: Ban | Subaction: 503 | Event: http | Count: Method
    def test_ban_action_event_http_request_subaction_503_count_by_method(self, cli, target):
        RateLimitHelper.check_503_response(target, "test_ban_http_subaction_503_count_by_method", 8, 10)

    #  Action: Ban | Subaction: 503 | Event: http | Count: Company
    def test_ban_action_event_http_request_subaction_503_count_by_company(self, cli, target):
        RateLimitHelper.check_503_response(target, "test_ban_http_subaction_503_count_by_company", 8, 10)

    #  Action: Ban | Subaction: 503 | Event: http | Count: Country
    def test_ban_action_event_http_request_subaction_503_count_by_country(self, cli, target):
        RateLimitHelper.check_503_response(target, "test_ban_http_subaction_503_count_by_country", 8, 10)

    #  Action: Ban | Subaction: 503 | Event: http | Count: Authority
    def test_ban_action_event_http_request_subaction_503_count_by_authority(self, cli, target):
        RateLimitHelper.check_503_response(target, "test_ban_http_subaction_503_count_by_authority", 8, 10)

    #  Action: Ban | Subaction: Challenge | Event: http | Count: Ip
    def test_ban_action_event_http_request_subaction_challenge_count_by_ip(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target, "test_ban_http_subaction_challenge_count_by_ip",
                                                          6, 5)

    #  Action: Ban | Subaction: Challenge | Event: http | Count: Uri
    def test_ban_action_event_http_request_subaction_challenge_count_by_uri(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target, "test_ban_http_subaction_challenge_count_by_uri",
                                                          6, 5)

    #  Action: Ban | Subaction: Challenge | Event: http | Count: Query
    def test_ban_action_event_http_request_subaction_challenge_count_by_query(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target, "test_ban_http_subaction_challenge_count_by_query",
                                                          6, 5)

    #  Action: Ban | Subaction: Challenge | Event: http | Count: Path
    def test_ban_action_event_http_request_subaction_challenge_count_by_path(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target, "test_ban_http_subaction_challenge_count_by_path",
                                                          6, 5)

    #  Action: Ban | Subaction: Challenge | Event: http | Count: Asn
    def test_ban_action_event_http_request_subaction_challenge_count_by_asn(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target, "test_ban_http_subaction_challenge_count_by_asn",
                                                          6, 5)

    #  Action: Ban | Subaction: Challenge | Event: http | Count: Method
    def test_ban_action_event_http_request_subaction_challenge_count_by_method(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target,
                                                          "test_ban_http_subaction_challenge_count_by_method",
                                                          6, 5)

    #  Action: Ban | Subaction: Challenge | Event: http | Count: Company
    def test_ban_action_event_http_request_subaction_challenge_count_by_company(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target,
                                                          "test_ban_http_subaction_challenge_count_by_company",
                                                          6, 5)

    #  Action: Ban | Subaction: Challenge | Event: http | Count: Country
    def test_ban_action_event_http_request_subaction_challenge_count_by_country(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target,
                                                          "test_ban_http_subaction_challenge_count_by_country",
                                                          6, 5)

    #  Action: Ban | Subaction: Challenge | Event: http | Count: Authority
    def test_ban_action_event_http_request_subaction_challenge_count_by_authority(self, cli, target):
        RateLimitHelper.check_rate_limit_challenge_action(target,
                                                          "test_ban_http_subaction_challenge_count_by_authority",
                                                          6, 5)

    #  Action: Ban | Subaction: Tag only | Event: http | Count: Ip
    def test_ban_action_event_http_request_subaction_tag_only_count_by_ip(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_ban_http_sub_tag_count_by_ip",
                                                                       "tags",
                                                                       "test-ban-http-sub-tag-count-by-ip", 1)

    #  Action: Ban | Subaction: Tag only | Event: http | Count: Uri
    def test_ban_action_event_http_request_subaction_tag_only_count_by_uri(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_ban_http_sub_tag_count_by_uri",
                                                                       "tags", "test-ban-http-sub-tag-count-by-uri",
                                                                       1)

    #  Action: Ban | Subaction: Tag only | Event: http | Count: Query
    def test_ban_action_event_http_request_subaction_tag_only_count_by_query(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_ban_http_sub_tag_count_by_query",
                                                                       "tags",
                                                                       "test-ban-http-sub-tag-count-by-query", 1)

    #  Action: Ban | Subaction: Tag only | Event: http | Count: Path
    def test_ban_action_event_http_request_subaction_tag_only_count_by_path(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_ban_http_sub_tag_count_by_path",
                                                                       "tags",
                                                                       "test-ban-http-sub-tag-count-by-path", 1)

    #  Action: Ban | Subaction: Tag only | Event: http | Count: Asn
    def test_ban_action_event_http_request_subaction_tag_only_count_by_asn(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_ban_http_sub_tag_count_by_asn",
                                                                       "tags",
                                                                       "test-ban-http-sub-tag-count-by-asn", 1)

    #  Action: Ban | Subaction: Tag only | Event: http | Count: Method
    def test_ban_action_event_http_request_subaction_tag_only_count_by_method(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_ban_http_sub_tag_count_by_method",
                                                                       "tags",
                                                                       "test-ban-http-sub-tag-count-by-method", 1)

    #  Action: Ban | Subaction: Tag only | Event: http | Count: Company
    def test_ban_action_event_http_request_subaction_tag_only_count_by_company(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_ban_http_sub_tag_count_by_company",
                                                                       "tags",
                                                                       "test-ban-http-sub-tag-count-by-company", 1)

    #  Action: Ban | Subaction: Tag only | Event: http | Count: Country
    def test_ban_action_event_http_request_subaction_tag_only_count_by_country(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_ban_http_sub_tag_count_by_country",
                                                                       "tags",
                                                                       "test-ban-http-sub-tag-count-by-country", 1)

    #  Action: Ban | Subaction: Tag only | Event: http | Count: Authority
    def test_ban_action_event_http_request_subaction_tag_only_count_by_authority(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_action_tag_only_with_pattern(log_fixture, target,
                                                                       "test_ban_http_sub_tag_count_by_authority",
                                                                       "tags",
                                                                       "test-ban-http-sub-tag-count-by-authority", 1)

    #  Action: Ban | Subaction: Response | Event: http | Count: Ip
    def test_ban_action_event_http_request_subaction_response_count_by_ip(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_response_action(target, "test_ban_http_sub_response_count_by_ip",
                                                          "response_body_ip", "503", 4, 3)

    #  Action: Ban | Subaction: Response | Event: http | Count: Uri
    def test_ban_action_event_http_request_subaction_response_count_by_uri(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_response_action(target, "test_ban_http_sub_response_count_by_uri",
                                                          "response_body_uri", "503", 4, 3)

    #  Action: Ban | Subaction: Response | Event: http | Count: Query
    def test_ban_action_event_http_request_subaction_response_count_by_query(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_response_action(target, "test_ban_http_sub_response_count_by_query",
                                                          "response_body_query", "503", 4, 3)

    #  Action: Ban | Subaction: Response | Event: http | Count: Path
    def test_ban_action_event_http_request_subaction_response_count_by_path(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_response_action(target, "test_ban_http_sub_response_count_by_path",
                                                          "response_body_path", "503", 4, 3)

    #  Action: Ban | Subaction: Response | Event: http | Count: Asn
    def test_ban_action_event_http_request_subaction_response_count_by_asn(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_response_action(target, "test_ban_http_sub_response_count_by_asn",
                                                          "response_body_asn", "503", 4, 3)

    #  Action: Ban | Subaction: Response | Event: http | Count: Method
    def test_ban_action_event_http_request_subaction_response_count_by_method(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_response_action(target, "test_ban_http_sub_response_count_by_method",
                                                          "response_body_method", "503", 4, 3)

    #  Action: Ban | Subaction: Response | Event: http | Count: Company
    def test_ban_action_event_http_request_subaction_response_count_by_company(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_response_action(target, "test_ban_http_sub_response_count_by_company",
                                                          "response_body_company", "503", 4, 3)

    #  Action: Ban | Subaction: Response | Event: http | Count: Country
    def test_ban_action_event_http_request_subaction_response_count_by_country(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_response_action(target, "test_ban_http_sub_response_count_by_country",
                                                          "response_body_country", "503", 4, 3)

    #  Action: Ban | Subaction: Response | Event: http | Count: Authority
    def test_ban_action_event_http_request_subaction_response_count_by_authority(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limits_response_action(target, "test_ban_http_sub_response_count_by_authority",
                                                          "response_body_authority", "503", 4, 3)

    #  Action: Ban | Subaction: Redirect | Event: http | Count: Ip
    def test_ban_action_event_http_request_subaction_redirect_count_by_ip(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_ban_http_sub_redirect_count_by_ip",
                                                         "200", 11, 20, "https://google.com")

    #  Action: Ban | Subaction: Redirect | Event: http | Count: Asn
    def test_ban_action_event_http_request_subaction_redirect_count_by_asn(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_ban_http_sub_redirect_count_by_asn",
                                                         "200", 11, 20, "https://google.com")

    #  Action: Ban | Subaction: Redirect | Event: http | Count: Country
    def test_ban_action_event_http_request_subaction_redirect_count_by_country(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_ban_http_sub_redirect_count_by_country",
                                                         "200", 11, 20, "https://google.com")

    #  Action: Ban | Subaction: Redirect | Event: http | Count: Uri
    def test_ban_action_event_http_request_subaction_redirect_count_by_uri(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_ban_http_sub_redirect_count_by_uri",
                                                         "200", 11, 20, "https://google.com")

    #  Action: Ban | Subaction: Redirect | Event: http | Count: Path
    def test_ban_action_event_http_request_subaction_redirect_count_by_path(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_ban_http_sub_redirect_count_by_path",
                                                         "200", 11, 20, "https://google.com")

    #  Action: Ban | Subaction: Redirect | Event: http | Count: Query
    def test_ban_action_event_http_request_subaction_redirect_count_by_query(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_ban_http_sub_redirect_count_by_query",
                                                         "200", 11, 20, "https://google.com")

    #  Action: Ban | Subaction: Redirect | Event: http | Count: Method
    def test_ban_action_event_http_request_subaction_redirect_count_by_method(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_ban_http_sub_redirect_count_by_method",
                                                         "200", 11, 20, "https://google.com")

    #  Action: Ban | Subaction: Redirect | Event: http | Count: Company
    def test_ban_action_event_http_request_subaction_redirect_count_by_company(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_ban_http_sub_redirect_count_by_company",
                                                         "200", 11, 20, "https://google.com")

    #  Action: Ban | Subaction: Redirect | Event: http | Count: Authority
    def test_ban_action_event_http_request_subaction_redirect_count_by_authority(self, cli, target, log_fixture):
        RateLimitHelper.check_rate_limit_redirect_action(target, "test_ban_http_sub_redirect_count_by_authority",
                                                         "200", 11, 20, "https://google.com")

