import codecs
import base64
import json

DOCUMENTS_PATH = {
    "ratelimits": "config/json/limits.json",
    "urlmaps": "config/json/urlmap.json",
    "wafrules": "config/json/waf-signatures.json",
    "wafpolicies": "config/json/waf-profiles.json",
    "aclpolicies": "config/json/acl-profiles.json",
    "tagrules": "config/json/profiling-lists.json",
    "flowcontrol": "config/json/flow-control.json",
}

BLOBS_PATH = {
    "geolite2asn": "config/maxmind/GeoLite2-ASN.mmdb",
    "geolite2country": "config/maxmind/GeoLite2-Country.mmdb",
    "geolite2city": "config/maxmind/GeoLite2-City.mmdb",
}

BLOBS_BOOTSTRAP = {
    "geolite2asn"     : b"",
    "geolite2country" : b"",
    "geolite2city" : b"",
}

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
            cmp,b = fmt.rsplit("+",1)
            if cmp not in ["zip", "bz2"]:
                raise Exception("unknown blob format: [%s]" % fmt )
            jraw = codecs.decode(jraw, cmp)
        return jraw
    raise Exception("unknown blob format: [%s]" % fmt )


def bytes2jblob(b, fmthint=None):
    try:
        if fmthint == "json":
            c = json.loads(b.decode("utf-8"))
            return { "format": "json", "blob": c}
    except:
        pass
    compb = codecs.encode(b, "bz2")
    if len(compb) < len(b):
        b = compb
        fmt = "bz2+base64"
    else:
        fmt = "base64"
    bl = base64.b64encode(b).decode("utf-8")
    return  { "format": fmt, "blob": bl }
