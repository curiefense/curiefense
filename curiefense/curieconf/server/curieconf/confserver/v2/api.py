import jsonschema

# monkey patch to force RestPlus to use Draft3 validator to benefit from "any" json type
jsonschema.Draft4Validator = jsonschema.Draft3Validator

from flask import Blueprint, request, current_app, abort, make_response
from flask_restplus import Resource, Api, fields, marshal, reqparse
from curieconf import utils
from curieconf.utils import cloud
import requests
from jsonschema import validate
from pathlib import Path
import json


api_bp = Blueprint("api_v2", __name__)
api = Api(api_bp, version="2.0", title="Curiefense configuration API server v2.0")


ns_configs = api.namespace("configs", description="Configurations")
ns_db = api.namespace("db", description="Database")
ns_tools = api.namespace("tools", description="Tools")


##############
### MODELS ###
##############


### Models for documents


class AnyType(fields.Raw):
    __schema_type__ = "any"


# limit


m_limit = api.model(
    "Rate Limit",
    {
        "id": fields.String(required=True),
        "name": fields.String(required=True),
        "description": fields.String(required=True),
        "timeframe": fields.String(required=True),
        "limit": fields.String(required=True),
        "action": fields.Raw(required=True),
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
        "content_filter_rules_ids": fields.List(
            fields.String(), skip_none=True
        ),
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

m_aclprofile = api.model(
    "ACL Profile",
    {
        "id": fields.String(required=True),
        "name": fields.String(required=True),
        "allow": fields.List(fields.String()),
        "allow_bot": fields.List(fields.String()),
        "deny_bot": fields.List(fields.String()),
        "passthrough": fields.List(fields.String()),
        "deny": fields.List(fields.String()),
        "force_deny": fields.List(fields.String()),
    },
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

### Document Schema validation


def validateJson(json_data, schema_type):
    try:
        validate(instance=json_data, schema=schema_type_map[schema_type])
    except jsonschema.exceptions.ValidationError as err:
        print(str(err))
        return False
    return True


base_path = Path(__file__).parent
# base_path = "/etc/curiefense/json/"
acl_profile_file_path = (base_path / "./json/acl-profile.schema").resolve()
with open(acl_profile_file_path) as json_file:
    acl_profile_schema = json.load(json_file)
ratelimits_file_path = (base_path / "../json/rate-limits.schema").resolve()
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
flowcontrol_file_path = (base_path / "../json/flow-control.schema").resolve()
with open(flowcontrol_file_path) as json_file:
    flowcontrol_schema = json.load(json_file)
content_filter_rule_file_path = (
    base_path / "./json/content-filter-rule.schema"
).resolve()
with open(content_filter_rule_file_path) as json_file:
    content_filter_rule_schema = json.load(json_file)
content_filter_groups_file_path = (
    base_path / "./json/content-filter-groups.schema"
).resolve()
with open(content_filter_groups_file_path) as json_file:
    content_filter_groups_schema = json.load(json_file)


schema_type_map = {
    "ratelimits": ratelimits_schema,
    "securitypolicies": securitypolicies_schema,
    "contentfilterprofiles": content_filter_profile_schema,
    "aclprofiles": acl_profile_schema,
    "globalfilters": globalfilters_schema,
    "flowcontrol": flowcontrol_schema,
    "contentfilterrules": content_filter_rule_schema,
    "contentfiltergroups": content_filter_groups_schema,
}


################
### CONFIGS ###
################


@ns_configs.route("/")
class Configs(Resource):
    @ns_configs.marshal_list_with(m_meta, skip_none=True)
    def get(self):
        "Get the detailed list of existing configurations"
        return current_app.backend.configs_list()

    @ns_configs.expect(m_config, validate=True)
    def post(self):
        "Create a new configuration"
        data = request.json
        return current_app.backend.configs_create(data)


@ns_configs.route("/<string:config>/")
class Config(Resource):
    @ns_configs.marshal_with(m_config, skip_none=True)
    def get(self, config):
        "Retrieve a complete configuration"
        return current_app.backend.configs_get(config)

    @ns_configs.expect(m_config, validate=True)
    def post(self, config):
        "Create a new configuration. Configuration name in URL overrides configuration in POST data"
        data = request.json
        return current_app.backend.configs_create(data, name=config)

    @ns_configs.expect(m_meta, validate=True)
    def put(self, config):
        "Update an existing configuration"
        data = request.json
        return current_app.backend.configs_update(config, data)

    def delete(self, config):
        "Delete a configuration"
        return current_app.backend.configs_delete(config)


@ns_configs.route("/<string:config>/clone/")
class ConfigClone(Resource):
    @ns_configs.expect(m_meta, validate=True)
    def post(self, config):
        "Clone a configuration. New name is provided in POST data"
        data = request.json
        return current_app.backend.configs_clone(config, data)


@ns_configs.route("/<string:config>/clone/<string:new_name>/")
class ConfigCloneName(Resource):
    @ns_configs.expect(m_meta, validate=True)
    def post(self, config, new_name):
        "Clone a configuration. New name is provided URL"
        data = request.json
        return current_app.backend.configs_clone(config, data, new_name)


@ns_configs.route("/<string:config>/v/")
class ConfigListVersion(Resource):
    @ns_configs.marshal_with(m_version_log, skip_none=True)
    def get(self, config):
        "Get all versions of a given configuration"
        return current_app.backend.configs_list_versions(config)


@ns_configs.route("/<string:config>/v/<string:version>/")
class ConfigVersion(Resource):
    def get(self, config, version):
        "Retrieve a specific version of a configuration"
        return current_app.backend.configs_get(config, version)


@ns_configs.route("/<string:config>/v/<string:version>/revert/")
class ConfigRevert(Resource):
    def put(self, config, version):
        "Create a new version for a configuration from an old version"
        return current_app.backend.configs_revert(config, version)


#############
### Blobs ###
#############


@ns_configs.route("/<string:config>/b/")
class BlobsResource(Resource):
    @ns_configs.marshal_with(m_blob_list_entry, skip_none=True)
    def get(self, config):
        "Retrieve the list of available blobs"
        res = current_app.backend.blobs_list(config)
        return res


@ns_configs.route("/<string:config>/b/<string:blob>/")
class BlobResource(Resource):
    @ns_configs.marshal_with(m_blob_entry, skip_none=True)
    def get(self, config, blob):
        "Retrieve a blob"
        return current_app.backend.blobs_get(config, blob)

    @ns_configs.expect(m_blob_entry, validate=True)
    def post(self, config, blob):
        "Create a new blob"
        return current_app.backend.blobs_create(config, blob, request.json)

    @ns_configs.expect(m_blob_entry, validate=True)
    def put(self, config, blob):
        "Replace a blob with new data"
        return current_app.backend.blobs_update(config, blob, request.json)

    def delete(self, config, blob):
        "Delete a blob"
        return current_app.backend.blobs_delete(config, blob)


@ns_configs.route("/<string:config>/b/<string:blob>/v/")
class BlobListVersionResource(Resource):
    @ns_configs.marshal_list_with(m_version_log, skip_none=True)
    def get(self, config, blob):
        "Retrieve the list of versions of a given blob"
        res = current_app.backend.blobs_list_versions(config, blob)
        return res


@ns_configs.route("/<string:config>/b/<string:blob>/v/<string:version>/")
class BlobVersionResource(Resource):
    @ns_configs.marshal_list_with(m_version_log, skip_none=True)
    def get(self, config, blob, version):
        "Retrieve the given version of a blob"
        return current_app.backend.blobs_get(config, blob, version)


@ns_configs.route("/<string:config>/b/<string:blob>/v/<string:version>/revert/")
class BlobRevertResource(Resource):
    def put(self, config, blob, version):
        "Create a new version for a blob from an old version"
        return current_app.backend.blobs_revert(config, blob, version)


#################
### DOCUMENTS ###
#################


@ns_configs.route("/<string:config>/d/")
class DocumentsResource(Resource):
    @ns_configs.marshal_with(m_document_list_entry, skip_none=True)
    def get(self, config):
        "Retrieve the list of existing documents in this configuration"
        res = current_app.backend.documents_list(config)
        return res


@ns_configs.route("/<string:config>/d/<string:document>/")
class DocumentResource(Resource):
    @ns_configs.marshal_with(m_document_mask, mask="*", skip_none=True)
    def get(self, config, document):
        "Get a complete document"
        if document not in models:
            abort(404, "document does not exist")
        res = current_app.backend.documents_get(config, document)
        return marshal(res, models[document], skip_none=True)

    def post(self, config, document):
        "Create a new complete document"
        if document not in models:
            abort(404, "document does not exist")
        data = marshal(request.json, models[document], skip_none=True)
        res = current_app.backend.documents_create(config, document, data)
        return res

    def put(self, config, document):
        "Update an existing document"
        if document not in models:
            abort(404, "document does not exist")
        data = marshal(request.json, models[document], skip_none=True)
        res = current_app.backend.documents_update(config, document, data)
        return res

    def delete(self, config, document):
        "Delete/empty a document"
        if document not in models:
            abort(404, "document does not exist")
        res = current_app.backend.documents_delete(config, document)
        return res


@ns_configs.route("/<string:config>/d/<string:document>/v/")
class DocumentListVersionResource(Resource):
    def get(self, config, document):
        "Retrieve the existing versions of a given document"
        if document not in models:
            abort(404, "document does not exist")
        res = current_app.backend.documents_list_versions(config, document)
        return marshal(res, m_version_log, skip_none=True)


@ns_configs.route("/<string:config>/d/<string:document>/v/<string:version>/")
class DocumentVersionResource(Resource):
    def get(self, config, document, version):
        "Get a given version of a document"
        if document not in models:
            abort(404, "document does not exist")
        res = current_app.backend.documents_get(config, document, version)
        return marshal(res, models[document], skip_none=True)


@ns_configs.route("/<string:config>/d/<string:document>/v/<string:version>/revert/")
class DocumentRevertResource(Resource):
    def put(self, config, document, version):
        "Create a new version for a document from an old version"
        return current_app.backend.documents_revert(config, document, version)


###############
### ENTRIES ###
###############


@ns_configs.route("/<string:config>/d/<string:document>/e/")
class EntriesResource(Resource):
    def get(self, config, document):
        "Retrieve the list of entries in a document"
        if document not in models:
            abort(404, "document does not exist")
        res = current_app.backend.entries_list(config, document)
        return res  # XXX: marshal

    def post(self, config, document):
        "Create an entry in a document"
        if document not in models:
            abort(404, "document does not exist")
        data = marshal(request.json, models[document], skip_none=True)
        res = current_app.backend.entries_create(config, document, data)
        return res


@ns_configs.route("/<string:config>/d/<string:document>/e/<string:entry>/")
class EntryResource(Resource):
    def get(self, config, document, entry):
        "Retrieve an entry from a document"
        if document not in models:
            abort(404, "document does not exist")
        res = current_app.backend.entries_get(config, document, entry)
        return marshal(res, models[document], skip_none=True)

    def put(self, config, document, entry):
        "Update an entry in a document"
        if document not in models:
            abort(404, "document does not exist")
        isValid = validateJson(request.json, document)
        if isValid:
            data = marshal(request.json, models[document], skip_none=True)
            res = current_app.backend.entries_update(config, document, entry, data)
            return res
        else:
            abort(500, "schema mismatched")

    def delete(self, config, document, entry):
        "Delete an entry from a document"
        if document not in models:
            abort(404, "document does not exist")
        res = current_app.backend.entries_delete(config, document, entry)
        return res


@ns_configs.route("/<string:config>/d/<string:document>/e/<string:entry>/edit/")
class EntryEditResource(Resource):
    def put(self, config, document, entry):
        "Update an entry in a document"
        if document not in models:
            abort(404, "document does not exist")
        data = marshal(request.json, m_edit, skip_none=True)
        if type(data) is not list:
            data = [data]
        res = current_app.backend.entries_edit(config, document, entry, data)
        return res


@ns_configs.route("/<string:config>/d/<string:document>/e/<string:entry>/v/")
class EntryListVersionResource(Resource):
    def get(self, config, document, entry):
        "Get the list of existing versions of a given entry in a document"
        if document not in models:
            abort(404, "document does not exist")
        res = current_app.backend.entries_list_versions(config, document, entry)
        return marshal(res, m_version_log, skip_none=True)


@ns_configs.route(
    "/<string:config>/d/<string:document>/e/<string:entry>/v/<string:version>/"
)
class EntryVersionResource(Resource):
    def get(self, config, document, entry, version):
        "Get a given version of a document entry"
        if document not in models:
            abort(404, "document does not exist")
        res = current_app.backend.entries_get(config, document, entry, version)
        return marshal(res, models[document], skip_none=True)


################
### Database ###
################


@ns_db.route("/")
class DbResource(Resource):
    def get(self):
        "Get the list of existing namespaces"
        return current_app.backend.ns_list()


@ns_db.route("/v/")
class DbQueryResource(Resource):
    def get(self):
        "List all existing versions of namespaces"
        return current_app.backend.ns_list_versions()


@ns_db.route("/<string:nsname>/")
class NSResource(Resource):
    def get(self, nsname):
        "Get a complete namespace"
        try:
            return current_app.backend.ns_get(nsname, version=None)
        except KeyError:
            abort(404, "namespace [%s] does not exist" % nsname)

    @ns_db.expect(m_db, validate=True)
    def post(self, nsname):
        "Create a non-existing namespace from data"
        try:
            return current_app.backend.ns_create(nsname, request.json)
        except Exception:
            abort(409, "namespace [%s] already exists" % nsname)

    @ns_db.expect(m_db, validate=True)
    def put(self, nsname):
        "Merge data into a namespace"
        return current_app.backend.ns_update(nsname, request.json)

    def delete(self, nsname):
        "Delete an existing namespace"
        try:
            return current_app.backend.ns_delete(nsname)
        except KeyError:
            abort(409, "namespace [%s] does not exist" % nsname)


@ns_db.route("/<string:nsname>/v/<string:version>/")
class NSVersionResource(Resource):
    def get(self, nsname, version):
        "Get a given version of a namespace"
        return current_app.backend.ns_get(nsname, version)


@ns_db.route("/<string:nsname>/v/<string:version>/revert/")
class NSVersionResource(Resource):
    def put(self, nsname, version):
        "Create a new version for a namespace from an old version"
        try:
            return current_app.backend.ns_revert(nsname, version)
        except KeyError:
            abort(404, "namespace [%s] version [%s] not found" % (nsname, version))


@ns_db.route("/<string:nsname>/q/")
class NSQueryResource(Resource):
    def post(self, nsname):
        "Run a JSON query on the namespace and returns the results"
        return current_app.backend.ns_query(nsname, request.json)


@ns_db.route("/<string:nsname>/k/")
class KeysResource(Resource):
    def get(self, nsname):
        "List all keys of a given namespace"
        return current_app.backend.key_list(nsname)


@ns_db.route("/<string:nsname>/k/<string:key>/v/")
class KeysListVersionsResource(Resource):
    def get(self, nsname, key):
        "Get all versions of a given key in namespace"
        return current_app.backend.key_list_versions(nsname, key)


@ns_db.route("/<string:nsname>/k/<string:key>/")
class KeyResource(Resource):
    def get(self, nsname, key):
        "Retrieve a given key's value from a given namespace"
        return current_app.backend.key_get(nsname, key)

    def put(self, nsname, key):
        "Create or update the value of a key"
        return current_app.backend.key_set(nsname, key, request.json)

    def delete(self, nsname, key):
        "Delete a key"
        return current_app.backend.key_delete(nsname, key)


#############
### Tools ###
#############


req_fetch_parser = reqparse.RequestParser()
req_fetch_parser.add_argument("url", location="args", help="url to retrieve")


@ns_tools.route("/fetch")
class FetchResource(Resource):
    @ns_tools.expect(req_fetch_parser, validate=True)
    def get(self):
        "Fetch an URL"
        args = req_fetch_parser.parse_args()
        try:
            r = requests.get(args.url)
        except Exception as e:
            abort(400, "cannot retrieve [%s]: %s" % (args.url, e))
        return make_response(r.content)


@ns_tools.route("/publish/<string:config>/")
@ns_tools.route("/publish/<string:config>/v/<string:version>/")
class PublishResource(Resource):
    @ns_tools.expect([m_bucket], validate=True)
    def put(self, config, version=None):
        "Push configuration to s3 buckets"
        conf = current_app.backend.configs_get(config, version)
        ok = True
        status = []
        if type(request.json) is not list:
            abort(400, "body must be a list")
        for bucket in request.json:
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
            status.append(
                {"name": bucket["name"], "ok": s, "logs": logs, "message": msg}
            )
        return make_response({"ok": ok, "status": status})


@ns_tools.route("/gitpush/")
class GitPushResource(Resource):
    @ns_tools.expect([m_giturl], validate=True)
    def put(self):
        "Push git configuration to remote git repositories"
        ok = True
        status = []
        for giturl in request.json:
            try:
                current_app.backend.gitpush(giturl["giturl"])
            except Exception as e:
                msg = repr(e)
                s = False
            else:
                msg = "ok"
                s = True
            status.append({"url": giturl["giturl"], "ok": s, "message": msg})
        return make_response({"ok": ok, "status": status})


@ns_tools.route("/gitfetch/")
class GitFetchResource(Resource):
    @ns_tools.expect(m_giturl, validate=True)
    def put(self):
        "Fetch git configuration from specified remote repository"
        ok = True
        try:
            current_app.backend.gitfetch(request.json["giturl"])
        except Exception as e:
            ok = False
            msg = repr(e)
        else:
            msg = "ok"
        return make_response({"ok": ok, "status": msg})
