import random
import string
import time
import pytest
from e2e.core.base_helper import target, cli, log_fixture, BaseHelper


@pytest.mark.usefixtures("api_setup")
@pytest.mark.log_tests
@pytest.mark.all_modules
class TestLogs(BaseHelper):
    def test_logs(self, cli, target, log_fixture):
        test_pattern = "/test" + BaseHelper.generate_random_string(20)
        assert target.is_reachable(test_pattern)
        assert log_fixture.check_log_pattern("request.attributes.uri", test_pattern)
