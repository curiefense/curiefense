import pytest
import os
from e2e.core import config
import json
from os.path import isfile, join

# run with --log-level info for debugging tests & fixtures
# pylint: disable=missing-function-docstring,missing-module-docstring


def pytest_addoption(parser):
    parser.addoption("--all", action="store_true", help="run all combinations")
    parser.addoption(
        "--base-conf-url",
        help="Base url for confserver API",
        type=str,
        default="http://localhost:30000/api/v1/",
    )
    parser.addoption(
        "--base-protected-url",
        help="Base URL for the protected website",
        default="http://localhost:30081",
    )
    parser.addoption(
        "--base-ui-url",
        help="Base URL for the UI server",
        default="http://localhost:30080",
    )
    parser.addoption(
        "--elasticsearch-url",
        help="Elasticsearch URL (ex. http://localhost:9200)",
        default="http://localhost:9200",
    )
    parser.addoption(
        "--module",
        action="store",
        default="all",
        help="please enter module name as a must param: --module")


@pytest.fixture(scope='session')
def module(request):
    module = request.config.getoption('--module')
    if not module:
        with open(os.path.join(config.HOME_PATH, 'tests', 'config.json')) as config_file:
            config_data = json.load(config_file)
            module = config_data['module']
    return module


@pytest.fixture(scope='session')
def data(module):
    if module == 'all':
        path = os.path.join(config.HOME_PATH, 'data')
        only_files = [f for f in os.listdir(path) if isfile(join(path, f))]
        merged_data = {}
        for file in only_files:
            with open(path + '/' + file) as infile:
                loaded = json.load(infile)
                merged_data.update(loaded)
        return merged_data
    else:
        env_data_files = {
            'acl': 'acl_data.json',
            'rl': 'rl_data.json'
        }
        with open(os.path.join(config.HOME_PATH, 'data', env_data_files[module])) as data_file:
            data = json.load(data_file)
        return data


@pytest.fixture(scope="class")
def api_setup(request, data):
    request.cls.data = data



