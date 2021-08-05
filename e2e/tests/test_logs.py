import random
import string
import time
import pytest
from e2e.core.base_helpers import target, cli, BaseHelper
from e2e.helpers.log_helper import log_fixture


@pytest.mark.usefixtures('api_setup')
@pytest.mark.log_tests
@pytest.mark.all_modules
class TestLogs(BaseHelper):
    def test_logs(self, cli, target, log_fixture):
        test_pattern = "/test" + BaseHelper.generate_random_string(20)
        assert target.is_reachable(test_pattern)
        time.sleep(10)
        assert log_fixture.check_log_pattern(test_pattern)