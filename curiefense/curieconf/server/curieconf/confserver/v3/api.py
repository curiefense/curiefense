import datetime
import logging
import typing
import os
import requests
import json
import jsonschema
import bleach
from jsonschema import validate
from pathlib import Path
from enum import Enum
from typing import Optional, List, Union
from fastapi import Request, HTTPException, APIRouter, Header, Depends
from pydantic import BaseModel, Field, StrictStr, StrictBool, StrictInt, Extra, HttpUrl
from urllib.parse import unquote

from curieconf.utils import cloud


logger = logging.getLogger("confserver")

# monkey patch to force RestPlus to use Draft3 validator to benefit from "any" json type
jsonschema.Draft4Validator = jsonschema.Draft3Validator

# TODO: TEMP DEFINITIONS
router = APIRouter(prefix="/api/v3")
options = {}
val = os.environ.get("CURIECONF_TRUSTED_USERNAME_HEADER", None)
if val:
    options["trusted_username_header"] = val
val = os.environ.get("CURIECONF_TRUSTED_EMAIL_HEADER", None)
if val:
    options["trusted_email_header"] = val


##############
### MODELS ###
##############


### Models for documents
anyTypeUnion = Union[int, float, bool, object, list, None]
anyOp = Optional[object]
anyType = ["number", "string", "boolean", "object", "array", "null"]


class Threshold(BaseModel):
    limit: StrictInt
    action: StrictStr


class Limit(BaseModel):
    id: StrictStr
    name: StrictStr
    description: Optional[StrictStr]
    global_: StrictBool = Field(alias="global")
    active: StrictBool
    timeframe: StrictInt
    thresholds: Optional[List[Threshold]]
    include: typing.Any
    exclude: typing.Any
    key: anyTypeUnion
    pairwith: typing.Any
    tags: Optional[List[StrictStr]]


# securitypolicy
class SecProfileMap(BaseModel):
    id: StrictStr
    name: StrictStr
    description: Optional[StrictStr]
    match: StrictStr
    acl_profile: StrictStr
    acl_active: StrictBool
    content_filter_profile: StrictStr
    content_filter_active: StrictBool
    limit_ids: Optional[list]


class SecurityPolicy(BaseModel):
    id: StrictStr
    name: StrictStr
    description: Optional[StrictStr]
    tags: Optional[List[StrictStr]]
    match: StrictStr
    session: anyTypeUnion
    session_ids: anyTypeUnion
    map: Optional[List[SecProfileMap]]


# content filter rule


class ContentFilterRule(BaseModel):
    id: StrictStr
    name: StrictStr
    msg: StrictStr
    operand: StrictStr
    severity: StrictInt
    certainity: StrictInt
    category: StrictStr
    subcategory: StrictStr
    risk: StrictInt
    tags: Optional[List[StrictStr]]
    description: Optional[StrictStr]


# content filter profile
class ContentFilterProfile(BaseModel):
    id: StrictStr
    name: StrictStr
    description: Optional[StrictStr]
    ignore_alphanum: StrictBool
    args: typing.Any
    headers: typing.Any
    cookies: typing.Any
    path: typing.Any
    allsections: typing.Any
    decoding: typing.Any
    masking_seed: StrictStr
    graphql_path: Optional[StrictStr]
    content_type: Optional[List[StrictStr]]
    active: Optional[List[StrictStr]]
    report: Optional[List[StrictStr]]
    ignore: Optional[List[StrictStr]]
    tags: Optional[List[StrictStr]]
    action: Optional[StrictStr]
    ignore_body: StrictBool


# aclprofile
class ACLProfile(BaseModel):
    id: StrictStr
    name: StrictStr
    description: Optional[StrictStr]
    allow: Optional[List[StrictStr]]
    allow_bot: Optional[List[StrictStr]]
    deny_bot: Optional[List[StrictStr]]
    passthrough: Optional[List[StrictStr]]
    deny: Optional[List[StrictStr]]
    force_deny: Optional[List[StrictStr]]
    tags: Optional[List[StrictStr]]
    action: Optional[StrictStr]


# Global Filter
class GlobalFilter(BaseModel):
    id: StrictStr
    name: StrictStr
    source: StrictStr
    mdate: StrictStr
    description: Optional[StrictStr]
    active: StrictBool
    action: typing.Any
    tags: Optional[List[StrictStr]]
    rule: anyTypeUnion


# Flow Control


class FlowControl(BaseModel):
    id: StrictStr
    name: StrictStr
    timeframe: StrictInt
    key: List[typing.Any]
    sequence: List[typing.Any]
    tags: Optional[List[StrictStr]]
    include: Optional[List[StrictStr]]
    exclude: Optional[List[StrictStr]]
    description: Optional[StrictStr]
    active: StrictBool


# Action


class Action(BaseModel):
    id: StrictStr
    name: StrictStr
    description: Optional[StrictStr]
    tags: List[StrictStr]
    params: typing.Any
    type_: StrictStr = Field(alias="type")

    class Config:
        fields = {"_type": "type"}


# Virtual Tag
class VirtualTag(BaseModel):
    id: StrictStr
    name: StrictStr
    description: Optional[StrictStr]
    match: List[typing.Any]


# custom
class Custom(BaseModel, extra=Extra.allow):
    id: StrictStr
    name: StrictStr


### mapping from doc name to model

models = {
    "ratelimits": Limit,
    "securitypolicies": SecurityPolicy,
    "contentfilterrules": ContentFilterRule,
    "contentfilterprofiles": ContentFilterProfile,
    "aclprofiles": ACLProfile,
    "globalfilters": GlobalFilter,
    "flowcontrol": FlowControl,
    "actions": Action,
    "virtualtags": Custom,
    "custom": Custom,
}


### Other models
class DocumentMask(BaseModel, extra=Extra.allow):
    id: StrictStr
    name: StrictStr
    description: Optional[StrictStr]
    map: Optional[List[SecProfileMap]]
    include: Optional[List[typing.Any]]
    exclude: Optional[List[typing.Any]]
    tags: Optional[List[StrictStr]]
    active: Optional[typing.Any]
    action: typing.Any
    sequence: Optional[List[typing.Any]]
    timeframe: Optional[StrictInt]
    thresholds: Optional[List[Threshold]]
    pairwith: typing.Any
    content_type: Optional[List[StrictStr]]
    params: typing.Any
    decoding: typing.Any
    category: Optional[StrictStr]
    subcategory: Optional[StrictStr]
    risk: Optional[StrictInt]
    allow: Optional[List[StrictStr]]
    allow_bot: Optional[List[StrictStr]]
    deny_bot: Optional[List[StrictStr]]
    passthrough: Optional[List[StrictStr]]
    deny: Optional[List[StrictStr]]
    force_deny: Optional[List[StrictStr]]
    match: Optional[StrictStr]
    type_: Optional[StrictStr] = Field(alias="type")
    star_: Optional[List[typing.Any]] = Field(alias="*")


class VersionLog(BaseModel, extra=Extra.allow):
    version: Optional[StrictStr]
    # TODO - dt_format="iso8601"
    date: Optional[datetime.datetime]


class Meta(BaseModel):
    id: StrictStr
    description: StrictStr
    date: Optional[datetime.datetime]
    logs: Optional[List[VersionLog]] = []
    version: Optional[StrictStr]


class BlobEntry(BaseModel):
    format: StrictStr
    blob: anyTypeUnion


class BlobListEntry(BaseModel):
    name: Optional[StrictStr]


class DocumentListEntry(BaseModel):
    name: Optional[StrictStr]
    entries: Optional[StrictInt]


class ConfigDocuments(BaseModel):
    ratelimits: Optional[List[models["ratelimits"]]] = []
    securitypolicies: Optional[List[models["securitypolicies"]]] = []
    contentfilterrules: Optional[List[models["contentfilterrules"]]] = []
    contentfilterprofiles: Optional[List[models["contentfilterprofiles"]]] = []
    aclprofiles: Optional[List[models["aclprofiles"]]] = []
    globalfilters: Optional[List[models["globalfilters"]]] = []
    flowcontrol: Optional[List[models["flowcontrol"]]] = []
    actions: Optional[List[models["actions"]]] = []
    virtualtags: Optional[List[models["virtualtags"]]] = []
    custom: Optional[List[models["custom"]]] = []


class ConfigBlobs(BaseModel):
    geolite2asn: Optional[BlobEntry]
    geolite2country: Optional[BlobEntry]
    geolite2city: Optional[BlobEntry]
    customconf: Optional[BlobEntry]


class ConfigDeleteBlobs(BaseModel):
    geolite2asn: Optional[StrictBool]
    geolite2country: Optional[StrictBool]
    geolite2city: Optional[StrictBool]
    customconf: Optional[StrictBool]


class Config(BaseModel):
    meta: Meta = {}
    documents: ConfigDocuments = {}
    blobs: ConfigBlobs = {}
    delete_documents: ConfigDocuments = {}
    delete_blobs: ConfigDeleteBlobs = {}


class Edit(BaseModel):
    path: StrictStr
    value: StrictStr


class BasicEntry(BaseModel):
    id: StrictStr
    name: StrictStr
    description: Optional[StrictStr]


### Publish


class Bucket(BaseModel):
    name: StrictStr
    url: StrictStr


### Git push & pull


class GitUrl(BaseModel):
    giturl: StrictStr


### Db
class DB(BaseModel):
    pass


### Document Schema validation


def validateJson(json_data, schema_type):
    try:
        validate(instance=json_data, schema=schema_type_map[schema_type])
    except jsonschema.exceptions.ValidationError as err:
        print(str(err))
        return False, str(err)
    return True, ""


### DB Schema validation


def validateDbJson(json_data, schema):
    try:
        validate(instance=json_data, schema=schema)
    except jsonschema.exceptions.ValidationError as err:
        print(str(err))
        return False
    return True


### Set git actor according to config & defined HTTP headers


def get_gitactor(request):
    email, username = "", ""
    email_header = request.app.options.get("trusted_email_header", None)
    if email_header:
        email = request.headers.get(email_header, "")
    username_header = request.app.options.get("trusted_username_header", None)
    if username_header:
        username = request.headers.get(username_header, "")
    return request.app.backend.prepare_actor(username, email)


base_path = Path(__file__).parent
# base_path = "/etc/curiefense/json/"
acl_profile_file_path = (base_path / "./json/acl-profile.schema").resolve()
with open(acl_profile_file_path) as json_file:
    acl_profile_schema = json.load(json_file)
ratelimits_file_path = (base_path / "./json/rate-limits.schema").resolve()
with open(ratelimits_file_path) as json_file:
    ratelimits_schema = json.load(json_file)
securitypolicies_file_path = (base_path / "./json/security-policies.schema").resolve()
with open(securitypolicies_file_path) as json_file:
    securitypolicies_schema = json.load(json_file)
content_filter_profile_file_path = (
    base_path / "./json/content-filter-profile.schema"
).resolve()
with open(content_filter_profile_file_path) as json_file:
    content_filter_profile_schema = json.load(json_file)
globalfilters_file_path = (base_path / "./json/global-filters.schema").resolve()
with open(globalfilters_file_path) as json_file:
    globalfilters_schema = json.load(json_file)
flowcontrol_file_path = (base_path / "./json/flow-control.schema").resolve()
with open(flowcontrol_file_path) as json_file:
    flowcontrol_schema = json.load(json_file)
content_filter_rule_file_path = (
    base_path / "./json/content-filter-rule.schema"
).resolve()
with open(content_filter_rule_file_path) as json_file:
    content_filter_rule_schema = json.load(json_file)
action_file_path = (base_path / "./json/action.schema").resolve()
with open(action_file_path) as json_file:
    action_schema = json.load(json_file)
virtualtag_file_path = (base_path / "./json/virtual-tags.schema").resolve()
with open(virtualtag_file_path) as json_file:
    virtual_tags_schema = json.load(json_file)
custom_file_path = (base_path / "./json/custom.schema").resolve()
with open(custom_file_path) as json_file:
    custom_schema = json.load(json_file)
schema_type_map = {
    "ratelimits": ratelimits_schema,
    "securitypolicies": securitypolicies_schema,
    "contentfilterprofiles": content_filter_profile_schema,
    "aclprofiles": acl_profile_schema,
    "globalfilters": globalfilters_schema,
    "flowcontrol": flowcontrol_schema,
    "contentfilterrules": content_filter_rule_schema,
    "actions": action_schema,
    "virtualtags": virtual_tags_schema,
    "custom": custom_schema,
}


class Tags(Enum):
    congifs = "configs"
    db = "db"
    tools = "tools"


################
### CONFIGS ###
################


@router.get(
    "/configs/",
    tags=[Tags.congifs],
    response_model=List[Meta],
    response_model_exclude_unset=True,
)
async def configs_get(request: Request):
    """Get the detailed list of existing configurations"""
    res = request.app.backend.configs_list()
    return res


@router.post("/configs/", tags=[Tags.congifs])
async def configs_post(config: Config, request: Request):
    """Create a new configuration"""
    data = await request.json()
    return request.app.backend.configs_create(data=data, actor=get_gitactor(request))


@router.get("/configs/{config}/", tags=[Tags.congifs])
async def config_get(config: str, request: Request):
    """Retrieve a complete configuration"""
    res = request.app.backend.configs_get(config)
    return {key: res[key] for key in Config.__fields__.keys() if key in res}


@router.post("/configs/{config}/", tags=[Tags.congifs])
async def config_post(config: str, m_config: Config, request: Request):
    "Create a new configuration. Configuration name in URL overrides configuration in POST data"
    data = await request.json()
    return request.app.backend.configs_create(data, config, get_gitactor(request))


@router.put("/configs/{config}/", tags=[Tags.congifs])
async def config_put(config: str, meta: Meta, request: Request):
    """Update an existing configuration"""
    data = await request.json()
    return request.app.backend.configs_update(config, data, get_gitactor(request))


@router.delete("/configs/{config}/", tags=[Tags.congifs])
async def config_delete(config: str, request: Request):
    """Delete a configuration"""
    return request.app.backend.configs_delete(config)


@router.post("/configs/{config}/clone/", tags=[Tags.congifs])
async def config_clone_post(config: str, meta: Meta, request: Request):
    """Clone a configuration. New name is provided in POST data"""
    data = await request.json()
    return request.app.backend.configs_clone(config, data)


@router.post("/configs/{config}/clone/{new_name}/", tags=[Tags.congifs])
async def config_clone_name_post(
    config: str, new_name: str, meta: Meta, request: Request
):
    """Clone a configuration. New name is provided URL"""
    data = await request.json()
    return request.app.backend.configs_clone(config, data, new_name)


# Meant to mimick flask https://flask-restx.readthedocs.io/en/latest/mask.html functionality
# filtering only keys requested in X-Fields header. (works only for non-nested keys)
def filter_x_fields(res, x_fields):
    if x_fields.startswith(("[", "{", "(")):
        x_fields = x_fields[1:-1]
    x_fields = x_fields.replace(" ", "")
    fields = x_fields.split(",")
    if isinstance(res, list):
        return [{field: r[field] for field in fields if field in r} for r in res]
    else:
        return {field: res[field] for field in fields if field in res}


@router.get(
    "/configs/{config}/v/",
    tags=[Tags.congifs],
)
async def config_list_version_get(
    config: str,
    request: Request,
    x_fields: Optional[str] = Header(default=None, alias="X-Fields"),
):
    """Get all versions of a given configuration"""
    res = request.app.backend.configs_list_versions(config)

    if x_fields:
        res = filter_x_fields(res, x_fields)
    return res


@router.get("/configs/{config}/v/{version}/", tags=[Tags.congifs])
async def config_version_get(config: str, version: str, request: Request):
    """Retrieve a specific version of a configuration"""
    return request.app.backend.configs_get(config, version)


@router.put("/configs/{config}/v/{version}/revert/", tags=[Tags.congifs])
async def config_revert_put(config: str, version: str, request: Request):
    """Create a new version for a configuration from an old version"""
    return request.app.backend.configs_revert(config, version, get_gitactor(request))


#############
### Blobs ###
#############


@router.get(
    "/configs/{config}/b/", tags=[Tags.congifs], response_model=List[BlobListEntry]
)
async def blobs_resource_get(config: str, request: Request):
    """Retrieve the list of available blobs"""
    res = request.app.backend.blobs_list(config)
    return res


@router.get(
    "/configs/{config}/b/{blob}/", tags=[Tags.congifs], response_model=BlobEntry
)
async def blob_resource_get(config: str, blob: str, request: Request):
    """Retrieve a blob"""
    return request.app.backend.blobs_get(config, blob)


@router.post("/configs/{config}/b/{blob}/", tags=[Tags.congifs])
async def blob_resource_post(
    config: str, blob: str, blob_entry: BlobEntry, request: Request
):
    """Create a new blob"""
    b_entry = await request.json()
    return request.app.backend.blobs_create(
        config, blob, b_entry, get_gitactor(request)
    )


@router.put("/configs/{config}/b/{blob}/", tags=[Tags.congifs])
async def blob_resource_put(
    config: str, blob: str, blob_entry: BlobEntry, request: Request
):
    """upaate an existing blob"""
    b_entry = await request.json()

    return request.app.backend.blobs_update(
        config, blob, b_entry, get_gitactor(request)
    )


@router.delete("/configs/{config}/b/{blob}/", tags=[Tags.congifs])
async def blob_resource_delete(config: str, blob: str, request: Request):
    """Delete a blob"""
    return request.app.backend.blobs_delete(config, blob, get_gitactor(request))


@router.get("/configs/{config}/b/{blob}/v/", tags=[Tags.congifs])
async def blob_list_version_resource_get(
    config: str,
    blob: str,
    request: Request,
    x_fields: Optional[str] = Header(default=None, alias="X-Fields"),
):
    """Retrieve the list of versions of a given blob"""
    res = request.app.backend.blobs_list_versions(config, blob)
    if x_fields:
        res = filter_x_fields(res, x_fields)
    return res


@router.get(
    "/configs/{config}/b/{blob}/v/{version}/",
    tags=[Tags.congifs],
    response_model=BlobEntry,
)
async def blob_version_resource_get(
    config: str,
    blob: str,
    version: str,
    request: Request,
    x_fields: Optional[str] = Header(default=None, alias="X-Fields"),
):
    """Retrieve the given version of a blob"""

    res = request.app.backend.blobs_get(config, blob, version)
    if x_fields:
        res = filter_x_fields([res], x_fields)
    return res


@router.put("/configs/{config}/b/{blob}/v/{version}/revert/", tags=[Tags.congifs])
async def blob_revert_resource_put(
    config: str, blob: str, version: str, request: Request
):
    """Create a new version for a blob from an old version"""
    return request.app.backend.blobs_revert(
        config, blob, version, get_gitactor(request)
    )


#################
### DOCUMENTS ###
#################


@router.get(
    "/configs/{config}/d/", tags=[Tags.congifs], response_model=List[DocumentListEntry]
)
async def document_resource_get(config: str, request: Request):
    """Retrieve the list of existing documents in this configuration"""
    res = request.app.backend.documents_list(config)
    return res


@router.get("/configs/{config}/d/{document}/", tags=[Tags.congifs])
async def document_resource_get(
    config: str,
    document: str,
    request: Request,
    x_fields: Optional[str] = Header(default=None, alias="X-Fields"),
):
    """Get a complete document"""
    if document not in models:
        raise HTTPException(status_code=404, detail="document does not exist")
    res = request.app.backend.documents_get(config, document)

    if x_fields:
        res = filter_x_fields(res, x_fields)
    return res


async def _filter(data, keys):
    return {key: data[key] for key in switch_alias(keys) if key in data}


@router.post("/configs/{config}/d/{document}/", tags=[Tags.congifs])
async def document_resource_post(
    config: str, document: str, basic_entries: List[BasicEntry], request: Request
):
    """Create a new complete document"""
    if document not in models.keys():
        raise HTTPException(
            status_code=404,
            detail="document name is not one of the possible name in 'models' module",
        )

    as_dict = await request.json()
    if document == "custom":
        data = as_dict
    else:
        data = [
            await _filter(dict(entry), list(models[document].__fields__.keys()))
            for entry in as_dict
        ]
    for entry in data:
        isValid, err = validateJson(entry, document)
        if isValid is False:
            raise HTTPException(400, "schema mismatched: " + err)
    res = request.app.backend.documents_create(
        config, document, data, get_gitactor(request)
    )
    return res


@router.put("/configs/{config}/d/{document}/", tags=[Tags.congifs])
async def document_resource_put(
    config: str, document: str, basic_entries: List[BasicEntry], request: Request
):
    """Update an existing document"""
    if document not in models:
        raise HTTPException(status_code=404, detail="document does not exist")
    as_dict = await request.json()
    if document == "custom":
        data = as_dict
    else:
        data = [
            await _filter(entry, switch_alias(list(models[document].__fields__.keys())))
            for entry in as_dict
        ]
    for entry in data:
        isValid, err = validateJson(entry, document)
        if isValid is False:
            raise HTTPException(
                400, "schema mismatched for entry: " + str(entry) + "\n" + err
            )
    res = request.app.backend.documents_update(
        config, document, data, get_gitactor(request)
    )
    return res


@router.delete("/configs/{config}/d/{document}/", tags=[Tags.congifs])
async def document_resource_delete(config: str, document: str, request: Request):
    """Delete/empty a document"""
    if document not in models:
        raise HTTPException(404, "document does not exist")
    res = request.app.backend.documents_delete(config, document, get_gitactor(request))
    return res


@router.get(
    "/configs/{config}/d/{document}/v/",
    tags=[Tags.congifs],
)
async def document_list_version_resource_get(
    config: str, document: str, request: Request
):
    """Retrieve the existing versions of a given document"""
    if document not in models:
        raise HTTPException(404, "document does not exist")
    res = request.app.backend.documents_list_versions(config, document)
    return res


@router.get("/configs/{config}/d/{document}/v/{version}/", tags=[Tags.congifs])
async def document_version_resource_get(
    config: str, document: str, version: str, request: Request
):
    """Get a given version of a document"""
    if document not in models:
        raise HTTPException(404, "document does not exist")
    res = request.app.backend.documents_get(config, document, version)
    if document == "custom":
        return res
    else:
        return [
            {
                key: r[key]
                for key in switch_alias(list(models[document].__fields__.keys()))
                if key in r
            }
            for r in res
        ]


@router.put("/configs/{config}/d/{document}/v/{version}/revert/", tags=[Tags.congifs])
async def document_revert_resource_put(
    config: str, document: str, version: str, request: Request
):
    """Create a new version for a document from an old version"""
    return request.app.backend.documents_revert(
        config, document, version, get_gitactor(request)
    )


###############
### ENTRIES ###
###############


@router.get("/configs/{config}/d/{document}/e/", tags=[Tags.congifs])
async def entries_resource_get(config: str, document: str, request: Request):
    """Retrieve the list of entries in a document"""
    if document not in models:
        raise HTTPException(404, "document does not exist")
    res = request.app.backend.entries_list(config, document)
    return res  # XXX: marshal


@router.post("/configs/{config}/d/{document}/e/", tags=[Tags.congifs])
async def entries_resource_post(
    config: str, document: str, basic_entry: BasicEntry, request: Request
):
    """Create an entry in a document"""

    data_json = await request.json()
    if document not in models:
        raise HTTPException(404, "document does not exist")
    isValid, err = validateJson(data_json, document)
    if isValid:
        if document == "custom":
            res = request.app.backend.entries_create(
                config, document, data_json, get_gitactor(request)
            )
        else:
            keys = switch_alias(list(models[document].__fields__.keys()))
            data = {key: data_json[key] for key in keys if key in data_json}
            res = request.app.backend.entries_create(
                config, document, data, get_gitactor(request)
            )
        return res
    else:
        raise HTTPException(400, "schema mismatched: \n" + err)


def switch_alias(keys):
    return [key[:-1] if key.endswith("_") else key for key in keys]


@router.get("/configs/{config}/d/{document}/e/{entry}/", tags=[Tags.congifs])
async def entry_resource_get(config: str, document: str, entry: str, request: Request):
    """Retrieve an entry from a document"""
    if document not in models:
        raise HTTPException(404, "document does not exist")
    res = request.app.backend.entries_get(config, document, entry)
    if document == "custom":
        return res
    keys = switch_alias(list(models[document].__fields__.keys()))
    return {key: res[key] for key in keys if key in res}


@router.put("/configs/{config}/d/{document}/e/{entry}/", tags=[Tags.congifs])
async def entry_resource_put(
    config: str, document: str, entry: str, basic_entry: BasicEntry, request: Request
):
    """Update an entry in a document"""
    data_json = await request.json()
    if document not in models:
        raise HTTPException(404, "document does not exist")
    isValid, err = validateJson(data_json, document)
    if isValid:
        if document == "custom":
            data = data_json
        else:
            data = {
                key: data_json[key]
                for key in switch_alias(list(models[document].__fields__.keys()))
                if key in data_json
            }

        res = request.app.backend.entries_update(
            config, document, entry, data, get_gitactor(request)
        )
        return res
    else:
        raise HTTPException(400, "schema mismatched: \n" + err)


@router.delete("/configs/{config}/d/{document}/e/{entry}/", tags=[Tags.congifs])
async def entry_resource_delete(
    config: str, document: str, entry: str, request: Request
):
    """Delete an entry from a document"""
    if document not in models:
        raise HTTPException(404, "document does not exist")
    res = request.app.backend.entries_delete(
        config, document, entry, get_gitactor(request)
    )
    return res


@router.get("/configs/{config}/d/{document}/e/{entry}/v/", tags=[Tags.congifs])
async def entry_list_version_resource_get(
    config: str,
    document: str,
    entry: str,
    request: Request,
):
    """Get the list of existing versions of a given entry in a document"""
    if document not in models:
        raise HTTPException(404, "document does not exist")
    res = request.app.backend.entries_list_versions(config, document, entry)
    return res


@router.get(
    "/configs/{config}/d/{document}/e/{entry}/v/{version}/", tags=[Tags.congifs]
)
async def entry_version_resource_get(
    config: str, document: str, entry: str, version: str, request: Request
):
    """Get a given version of a document entry"""
    if document not in models:
        raise HTTPException(404, "document does not exist")
    res = request.app.backend.entries_get(config, document, entry, version)
    if document == "custom":
        return res
    keys = switch_alias(list(models[document].__fields__.keys()))
    return {key: res[key] for key in keys if key in res}


################
### Database ###
################

bootsrap_system_keys = ["publishinfo", "tags", "links", "dashboardsinfo", "user"]
preserved_namespace = "system"


@router.get("/db/", tags=[Tags.db])
async def db_resource_get(request: Request):
    """Get the list of existing namespaces"""
    return request.app.backend.ns_list()


@router.get("/db/v/", tags=[Tags.db])
async def db_query_resource_get(request: Request):
    """List all existing versions of namespaces"""
    return request.app.backend.ns_list_versions()


@router.get("/db/{nsname}/", tags=[Tags.db])
async def ns_resource_get(nsname: str, request: Request):
    """Get a complete namespace"""
    try:
        return request.app.backend.ns_get(nsname, version=None)
    except KeyError:
        raise HTTPException(404, "namespace [%s] does not exist" % nsname)


async def verify_namespace(nsname: str, request: Request):
    _db = await request.json()
    if nsname == preserved_namespace and set.intersection(
        set(_db.keys()), set(bootsrap_system_keys)
    ) != set(bootsrap_system_keys):
        raise HTTPException(
            500, f"'{nsname}' namespace must include {bootsrap_system_keys}"
        )


@router.post("/db/{nsname}/", tags=[Tags.db], dependencies=[Depends(verify_namespace)])
async def ns_resource_post(nsname: str, request: Request):
    """Create a non-existing namespace from data"""
    _db = await request.json()
    try:
        return request.app.backend.ns_create(nsname, _db, get_gitactor(request))
    except Exception:
        raise HTTPException(409, "namespace [%s] already exists" % nsname)


@router.put("/db/{nsname}/", tags=[Tags.db], dependencies=[Depends(verify_namespace)])
async def ns_resource_put(nsname: str, request: Request):
    """Merge data into a namespace"""
    _db = await request.json()

    return request.app.backend.ns_update(nsname, _db, get_gitactor(request))


@router.delete("/db/{nsname}/", tags=[Tags.db])
async def ns_resource_delete(nsname: str, request: Request):
    if nsname == preserved_namespace:
        raise HTTPException(403, f"the '{nsname}' namespace cannot be deleted")
    """Delete an existing namespace"""
    try:
        return request.app.backend.ns_delete(nsname, get_gitactor(request))
    except KeyError:
        raise HTTPException(409, "namespace [%s] does not exist" % nsname)


@router.get("/db/{nsname}/v/{version}/", tags=[Tags.db])
async def ns_version_resource_get(nsname: str, version: str, request: Request):
    """Get a given version of a namespace"""
    return request.app.backend.ns_get(nsname, version)


@router.put("/db/{nsname}/v/{version}/revert/", tags=[Tags.db])
async def ns_version_revert_resource_put(nsname: str, version: str, request: Request):
    """Create a new version for a namespace from an old version"""
    try:
        return request.app.backend.ns_revert(nsname, version, get_gitactor(request))
    except KeyError:
        raise HTTPException(
            404, "namespace [%s] version [%s] not found" % (nsname, version)
        )


@router.post("/db/{nsname}/q/", tags=[Tags.db])
async def ns_query_resource_post(nsname: str, request: Request):
    """Run a JSON query on the namespace and returns the results"""
    req_json = await request.json()
    return request.app.backend.ns_query(nsname, req_json)


@router.get("/db/{nsname}/k/", tags=[Tags.db])
async def keys_resource_get(nsname: str, request: Request):
    """List all keys of a given namespace"""
    return request.app.backend.key_list(nsname)


@router.get("/db/{nsname}/k/{key}/v/", tags=[Tags.db])
async def keys_list_versions_resource_get(nsname: str, key: str, request: Request):
    """Get all versions of a given key in namespace"""
    return request.app.backend.key_list_versions(nsname, key)


@router.get("/db/{nsname}/k/{key}/", tags=[Tags.db])
async def key_resource_get(nsname: str, key: str, request: Request):
    """Retrieve a given key's value from a given namespace"""
    return request.app.backend.key_get(nsname, key)


@router.put("/db/{nsname}/k/{key}/", tags=[Tags.db])
async def key_resource_put(nsname: str, key: str, request: Request):
    """Create or update the value of a key"""
    # check if "reblaze/k/<key>" exists in system/schema-validation
    req_json = await request.json()

    if nsname != "system":
        keyName = nsname + "/k/" + key
        schemas = request.app.backend.key_get("system", "schema-validation")
        schema = None
        # find schema if exists and validate the json input
        for item in schemas.items():
            if item[0] == keyName:
                schema = item[1]
                break
        if schema:
            isValid = validateDbJson(req_json, schema)
            if isValid is False:
                raise HTTPException(500, "schema mismatched")
    return request.app.backend.key_set(nsname, key, req_json, get_gitactor(request))


@router.delete("/db/{nsname}/k/{key}/", tags=[Tags.db])
async def key_resource_delete(nsname: str, key: str, request: Request):
    """Delete a key"""
    if nsname == preserved_namespace and key in bootsrap_system_keys:
        raise HTTPException(403, f"'{key}' should not be deleted from '{nsname}' namespace")
    return request.app.backend.key_delete(nsname, key, get_gitactor(request))


#############
### Tools ###
#############


@router.get("/tools/fetch", tags=[Tags.tools])
async def fetch_resource_get(url: HttpUrl):
    """Fetch an URL"""
    try:
        if not url.startswith("https://"):
            raise HTTPException(400, "forbidden url")
        r = requests.get(url)
        r_string = r.content.decode()
        if not bleach.clean(r_string) == r_string:
            raise HTTPException(400, "forbidden url")

    except Exception as e:
        raise HTTPException(400, "cannot retrieve [%s]: %s" % (url, e))
    return r.content


@router.put("/tools/publish/{config}/", tags=[Tags.tools])
@router.put("/tools/publish/{config}/v/{version}/", tags=[Tags.tools])
async def publish_resource_put(
    config: str, request: Request, buckets: List[Bucket], version: str = None
):
    """Push configuration to s3 buckets"""
    conf = request.app.backend.configs_get(config, version)
    ok = True
    status = []
    buckets = await request.json()
    if type(buckets) is not list:
        raise HTTPException(400, "body must be a list")

    for bucket in buckets:
        logs = []
        try:
            cloud.export(conf, bucket["url"], prnt=lambda x: logs.append(x))
        except Exception as e:
            ok = False
            s = False
            msg = repr(e)
        else:
            s = True
            msg = "ok"
        status.append({"name": bucket["name"], "ok": s, "logs": logs, "message": msg})
    return {"ok": ok, "status": status}


@router.put("/tools/gitpush/", tags=[Tags.tools])
async def git_push_resource_put(git_urls: List[GitUrl], request: Request):
    """Push git configuration to remote git repositories"""
    ok = True
    status = []
    git_jsons = await request.json()
    for giturl in git_jsons:
        try:
            request.app.backend.gitpush(giturl["giturl"])
        except Exception as e:
            msg = repr(e)
            s = False
        else:
            msg = "ok"
            s = True
        status.append({"url": giturl["giturl"], "ok": s, "message": msg})
    return {"ok": ok, "status": status}


@router.put("/tools/gitfetch/", tags=[Tags.tools])
async def git_fetch_resource_put(giturl: GitUrl, request: Request):
    """Fetch git configuration from specified remote repository"""
    ok = True
    giturl_json = await request.json()
    try:
        request.app.backend.gitfetch(giturl_json["giturl"])
    except Exception as e:
        ok = False
        msg = repr(e)
    else:
        msg = "ok"
    return {"ok": ok, "status": msg}


@router.put("/tools/backup/create", tags=[Tags.tools])
@router.put("/tools/backup/create/{backup_file_name}", tags=[Tags.tools])
async def backup_create(
    request: Request, buckets: List[Bucket], backup_file_name: str = "backup"
):
    """Create backup for database"""

    backup_file_name = unquote(backup_file_name)
    backup_file_name = "/cf-persistent-config/" + backup_file_name

    ok = True
    status = []
    current_backup_filename = None
    try:
        current_backup_filename = request.app.backend.create_zip_archive_for_folder(
            backup_file_name
        )
        status.append("Backup created")

        buckets = await request.json()
        if type(buckets) is not list:
            raise HTTPException(400, "body must be a list")

        for bucket in buckets:
            logs = []
            try:
                cloud.upload_file(
                    current_backup_filename,
                    bucket["url"],
                    prnt=lambda x: logs.append(x),
                )

            except Exception as e:
                ok = False
                s = False
                msg = repr(e)
                logger.error(f"Exception when upload backup to cloud. {e}")
            else:
                s = True
                msg = "ok"
            status.append(
                {"name": bucket["name"], "ok": s, "logs": logs, "message": msg}
            )

            os.remove(current_backup_filename)
            status.append("Backup removed")

    except (PermissionError, FileNotFoundError, OSError) as e:
        logger.error(f"Can't remove local backup. {e}")
        raise HTTPException(500, f"Can't remove local backup. {e}")

    except Exception as e:
        if current_backup_filename is not None:
            os.remove(current_backup_filename)
        raise HTTPException(500, f"Something went wrong. ${e}")

    return {"ok": ok, "status": status}
