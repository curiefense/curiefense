import pytest
from utils import compare_jblob
from simple_rest_client.exceptions import NotFoundError, AuthError, ClientError
import time

from curieconf.utils import bytes2jblob
from data import *
import data


##   ___ ___  _  _ ___ ___ ___ ___
##  / __/ _ \| \| | __|_ _/ __/ __|
## | (_| (_) | .` | _| | | (_ \__ \
##  \___\___/|_|\_|_| |___\___|___/


def test_configs_list(curieapi):
    r = curieapi.configs.list()
    assert r.status_code == 200
    assert {c["id"] for c in r.body} == {"master", "pytest"}


@pytest.mark.parametrize("conf", ["pytest", "master"])
def test_configs_list_versions(curieapi, conf):
    r = curieapi.configs.list_versions(conf)
    assert r.status_code == 200
    assert len(r.body) >= 1


def test_configs_create(curieapi_empty):
    curieapi_empty.configs.create(
        body={"meta": {"id": "pytest", "description": "pytest"}}
    )
    curieapi_empty.configs.create(
        body={"meta": {"id": "pytest2", "description": "pytest 2"}}
    )
    curieapi_empty.configs.create(
        body={"meta": {"id": "pytest3", "description": "pytest 3"}}
    )

    r = curieapi_empty.configs.list()
    lst = r.body
    (newconf1,) = [x for x in lst if x["id"] == "pytest"]
    (newconf2,) = [x for x in lst if x["id"] == "pytest2"]
    (newconf3,) = [x for x in lst if x["id"] == "pytest3"]

    assert len(newconf1["logs"]) == 3
    assert len(newconf2["logs"]) == 3
    assert len(newconf3["logs"]) == 3


def test_configs_create_dup(curieapi_empty):
    curieapi_empty.configs.create(
        body={"meta": {"id": "pytest", "description": "pytest"}}
    )
    with pytest.raises(ClientError) as e:
        curieapi_empty.configs.create(
            body={"meta": {"id": "pytest", "description": "pytest"}}
        )
    assert e.value.response.status_code == 409


def test_config_create_fail_clean(curieapi_empty):
    conf = {
        "meta": {"id": "pytest", "description": "pytest"},
        "documents": {
            "aclprofiles": [
                {"id": "qsmkldjqsdk", "name": "aqzdzd"},
                {"id": "sqd", "name": "qds"},
            ],
        },
    }
    curieapi_empty.configs.create_name("pytest1", body=conf)

    conf["blobs"] = {"geolite2asn": {}}  # geolite2asn should have field "format"
    with pytest.raises(ClientError) as e:
        curieapi_empty.configs.create_name("pytest2", body=conf)
    assert e.value.response.status_code == 400

    curieapi_empty.configs.create(
        body={"meta": {"id": "pytest3", "description": "pytest3"}}
    )
    r = curieapi_empty.documents.get("pytest3", "aclprofiles")
    assert r.body == []


def test_configs_get_empty(curieapi_empty):
    r = curieapi_empty.configs.get("master")
    assert r.status_code == 200
    res = r.body
    print(res)
    assert res["documents"] == {x: [] for x in DOCUMENTS_PATH}
    assert res["blobs"] == {k: bytes2jblob(v) for k, v in BLOBS_BOOTSTRAP.items()}


def test_configs_get(curieapi):
    r = curieapi.configs.get("pytest")
    assert r.status_code == 200
    res = r.body
    assert res["meta"]["id"] == "pytest"
    assert res["meta"]["version"] != None
    assert res["meta"]["date"] != None
    assert res["meta"]["description"] != None

    assert bootstrap_config_json["documents"].keys() == res["documents"].keys()
    for k in bootstrap_config_json["documents"]:
        assert bootstrap_config_json["documents"][k] == res["documents"][k], k

    assert bootstrap_config_json["blobs"].keys() == res["blobs"].keys()
    for k, retrieved in res["blobs"].items():
        sent = bootstrap_config_json["blobs"][k]
        assert compare_jblob(sent, retrieved)


def test_configs_update(curieapi_small):
    curieapi = curieapi_small
    r = curieapi.configs.get("pytest")
    assert r.status_code == 200

    jblob = {"blob": ["xxx"], "format": "json"}

    newlimits = [
        {
            "id": vec_limit["id"],
            "name": "foobar",
            "description": None,
            "timeframe": "3",
            "thresholds": [{"limit": "2", "action": None}],
            "include": None,
            "exclude": None,
            "key": "1",
            "pairwith": None,
        },
        {
            "id": "newid",
            "name": "barfoo",
            "description": None,
            "timeframe": "30",
            "thresholds": [{"limit": "20", "action": None}],
            "include": None,
            "exclude": None,
            "key": "10",
            "pairwith": None,
        },
    ]
    new_content_filter_rules = [
        {"id": vec_contentfilterrule["id"], "msg": "XXXX"},
        {
            "id": "newid",
            "name": None,
            "msg": "hola",
            "category": "1",
            "certainity": 2,
            "operand": "3",
            "severity": 4,
            "subcategory": "5",
        },
    ]
    update = {
        "meta": {"id": "renamed_pytest"},
        "blobs": {"geolite2country": jblob},
        "documents": {
            "ratelimits": newlimits,
            "contentfilterrules": new_content_filter_rules,
        },
        "delete_blobs": {"bltor": False, "blvpnip": True, "geolite2asn": True},
        "delete_documents": {
            "securitypolicies": {
                "sqdqsd": True,
                "fezfzf": True,
                vec_securitypolicy["id"]: False,
            },
            "contentfilterrules": {vec_contentfilterrule["id"]: True},
        },
    }

    r = curieapi.configs.update("pytest", body=update)
    assert r.status_code == 200
    with pytest.raises(NotFoundError):
        r = curieapi.configs.get("pytest")

    r = curieapi.configs.get("renamed_pytest")
    assert compare_jblob(r.body["blobs"]["geolite2country"], jblob)
    assert compare_jblob(r.body["blobs"]["geolite2asn"], {})
    assert r.body["documents"]["ratelimits"] == newlimits
    assert r.body["documents"]["contentfilterrules"] == new_content_filter_rules[1:]
    assert r.body["documents"]["securitypolicies"] == [vec_securitypolicy]


##  ___ _    ___  ___ ___
## | _ ) |  / _ \| _ ) __|
## | _ \ |_| (_) | _ \__ \
## |___/____\___/|___/___/


@pytest.mark.parametrize("blob", vec_blobs.keys())
def test_blobs_get(curieapi_small, blob):
    r = curieapi_small.blobs.get("pytest", blob)
    assert r.status_code == 200
    assert compare_jblob(r.body, vec_blobs[blob])


@pytest.mark.parametrize("blob", vec_blobs.keys())
def test_blobs_get_2(curieapi, blob):
    r = curieapi.blobs.get("pytest", blob)
    assert r.status_code == 200
    assert compare_jblob(r.body, bootstrap_config_json["blobs"][blob])


@pytest.mark.parametrize("blob", vec_blobs.keys())
def test_blobs_put(curieapi_small, blob):
    r = curieapi_small.blobs.get("pytest", blob)
    assert r.status_code == 200
    assert compare_jblob(r.body, vec_blobs[blob])
    jdata = {"format": "string", "blob": "LQKJDMLAKJDLQDS?NF%LQKNSKQ"}
    r = curieapi_small.blobs.update("pytest", blob, body=jdata)
    assert r.status_code == 200
    r = curieapi_small.blobs.get("pytest", blob)
    assert r.status_code == 200
    assert compare_jblob(r.body, jdata)


@pytest.mark.parametrize("blob", vec_blobs.keys())
def test_blobs_delete(curieapi_small, blob):
    r = curieapi_small.blobs.get("pytest", blob)
    assert r.status_code == 200
    assert compare_jblob(r.body, vec_blobs[blob])
    r = curieapi_small.blobs.delete("pytest", blob)
    assert r.status_code == 200
    with pytest.raises(NotFoundError):
        r = curieapi_small.blobs.get("pytest", blob)
    with pytest.raises(NotFoundError):
        r = curieapi_small.blobs.delete("pytest", blob)


@pytest.mark.parametrize("blob", vec_blobs.keys())
def test_blobs_revert(curieapi, blob):
    r = curieapi.blobs.get("pytest", blob)
    assert r.status_code == 200
    old = r.body
    r = curieapi.configs.list_versions("pytest")
    assert r.status_code == 200
    oldv = r.body[0]["version"]

    new = {"format": "string", "blob": "LQKJDMLAKJDLQDS?NF%LQKNSKQ"}
    r = curieapi.blobs.update("pytest", blob, body=new)
    assert r.status_code == 200
    r = curieapi.blobs.get("pytest", blob)
    assert r.status_code == 200
    assert compare_jblob(r.body, new)

    r = curieapi.blobs.revert("pytest", blob, oldv)
    assert r.status_code == 200
    r = curieapi.blobs.get("pytest", blob)
    assert r.status_code == 200
    assert r.body == old


@pytest.mark.parametrize("blobname", vec_blobs.keys())
def test_blobs_list_versions(curieapi, blobname):
    r = curieapi.blobs.get("pytest", blobname)
    assert r.status_code == 200
    old = r.body

    r = curieapi.blobs.list_versions("pytest", blobname)
    assert r.status_code == 200
    v1 = r.body
    assert len(v1) > 1
    assert "Initial" in v1[-1]["message"]

    r = curieapi.blobs.delete("pytest", blobname)
    assert r.status_code == 200
    r = curieapi.blobs.list_versions("pytest", blobname)
    assert r.status_code == 200
    v2 = r.body
    assert len(v2) - len(v1) == 1
    assert "Delete" in v2[0]["message"]

    r = curieapi.blobs.create("pytest", blobname, body=old)
    assert r.status_code == 200
    r = curieapi.blobs.list_versions("pytest", blobname)
    assert r.status_code == 200
    v3 = r.body
    assert len(v3) - len(v2) == 1
    assert "Create" in v3[0]["message"]

    newblob = {"format": "string", "blob": "LQKJDMLAKJDLQDS?NF%LQKNSKQ"}
    r = curieapi.blobs.update("pytest", blobname, body=newblob)
    assert r.status_code == 200
    r = curieapi.blobs.list_versions("pytest", blobname)
    assert r.status_code == 200
    v4 = r.body
    assert len(v4) - len(v3) == 1
    assert "Update" in v4[0]["message"]

    newblob = {"format": "string", "blob": "LQKJDMLAKJDLQDS?NF%LQKNSZAZZAKQ"}
    r = curieapi.blobs.update("master", blobname, body=newblob)
    assert r.status_code == 200

    r = curieapi.blobs.delete("pytest", blobname)
    assert r.status_code == 200

    r = curieapi.blobs.list_versions("pytest", blobname)
    assert r.status_code == 200
    v5 = r.body
    assert "Delete" in v5[0]["message"]

    r = curieapi.blobs.list_versions("master", blobname)
    assert r.status_code == 200
    v6 = r.body
    assert "Update" in v6[0]["message"]


##  ___   ___   ___ _   _ __  __ ___ _  _ _____ ___
## |   \ / _ \ / __| | | |  \/  | __| \| |_   _/ __|
## | |) | (_) | (__| |_| | |\/| | _|| .` | | | \__ \
## |___/ \___/ \___|\___/|_|  |_|___|_|\_| |_| |___/


@pytest.mark.parametrize("doc", vec_documents.keys())
def test_documents_list_small(curieapi_small, doc):
    r = curieapi_small.documents.get("pytest", doc)
    assert r.status_code == 200
    assert r.body == [vec_documents[doc]]


@pytest.mark.parametrize("doc", bootstrap_config_json["documents"].keys())
def test_documents_list(curieapi, doc):
    r = curieapi.documents.get("pytest", doc)
    assert r.status_code == 200
    assert r.body == bootstrap_config_json["documents"][doc]


def test_documents_list_404(curieapi_small):
    with pytest.raises(NotFoundError):
        r = curieapi_small.documents.get("pytest", "XXXXX")


@pytest.mark.parametrize("doc", vec_documents.keys())
def test_documents_create(curieapi_empty, doc):
    r = curieapi_empty.documents.create("master", doc, body=[vec_documents[doc]])
    assert r.status_code == 200
    r = curieapi_empty.documents.get("master", doc)
    assert r.status_code == 200
    assert r.body == [vec_documents[doc]]
    with pytest.raises(ClientError) as e:
        r = curieapi_empty.documents.create("master", doc, body=[vec_documents[doc]])
    assert e.value.response.status_code == 409


@pytest.mark.parametrize("doc", vec_documents.keys())
def test_documents_delete(curieapi, doc):
    r = curieapi.documents.get("pytest", doc)
    assert r.status_code == 200
    assert r.body != []
    r = curieapi.documents.delete("pytest", doc)
    assert r.status_code == 200
    r = curieapi.documents.get("pytest", doc)
    assert r.status_code == 200
    assert r.body == []


@pytest.mark.parametrize("doc", vec_documents.keys())
def test_documents_update(curieapi, doc):
    r = curieapi.documents.get("pytest", doc)
    assert r.status_code == 200
    assert r.body != []
    old = r.body
    oldids = {e["id"] for e in old}

    myid = "XXXXXXXXXXX"

    update = [{**old[0], **{"id": myid}}, old[0]]

    r = curieapi.documents.update("pytest", doc, body=update)
    assert r.status_code == 200
    r = curieapi.documents.get("pytest", doc)
    assert r.status_code == 200
    newids = {e["id"] for e in r.body}

    assert oldids | {myid} == newids


@pytest.mark.parametrize("doc", vec_documents.keys())
def test_documents_revert(curieapi, doc):
    r = curieapi.documents.get("pytest", doc)
    assert r.status_code == 200
    old = r.body
    r = curieapi.configs.list_versions("pytest")
    assert r.status_code == 200
    oldv = r.body[0]["version"]

    new = [{**old[0], **{"name": "%i" % time.time()}}]
    # use "update" because we can not delete __default__ content filter and acl
    r = curieapi.documents.update("pytest", doc, body=new)
    assert r.status_code == 200
    r = curieapi.documents.get("pytest", doc)
    assert r.status_code == 200
    assert r.body[0]["name"] == new[0]["name"]

    r = curieapi.documents.revert("pytest", doc, oldv)
    assert r.status_code == 200
    r = curieapi.documents.get("pytest", doc)
    assert r.status_code == 200
    assert r.body == old


@pytest.mark.parametrize("docname", vec_documents.keys())
def test_documents_list_versions(curieapi, docname):
    r = curieapi.documents.get("pytest", docname)
    assert r.status_code == 200
    old = r.body

    r = curieapi.documents.list_versions("pytest", docname)
    assert r.status_code == 200
    v1 = r.body
    assert len(v1) > 1
    assert "Initial" in v1[-1]["message"]

    r = curieapi.documents.delete("pytest", docname)
    assert r.status_code == 200
    r = curieapi.documents.list_versions("pytest", docname)
    assert r.status_code == 200
    v2 = r.body
    assert len(v2) - len(v1) == 1
    assert "Delete" in v2[0]["message"]

    r = curieapi.documents.create("pytest", docname, body=old)
    assert r.status_code == 200
    r = curieapi.documents.list_versions("pytest", docname)
    assert r.status_code == 200
    v3 = r.body
    assert len(v3) - len(v2) == 1
    assert "New version" in v3[0]["message"]

    r = curieapi.entries.create("pytest", docname, body={"id": "qdsdsq"})
    assert r.status_code == 200
    r = curieapi.documents.list_versions("pytest", docname)
    assert r.status_code == 200
    v4 = r.body
    assert len(v4) - len(v3) == 1
    assert "Add entry" in v4[0]["message"]

    r = curieapi.entries.create("master", docname, body={"id": "qdsdsq"})
    assert r.status_code == 200
    old.append({**old[0], **{"id": "vsdsd", "name": "%i" % time.time()}})
    r = curieapi.documents.update("pytest", docname, body=old)
    assert r.status_code == 200

    r = curieapi.documents.list_versions("pytest", docname)
    assert r.status_code == 200
    v5 = r.body
    assert "Update" in v5[0]["message"]

    r = curieapi.documents.list_versions("master", docname)
    assert r.status_code == 200
    v6 = r.body
    assert "Add entry" in v6[0]["message"]


##  ___ _  _ _____ ___ ___ ___ ___
## | __| \| |_   _| _ \_ _| __/ __|
## | _|| .` | | | |   /| || _|\__ \
## |___|_|\_| |_| |_|_\___|___|___/


@pytest.mark.parametrize("doc", vec_documents.keys())
def test_entries_list(curieapi_small, doc):
    r = curieapi_small.entries.list("pytest", doc)
    assert r.status_code == 200
    assert r.body == [vec_documents[doc]["id"]]


@pytest.mark.parametrize("doc", vec_documents.keys())
def test_entries_list_2(curieapi, doc):
    r = curieapi.entries.list("pytest", doc)
    assert r.status_code == 200
    assert r.body == [e["id"] for e in bootstrap_config_json["documents"][doc]]


@pytest.mark.parametrize("doc", vec_documents.keys())
def test_entries_get(curieapi_small, doc):
    r = curieapi_small.entries.get("pytest", doc, vec_documents[doc]["id"])
    assert r.status_code == 200
    assert r.body == vec_documents[doc]


def test_entries_get_404(curieapi_small):
    with pytest.raises(NotFoundError):
        r = curieapi_small.documents.get("pytest", "XXXX", "XXXX")
    with pytest.raises(NotFoundError):
        r = curieapi_small.documents.get("pytest", "limit", "XXXX")


@pytest.mark.parametrize("doc", vec_documents.keys())
def test_entries_create(curieapi, doc):
    vec = vec_documents[doc].copy()
    vec["id"] = "new-id"
    r = curieapi.entries.create("pytest", doc, body=vec)
    assert r.status_code == 200
    r = curieapi.entries.get("pytest", doc, vec["id"])
    assert r.status_code == 200
    assert r.body == vec
    with pytest.raises(ClientError) as e:
        r = curieapi.entries.create("pytest", doc, body=vec)
    assert e.value.response.status_code == 409


@pytest.mark.parametrize("doc", vec_documents.keys())
def test_entries_update(curieapi, doc):
    vec = vec_documents[doc].copy()
    vec["id"] = "new-id"
    with pytest.raises(NotFoundError):
        r = curieapi.entries.update("pytest", doc, vec["id"], body=vec)
    r = curieapi.entries.create("pytest", doc, body=vec)
    assert r.status_code == 200
    r = curieapi.entries.update("pytest", doc, vec["id"], body=vec)
    assert r.status_code == 200
    r = curieapi.entries.get("pytest", doc, vec["id"])
    assert r.status_code == 200
    assert r.body == vec
    old_id = vec["id"]
    vec["id"] = "new-id-2"
    r = curieapi.entries.update("pytest", doc, old_id, body=vec)
    assert r.status_code == 200
    with pytest.raises(NotFoundError):
        r = curieapi.entries.get("pytest", doc, old_id)
    r = curieapi.entries.get("pytest", doc, vec["id"])
    assert r.status_code == 200
    assert r.body == vec


@pytest.mark.parametrize("doc", vec_documents.keys())
def test_entries_delete(curieapi, doc):
    r = curieapi.documents.get("pytest", doc)
    assert r.status_code == 200
    rlist = r.body
    for e in rlist:
        r = curieapi.entries.get("pytest", doc, e["id"])
        assert r.status_code == 200
        r = curieapi.entries.delete("pytest", doc, e["id"])
        assert r.status_code == 200
        with pytest.raises(NotFoundError):
            r = curieapi.entries.get("pytest", doc, e["id"])
        with pytest.raises(NotFoundError):
            r = curieapi.entries.delete("pytest", doc, e["id"])
