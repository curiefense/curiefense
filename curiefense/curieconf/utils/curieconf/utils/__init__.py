from __future__ import nested_scopes
import codecs
import base64
import json

import pydash as _
from flask_restplus import fields

DOCUMENTS_PATH = {
    "ratelimits": "config/json/limits.json",
    "securitypolicies": "config/json/securitypolicy.json",
    "contentfilterrules": "config/json/contentfilter-rules.json",
    "contentfilterprofiles": "config/json/contentfilter-profiles.json",
    "aclprofiles": "config/json/acl-profiles.json",
    "globalfilters": "config/json/globalfilter-lists.json",
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


def vconvert(conf_type_name, vfrom, invert=False):
    """
    Convert configuration types terminology from demand API version to
    the actual one. It is needed to support multiple API versions in parallel.

    Args:
        conf_type_name (string): Configuration type to convert.
        vfrom (string): Version of the API from which to convert.
        invert (boolean): return name in requested API version by the actual API name.

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

    if invert:
        for key in apimap.keys():
            apimap[key] = _.objects.invert(apimap[key])

    return _.get(apimap, f"{vfrom}.{conf_type_name}", conf_type_name)


def _field_invert_names(field):
    """
    Helper function to recurse over child fields incase of Nested/Wildcard/List fields.

    Args:
        field (fields.Raw): field to recurse. Being mutated.

    Returns
        fields.Raw: converted field
    """

    if isinstance(field, fields.Nested):
        field.model = model_invert_names(field.model)
    elif isinstance(field, fields.List) or isinstance(field, fields.Wildcard):
        field.container = _field_invert_names(field.container)
    return field


def model_invert_names(model):
    """
    Invert key names in a model using fields attribute if exists.

    Args:
        model (Model): model to invert.

    Returns
        Model: inverted model
    """

    mod = model.clone(model.name)
    for key in list(mod):
        _field_invert_names(mod[key])
        if mod[key].attribute:
            new_key = mod[key].attribute
            mod[new_key] = mod[key]
            mod[new_key].attribute = key
            del mod[key]
    return mod


def dict_to_path_value(map, path="", starting_path_list=None):
    """
    Creates a list of path and value dicts for a map.

    Args:
        map (dict): dictionary to create the list for.
        path (String): current path, used for recursion.
        starting_path_list (List): list to append new values to, default None to return a new list.

    Returns
        List: list of path and value pairs
    """

    if starting_path_list == None:
        starting_path_list = []
    if not isinstance(map, dict):
        starting_path_list.append({"path": path, "value": map})
    else:
        for key, value in sorted(map.items()):
            new_path = "{}.{}".format(path, key) if path else "{}".format(key)
            dict_to_path_value(value, new_path, starting_path_list)
    return starting_path_list
