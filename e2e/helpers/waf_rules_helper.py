import pytest


@pytest.fixture(
    scope="function", params=[(100140, "htaccess"), (100112, "../../../../../")]
)
def wafrules(request):
    return request.param