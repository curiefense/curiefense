import pytest
from curieconf import utils
import json
import codecs
import base64
from flask_restplus import fields, model

binvec_hex = (
    "b70a1da09a4998bd56b083d76bf528053c9b924bbb07168792151a5a177bbaa232949a8600bcb2"
    + "5fffd487db3602aa77a5ac96441739be889f614f8e24cef51e487b36e4e2659a12b5c6de8cf0cd"
)

binvec = codecs.decode(binvec_hex, "hex")
binvec_b64 = base64.b64encode(binvec).decode("utf-8")
binvec_b64_nl = codecs.encode(binvec, "base64").decode("utf-8")
binvec_zip = base64.b64encode(codecs.encode(binvec, "zip")).decode("utf-8")
binvec_bz2 = base64.b64encode(codecs.encode(binvec, "bz2")).decode("utf-8")

jsonvec = [{"foo": "bar", "test": 6}, 42, True, "foobarboofar"]


@pytest.mark.parametrize(
    "fmt,blob",
    [
        ("base64", binvec_b64),
        ("base64", binvec_b64_nl),
        ("bz2+base64", binvec_bz2),
        ("zip+base64", binvec_zip),
    ],
)
def test_jblob2bytes_bin(fmt, blob):
    res = utils.jblob2bytes(
        {
            "format": fmt,
            "blob": blob,
        }
    )
    assert res == binvec


def test_jblob2bytes_json():
    res = utils.jblob2bytes({"format": "json", "blob": jsonvec})
    decjson = json.loads(res.decode("utf-8"))
    assert decjson == jsonvec


def test_bytes2jblob_json():
    vec = json.dumps(jsonvec).encode("utf8")
    res = utils.bytes2jblob(vec, fmthint="json")
    assert res == {"format": "json", "blob": jsonvec}

    vec_b64 = base64.b64encode(vec).decode("utf8")
    res = utils.bytes2jblob(vec)
    assert res == {"format": "base64", "blob": vec_b64}

    vec = b'{ "abc": 456, "broken json }'
    vec_b64 = base64.b64encode(vec).decode("utf8")
    res = utils.bytes2jblob(vec, fmthint="json")
    assert res == {"format": "base64", "blob": vec_b64}


def test_bytes2jblob_json():
    vec = b"A" * 500
    res = utils.bytes2jblob(vec)
    assert res == {
        "format": "bz2+base64",
        "blob": "QlpoOTFBWSZTWYtV77YAAACEAKAAIAggACEmQZioDi7kinChIRar32w=",
    }
    res2 = utils.jblob2bytes(res)
    assert res2 == vec

def test_vconvert():
    assert utils.vconvert("urlmaps", "v1", False) == "securitypolicies"
    assert utils.vconvert("wafrules", "v1", False) == "contentfilterrules"
    assert utils.vconvert("wafpolicies", "v1", False) == "contentfilterprofiles"
    assert utils.vconvert("aclpolicies", "v1", False) == "aclprofiles"
    assert utils.vconvert("tagrules", "v1", False) == "globalfilters"
    assert utils.vconvert("flowcontrol", "v1", False) == "flowcontrolpolicies"
    assert utils.vconvert("securitypolicies", "v1", True) == "urlmaps"
    assert utils.vconvert("contentfilterrules", "v1", True) == "wafrules"
    assert utils.vconvert("contentfilterprofiles", "v1", True) == "wafpolicies"
    assert utils.vconvert("aclprofiles", "v1", True) == "aclpolicies"
    assert utils.vconvert("globalfilters", "v1", True) == "tagrules"
    assert utils.vconvert("flowcontrolpolicies", "v1", True) == "flowcontrol"
    assert utils.vconvert("something", "v1", False) == "something"

def test_model_invert_names():
    mod1 = model.Model("test", {
            "test":fields.String(attribute="test2")
    })
    res = utils.model_invert_names(mod1)
    assert res.name == mod1.name and type(res['test2']) is fields.String \
        and res['test2'].attribute == 'test'

    mod2 = model.Model("test", {
            "test":fields.Nested(mod1, attribute="test2")
    })
    res = utils.model_invert_names(mod2)
    assert res.name == mod2.name \
        and type(res['test2']) is fields.Nested \
        and res['test2'].attribute == 'test' \
        and type(res['test2'].model['test2']) is fields.String \
        and res['test2'].model['test2'].attribute == 'test'

    mod3 = model.Model("test", {
            "test":fields.List(mod1, attribute="test2")
    })
    res = utils.model_invert_names(mod3)
    assert res.name == mod3.name \
        and type(res['test2']) is fields.List \
        and res['test2'].attribute == 'test' \
        and type(res['test2'].container['test2']) is fields.String \
        and res['test2'].container['test2'].attribute == 'test'

    mod4 = model.Model("test", {
            "test*":fields.Wildcard(mod2, attribute="test2*")
    })
    res = utils.model_invert_names(mod4)
    assert res.name == mod4.name \
        and type(res['test2']) is fields.Wildcard \
        and res['test2'].attribute == 'test' \
        and type(res['test2'].container['test2']) is fields.Nested \
        and res['test2'].container['test2'].attribute == 'test' \
        and type(res['test2'].container['test2'].model['test2']) is fields.String \
        and res['test2'].container['test2'].model['test2'].attribute == 'test'

def test_dict_to_path_value():
    assert utils.dict_to_path_value({}) == []
    assert utils.dict_to_path_value({'a':1,'b':{'b':2}}) == [{'path': 'a', 'value': 1}, {'path': 'b.b', 'value': 2}]
    assert utils.dict_to_path_value({'a':1,'b':{'b':{'b': 2}}}) == [{'path': 'a', 'value': 1}, {'path': 'b.b.b', 'value': 2}]
    assert utils.dict_to_path_value({'a':1,'b':{'b':2}, 'c':{'c':{'c': 3}}}) == [{'path': 'a', 'value': 1}, {'path': 'b.b', 'value': 2},  {'path': 'c.c.c', 'value': 3}]