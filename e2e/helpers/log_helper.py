import requests
import pytest


class LogHelper:
    def __init__(self, base_url, es_url):
        self._base_url = base_url
        self._es_url = es_url + "/_search"

    def check_log_pattern(self, pattern):
        data = {
            "query": {"bool": {"must": {"match": {"request.attributes.uri": pattern}}}}
        }
        res = requests.get(self._es_url, json=data)
        nbhits = res.json()["hits"]["total"]["value"]
        if nbhits == 1:
            return True
        else:
            print("Pattern %r" % (pattern,))
            print("Request result %r" % (res,))
            return False


@pytest.fixture(scope="session")
def log_fixture(request):
    url = request.config.getoption("--base-ui-url").rstrip("/")
    es_url = request.config.getoption("--elasticsearch-url").rstrip("/")
    return LogHelper(url, es_url)