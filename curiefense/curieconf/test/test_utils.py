import pytest
from curieconf import utils
import json
import codecs
import base64


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
