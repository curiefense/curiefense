import codecs
import base64
import json
import pydash as _

DOCUMENTS_PATH = {
    "ratelimits": "config/json/limits.json",
    "urlmaps": "config/json/urlmap.json",
    "wafrules": "config/json/waf-signatures.json",
    "wafpolicies": "config/json/waf-profiles.json",
    "aclpolicies": "config/json/acl-profiles.json",
    "tagrules": "config/json/globalfilter-lists.json",
    "flowcontrol": "config/json/flow-control.json",
}

BLOBS_PATH = {
    "geolite2asn": "config/maxmind/GeoLite2-ASN.mmdb",
    "geolite2country": "config/maxmind/GeoLite2-Country.mmdb",
    "geolite2city": "config/maxmind/GeoLite2-City.mmdb",
}

BLOBS_BOOTSTRAP = {
    "geolite2asn": b"",
    "geolite2country": b"",
    "geolite2city": b"",
}


def vconvert(conf_type_name, vfrom):
    """
    Convert configuration types terminology from demand API version to
    the actual one. It is needed to support multiple API versions in parallel.

    Args:
        conf_type_name (string): Configuration type to convert.
        vfrom (string): Version of the API from which to convert.

    Returns
        string: converted conf type
    """
    apimap = {
        "v1": {
            "urlmaps": "securitypolicies",
            "wafrules": "contentfilterrules",
            "wafpolicies": "contentfilterprofiles",
            "aclpolicies": "aclprofiles",
            "tagrules": "globalfilters",
            "flowcontrol": "flowcontrolpolicies",
        }
    }

    return _.get(apimap, f"{vfrom}.{conf_type_name}", conf_type_name)


def jblob2bytes(jblob):
    fmt = jblob["format"]
    jraw = jblob["blob"]
    if fmt == "json":
        return json.dumps(jraw).encode("utf8")
    elif fmt == "string":
        return jraw.encode("utf8")
    elif fmt == "base64" or fmt.endswith("+base64"):
        jraw = codecs.decode(jraw.encode("utf8"), "base64")
        if "+" in fmt:
            cmp, b = fmt.rsplit("+", 1)
            if cmp not in ["zip", "bz2"]:
                raise Exception("unknown blob format: [%s]" % fmt)
            jraw = codecs.decode(jraw, cmp)
        return jraw
    raise Exception("unknown blob format: [%s]" % fmt)


def bytes2jblob(b, fmthint=None):
    try:
        if fmthint == "json":
            c = json.loads(b.decode("utf-8"))
            return {"format": "json", "blob": c}
    except:
        pass
    compb = codecs.encode(b, "bz2")
    if len(compb) < len(b):
        b = compb
        fmt = "bz2+base64"
    else:
        fmt = "base64"
    bl = base64.b64encode(b).decode("utf-8")
    return {"format": fmt, "blob": bl}
