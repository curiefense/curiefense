import pytest
from curieconf import confclient
import subprocess
import socket
import time
from data import bootstrap_config_json, bootstrap_small_config_json

API_ADDRESS = ("127.0.0.1", 5000)
API_BASE = "http://%s:%i/api/v3/" % API_ADDRESS


@pytest.fixture(scope="function", params=["git://"])  # "mongodb://172.17.0.4/pytest"])
def curieapi_empty(request, tmpdir):
    if request.param.startswith("git"):
        url = request.param + str(tmpdir) + "/git"
    elif request.param.startswith("mongodb"):
        url = "%s_%i" % (request.param, time.time())
    else:
        raise NotImplemented

    p = subprocess.Popen(["curieconf_server", "--dbpath", url])

    t = time.time()
    while time.time() - t < 10:
        s = socket.socket()
        try:
            s.connect(API_ADDRESS)
        except ConnectionRefusedError:
            try:
                status = p.wait(0.05)
            except subprocess.TimeoutExpired:
                pass
            else:
                raise Exception("curieconf_server stopped with status %r" % status)
        except:
            raise
        else:
            break

    print("started")
    yield confclient.get_api(API_BASE)
    print("stopped")
    p.kill()
    p.wait()
    # XXX: drop mongodb test database


@pytest.fixture()
def curieapi(request, curieapi_empty):
    r = curieapi_empty.configs.create_name("pytest", body=bootstrap_config_json)
    assert r.status_code == 200
    return curieapi_empty


@pytest.fixture()
def curieapi_small(request, curieapi_empty):
    r = curieapi_empty.configs.create_name("pytest", body=bootstrap_small_config_json)
    assert r.status_code == 200
    return curieapi_empty
