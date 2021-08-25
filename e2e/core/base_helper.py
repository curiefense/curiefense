import logging
import subprocess
import json
import time
import requests
import pytest
from urllib.parse import urlparse
import random
import string
import socket
from bs4 import BeautifulSoup

log = logging.getLogger('e2e')


class BaseHelper:
    TEST_CONFIG_NAME = 'master'

    # geo=US, company=SPRINTLINK, asn=1239
    IP4_US = "199.0.0.1"

    # geo=JP, company=Softbank BB Corp., asn=17676
    IP4_JP = "126.0.0.1"

    # geo=AU, company=CLOUDFLARENET, asn=13335
    IP4_CLOUDFLARE = "1.0.0.0"

    # geo=FR, company=Orange, asn=3215
    IP4_ORANGE = "2.0.0.0"

    IP6_1 = "0000:0000:0000:0000:0000:0000:0000:0001"
    IP6_2 = "0000:0000:0000:0000:0000:0000:0000:0002"

    @staticmethod
    def generate_random_string(num: int) -> str:
        return ''.join(random.choice(string.ascii_lowercase) for i in range(num))

    @staticmethod
    def generate_random_mixed_string(num: int) -> str:
        return ''.join(random.choice(string.ascii_lowercase + string.digits) for i in range(num))

    @staticmethod
    def verify_pattern_in_html(page_content, pattern: str) -> bool:
        soup = BeautifulSoup(page_content, 'html.parser')
        return pattern in str(soup)


class CliHelper():

    def __init__(self, base_url, api_config):
        self._base_url = base_url
        self._initial_version_cache = None
        self._api_config = api_config

    def call(self, args, inputjson=None):
        logging.info("Calling CLI with arguments: %s", args)
        cmd = ["curieconfctl", "-u", self._base_url, "-o", "json"]
        cmd += args.split(" ")
        indata = None
        if inputjson:
            indata = json.dumps(inputjson).encode("utf-8")

        process = subprocess.run(
            cmd,
            shell=False,
            input=indata,
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        if process.stdout:
            logging.debug("CLI output: %s", process.stdout)

            try:
                return json.loads(process.stdout.decode("utf-8"))
            except json.JSONDecodeError:
                return process.stdout.decode("utf-8")
        else:
            return []

    def delete_test_config(self):
        self.call("conf delete test")

    def initial_version(self):
        if not self._initial_version_cache:
            versions = self.call("conf list-versions master")
            self._initial_version_cache = versions[-3]["version"]
        return self._initial_version_cache

    def empty_acl(self):
        version = self.initial_version()
        return self.call(f"doc get master aclpolicies --version {version}")

    def revert_and_enable(self, acl=True, waf=True):
        version = self.initial_version()
        self.call(f"conf revert {BaseHelper.TEST_CONFIG_NAME} {version}")
        urlmap = self.call(f"doc get {BaseHelper.TEST_CONFIG_NAME} {self._api_config['url_map']}")
        urlmap[0]["map"][0]["acl_active"] = acl
        urlmap[0]["map"][0]["waf_active"] = waf
        self.call(f"doc update {BaseHelper.TEST_CONFIG_NAME} {self._api_config['url_map']} /dev/stdin", inputjson=urlmap)

    def publish_and_apply(self):
        buckets = self.call("key get system publishinfo")

        for bucket in buckets["buckets"]:
            if bucket["name"] == "prod":
                url = bucket["url"]
        self.call(f"tool publish master {url}")
        time.sleep(10)

    def default_config(self):
        self.revert_and_enable()
        self.publish_and_apply()


class TargetHelper:
    def __init__(self, base_url):
        self._base_url = base_url

    def query(
            self, path="/", suffix="", method="GET", headers=None, srcip=None, **kwargs
    ):
        # specifying a path helps spot tests easily in the access log
        if headers is None:
            headers = {}
        if srcip is not None:
            headers["X-Forwarded-For"] = srcip
        res = requests.request(
            method=method, url=self._base_url + path + suffix, headers=headers, **kwargs
        )
        return res

    def is_reachable(self, *args, **kwargs):
        res = self.query(*args, **kwargs)
        return res.status_code in [200, 404]

    def authority(self) -> str:
        return urlparse(self._base_url).netloc


class LogHelper:
    def __init__(self, base_url, es_url):
        self._base_url = base_url
        self._es_url = es_url + "/_search"

    def check_log_pattern(self, field, pattern):
        time.sleep(12)  # waits for log file to be updated
        data = {
            "query": {"bool": {"must": {"match": {field: pattern}}}}
        }
        res = requests.get(self._es_url, json=data)
        nbhits = res.json()["hits"]["total"]["value"]
        if nbhits == 1:
            return True
        else:
            print("Pattern %r" % (pattern,))
            print("Request result %r" % (res,))
            return False

    def check_log_pattern_updates(self, field, pattern):
        time.sleep(12)  # waits for log file to be updated
        data = {
            "query": {"bool": {"must": {"match": {field: pattern}}}}
        }
        res = requests.get(self._es_url, json=data)
        nbhits = res.json()["hits"]["total"]["value"]
        return nbhits


@pytest.fixture(scope="session")
def log_fixture(request):
    url = request.config.getoption("--base-ui-url").rstrip("/")
    es_url = request.config.getoption("--elasticsearch-url").rstrip("/")
    return LogHelper(url, es_url)


@pytest.fixture(scope='session')
def cli(request, api_config):
    return CliHelper(request.config.getoption('--base-conf-url'), api_config)


@pytest.fixture(scope="class")
def default_config(cli):
    cli.revert_and_enable()
    cli.publish_and_apply()


@pytest.fixture(scope="session")
def target(request):
    url = request.config.getoption("--base-protected-url").rstrip("/")
    return TargetHelper(url)


@pytest.fixture(scope="function", params=["headers", "cookies", "params"])
def section(request):
    return request.param
