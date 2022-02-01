from flask_restx import fields, Namespace
from curieconf import utils

api = Namespace("models")


##############
### MODELS ###
##############


### Models for documents


class AnyType(fields.Raw):
    __schema_type__ = "any"


# limit

m_threshold = api.model(
    "Rate Limit Threshold",
    {
        "limit": fields.String(required=True),
        "action": fields.Raw(required=True),
    },
)

m_limit = api.model(
    "Rate Limit",
    {
        "id": fields.String(required=True),
        "name": fields.String(required=True),
        "description": fields.String(required=True),
        "timeframe": fields.String(required=True),
        "thresholds": fields.List(fields.Nested(m_threshold)),
        "include": fields.Raw(required=True),
        "exclude": fields.Raw(required=True),
        "key": AnyType(required=True),
        "pairwith": fields.Raw(required=True),
    },
)

# securitypolicy

m_secprofilemap = api.model(
    "Security Profile Map",
    {
        "name": fields.String(required=True),
        "match": fields.String(required=True),
        "acl_profile": fields.String(required=True),
        "acl_active": fields.Boolean(required=True),
        "content_filter_profile": fields.String(required=True),
        "content_filter_active": fields.Boolean(required=True),
        "limit_ids": fields.List(fields.Raw()),
    },
)

m_map = api.model(
    "Security Profile Map", {"*": fields.Wildcard(fields.Nested(m_secprofilemap))}
)

m_securitypolicy = api.model(
    "Security Policy",
    {
        "id": fields.String(required=True),
        "name": fields.String(required=True),
        "match": fields.String(required=True),
        "map": fields.List(fields.Nested(m_secprofilemap)),
    },
)

# content filter rule

m_contentfilterrule = api.model(
    "Content Filter Rule",
    {
        "id": fields.String(required=True),
        "name": fields.String(required=True),
        "msg": fields.String(required=True),
        "operand": fields.String(required=True),
        "severity": fields.Integer(required=True),
        "certainity": fields.Integer(required=True),
        "category": fields.String(required=True),
        "subcategory": fields.String(required=True),
    },
)

# contentfiltergroup

m_contentfiltergroup = api.model(
    "Content Filter Group",
    {
        "id": fields.String(required=True),
        "name": fields.String(required=True),
        "description": fields.String(required=True),
        "content_filter_rule_ids": fields.List(fields.String(), skip_none=True),
    },
)

# content filter profile

m_contentfilterprofile = api.model(
    "Content Filter Profile",
    {
        "id": fields.String(required=True),
        "name": fields.String(required=True),
        "ignore_alphanum": fields.Boolean(required=True),
        "max_header_length": fields.Integer(required=True),
        "max_cookie_length": fields.Integer(required=True),
        "max_arg_length": fields.Integer(required=True),
        "max_headers_count": fields.Integer(required=True),
        "max_cookies_count": fields.Integer(required=True),
        "max_args_count": fields.Integer(required=True),
        "args": fields.Raw(),
        "headers": fields.Raw(),
        "cookies": fields.Raw(),
    },
)

# aclprofile

tags_array = fields.List(fields.String(), unique=True, required=True)

m_aclprofile = api.model(
    "ACL Profile",
    {
        "id": fields.String(
            required=True, min_length=1, title="Id", description="Unique id"
        ),
        "name": fields.String(
            required=True,
            min_length=1,
            title="Name",
            description="Name of entity shown in UI",
        ),
        "allow": tags_array,
        "allow_bot": tags_array,
        "deny_bot": tags_array,
        "passthrough": tags_array,
        "deny": tags_array,
        "force_deny": tags_array,
    },
    strict=True,
)

# Global Filter

m_globalfilter = api.model(
    "Global Filter",
    {
        "id": fields.String(required=True),
        "name": fields.String(required=True),
        "source": fields.String(required=True),
        "mdate": fields.String(required=True),
        "description": fields.String(required=True),
        "active": fields.Boolean(required=True),
        "action": fields.Raw(required=True),
        "tags": fields.List(fields.String()),
        "rule": AnyType(),
    },
)

# Flow Control

m_flowcontrol = api.model(
    "Flow Control",
    {
        "id": fields.String(required=True),
        "name": fields.String(required=True),
        "timeframe": fields.Integer(required=True),
        "key": fields.List(fields.Raw(required=True)),
        "sequence": fields.List(fields.Raw(required=True)),
        "action": fields.Raw(required=True),
        "include": fields.List(fields.String()),
        "exclude": fields.List(fields.String()),
        "description": fields.String(required=True),
        "active": fields.Boolean(required=True),
    },
)

### mapping from doc name to model

models = {
    "ratelimits": m_limit,
    "securitypolicies": m_securitypolicy,
    "contentfilterrules": m_contentfilterrule,
    "contentfiltergroups": m_contentfiltergroup,
    "contentfilterprofiles": m_contentfilterprofile,
    "aclprofiles": m_aclprofile,
    "globalfilters": m_globalfilter,
    "flowcontrol": m_flowcontrol,
}

### Other models

m_document_mask = api.model(
    "Mask for document",
    {
        "id": fields.String(required=True),
        "name": fields.String(required=True),
        "description": fields.String(required=True),
        "map": fields.List(fields.Nested(m_secprofilemap)),
        "include": fields.Wildcard(fields.Raw()),
        "exclude": fields.Wildcard(fields.Raw()),
        "tags": fields.List(fields.String()),
        "allow": fields.List(fields.String()),
        "allow_bot": fields.List(fields.String()),
        "deny_bot": fields.List(fields.String()),
        "passthrough": fields.List(fields.String()),
        "deny": fields.List(fields.String()),
        "force_deny": fields.List(fields.String()),
        "match": fields.String(),
        "*": fields.Wildcard(fields.Raw()),
    },
)

m_version_log = api.model(
    "Version log",
    {
        "version": fields.String(),
        "date": fields.DateTime(dt_format=u"iso8601"),
        "*": fields.Wildcard(fields.Raw()),
    },
)

m_meta = api.model(
    "Meta",
    {
        "id": fields.String(required=True),
        "description": fields.String(required=True),
        "date": fields.DateTime(),
        "logs": fields.List(fields.Nested(m_version_log), default=[]),
        "version": fields.String(),
    },
)

m_blob_entry = api.model(
    "Blob Entry",
    {
        "format": fields.String(required=True),
        "blob": AnyType(),
    },
)

m_blob_list_entry = api.model(
    "Blob ListEntry",
    {
        "name": fields.String(),
    },
)

m_document_list_entry = api.model(
    "Document ListEntry",
    {
        "name": fields.String(),
        "entries": fields.Integer(),
    },
)

m_config_documents = api.model(
    "Config Documents",
    {x: fields.List(fields.Nested(models[x], default=[])) for x in models},
)

m_config_blobs = api.model(
    "Config Blobs",
    {x: fields.Nested(m_blob_entry, default={}) for x in utils.BLOBS_PATH},
)

m_config_delete_blobs = api.model(
    "Config Delete Blobs", {x: fields.Boolean() for x in utils.BLOBS_PATH}
)

m_config = api.model(
    "Config",
    {
        "meta": fields.Nested(m_meta, default={}),
        "documents": fields.Nested(m_config_documents, default={}),
        "blobs": fields.Nested(m_config_blobs, default={}),
        "delete_documents": fields.Nested(m_config_documents, default={}),
        "delete_blobs": fields.Nested(m_config_delete_blobs, default={}),
    },
)

m_edit = api.model(
    "Edit",
    {
        "path": fields.String(required=True),
        "value": fields.String(required=True),
    },
)

### Publish

m_bucket = api.model(
    "Bucket",
    {
        "name": fields.String(required=True),
        "url": fields.String(required=True),
    },
)

### Git push & pull

m_giturl = api.model(
    "GitUrl",
    {
        "giturl": fields.String(required=True),
    },
)

### Db

m_db = api.model("db", {})
