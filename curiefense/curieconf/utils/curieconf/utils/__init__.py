from __future__ import nested_scopes
import codecs
import base64
import json

import pydash
from flask_restx import fields

DOCUMENTS_PATH = {
    "ratelimits": "config/json/limits.json",
    "securitypolicies": "config/json/securitypolicy.json",
    "contentfilterrules": "config/json/contentfilter-rules.json",
    "contentfiltergroups": "config/json/contentfilter-groups.json",
    "contentfilterprofiles": "config/json/contentfilter-profiles.json",
    "aclprofiles": "config/json/acl-profiles.json",
    "globalfilters": "config/json/globalfilter-lists.json",
    "flowcontrol": "config/json/flow-control.json",
    "actions": "config/json/actions.json",
    "dynamicrules": "config/json/dynamic-rules.json",
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
            "wafgroups": "contentfiltergroups",
            "wafpolicies": "contentfilterprofiles",
            "aclpolicies": "aclprofiles",
            "tagrules": "globalfilters",
            "flowcontrol": "flowcontrolpolicies",
        }
    }

    return pydash.get(apimap, f"{vfrom}.{conf_type_name}", conf_type_name)


def backend_v1_rl_convert(backend_document):
    """
    Convert Rate Limit from backend into V1 API format.
    backend API supports multiple thresholds (action + limit) for a one Rate Limit.
    The function takes only the first threshold element from the array.
    To support backward compatibility we limit V1 users to only one threshold.
    Args:
        backend_document (dict): RL configuration document in backend format
    Returns:
        dict: converted to V1 format
    """
    v1 = backend_document.copy()
    v1["limit"] = pydash.get(v1, "thresholds[0].limit", "")
    v1["action"] = pydash.get(v1, "thresholds[0].action", {"type": "default"})
    del v1["thresholds"]
    return v1


def v1_backend_rl_convert(v1_document):
    """
    Convert Rate Limit from V1 into backend API format.
    backend API supports multiple thresholds (action + limit) for a one Rate Limit.
    But V1 accepts only one limit and action. The function takes limit and action
    and add it as a one element of thresholds array.
    To support backward compatibility we limit V1 users to only one threshold.
    Args:
        v1_document (dict): RL configuration document in V1 format
    Returns:
        dict: converted to backend format
    """
    backend = v1_document.copy()
    pydash.set_(
        backend,
        "thresholds[0]",
        {"limit": backend["limit"], "action": backend["action"]},
    )
    del backend["limit"]
    del backend["action"]
    return backend


def backend_v1_cfp_convert(backend_document):
    """
    Convert Content Filter Profiles from backend into V1 API format.
    backend API supports both groups and rules as exclusions, while v1 only support rules.
    The function takes only the rules and transforms the values to 1 as expected in v1.
    v1 does not support returning groups.
    Args:
        backend_document (dict): Content Filter Profiles configuration document in backend format
    Returns:
        dict: converted to V1 format
    """
    v1 = backend_document.copy()
    for section in _get_existing_keys(v1, ["args", "headers", "cookies"]):
        for section_key in _get_existing_keys(section, ["names", "regex"]):
            for i in range(len(section_key)):
                if section_key[i].get("exclusions"):
                    section_key[i]["exclusions"] = {
                        rule_id: 1
                        for rule_id, value in section_key[i]["exclusions"].items()
                        if value == "rule"
                    }
    return v1


def v1_backend_cfp_convert(v1_document):
    """
    Convert Content Filter Profiles from V1 into backend API format.
    backend API supports both groups and rules as exclusions, while v1 only support rules.
    The function takes all rules and transforms the values to "rule" as expected on backend.
    v1 does not support recieving groups.
    Args:
        v1_document (dict): Content Filter Profiles configuration document in V1 format
    Returns:
        dict: converted to backend format
    """
    backend = v1_document.copy()
    for section in _get_existing_keys(backend, ["args", "headers", "cookies"]):
        for section_key in _get_existing_keys(section, ["names", "regex"]):
            for i in range(len(section_key)):
                if section_key[i].get("exclusions"):
                    section_key[i]["exclusions"] = {
                        rule_id: "rule"
                        for rule_id, value in section_key[i]["exclusions"].items()
                        if value == 1
                    }
    return backend


def _get_existing_keys(target, keys):
    return list(filter(None, map(target.get, keys)))


def vconfigconvert(conf_type_name, document, vfrom, vto):
    """
    Convert configuration documents structure from between API versions formats.
    Args:
        conf_type_name (string): Configuration type to convert.
        document (dict): configuration document
        vfrom (string): Version of the API from which to convert.
        vfrom (string): Version of the API to which version to convert.
    Returns:
        dict: converted config document or the original one if nothing to convert.
    """
    apimap = {
        "v1_backend": {
            "ratelimits": v1_backend_rl_convert,
            "wafpolicies": v1_backend_cfp_convert,
        },
        "backend_v1": {
            "ratelimits": backend_v1_rl_convert,
            "wafpolicies": backend_v1_cfp_convert,
        },
    }

    def do_not_convert(document):
        return document

    convertfunc = pydash.get(apimap, f"{vfrom}_{vto}.{conf_type_name}", do_not_convert)
    return convertfunc(document)


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
            apimap[key] = pydash.objects.invert(apimap[key])

    return pydash.get(apimap, f"{vfrom}.{conf_type_name}", conf_type_name)


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
