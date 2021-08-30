import pydash as _
from flask_restplus import fields

def vconvert(conf_type_name, vfrom, invert=False):
    """
    Convert configuration types terminology from demand API version to
    the actual one. It is needed to support multiple API versions in parallel.

    Args:
        conf_type_name (string): Configuration type to convert.
        vfrom (string): Version of the API from which to convert.
        invert (boolean): Inverts to return api version name from backend name.

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

def dict_to_path_value(map, path='', starting_path_list=None):
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
            new_path = '{}.{}'.format(path, key) if path \
                else '{}'.format(key)
            dict_to_path_value(value, new_path, starting_path_list)
    return starting_path_list
