# run with --log-level info for debugging tests & fixtures
# pylint: disable=missing-function-docstring,missing-module-docstring


def pytest_addoption(parser):
    parser.addoption("--all", action="store_true", help="run all combinations")
    parser.addoption(
        "--base-conf-url",
        help="Base url for confserver API",
        type=str,
        default="http://localhost:30000/api/v2/",
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
        default="",
    )
    parser.addoption(
        "--flip-requests",
        help="For each request to the protected urls, also send len(request)*8 requests with 1 flipped bit",
        action="store_true",
        default=False,
    )
