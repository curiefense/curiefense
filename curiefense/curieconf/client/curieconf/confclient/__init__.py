from simple_rest_client.api import API
from simple_rest_client.resource import Resource


def GET(url):
    return dict(method="GET", url=url)


def POST(url):
    return dict(method="POST", url=url)


def PUT(url):
    return dict(method="PUT", url=url)


def DELETE(url):
    return dict(method="DELETE", url=url)


class ConfigsResource(Resource):
    actions = dict(
        list=GET("configs/"),
        get=GET("configs/{}/"),
        list_versions=GET("configs/{}/v/"),
        get_version=GET("configs/{}/v/{}/"),
        create=POST("configs/"),
        create_name=POST("configs/{}/"),
        delete=DELETE("configs/{}/"),
        update=PUT("configs/{}/"),
        revert=PUT("configs/{}/v/{}/revert/"),
        clone=POST("configs/{}/clone/"),
        clone_name=POST("configs/{}/clone/{}/"),
    )


class BlobsResource(Resource):
    actions = dict(
        list=GET("configs/{}/b/"),
        get=GET("configs/{}/b/{}/"),
        list_versions=GET("configs/{}/b/{}/v/"),
        get_version=GET("configs/{}/b/{}/v/{}/"),
        create=POST("configs/{}/b/{}/"),
        update=PUT("configs/{}/b/{}/"),
        delete=DELETE("configs/{}/b/{}/"),
        revert=PUT("configs/{}/b/{}/v/{}/revert/"),
    )


class DocumentsResource(Resource):
    actions = dict(
        list=GET("configs/{}/d/"),
        list_versions=GET("configs/{}/d/{}/v/"),
        get_version=GET("configs/{}/d/{}/v/{}/"),
        get=GET("configs/{}/d/{}/"),
        create=POST("configs/{}/d/{}/"),
        update=PUT("configs/{}/d/{}/"),
        delete=DELETE("configs/{}/d/{}/"),
        revert=PUT("configs/{}/d/{}/v/{}/revert/"),
    )


class EntriesResource(Resource):
    actions = dict(
        list=GET("configs/{}/d/{}/e/"),
        get=GET("configs/{}/d/{}/e/{}/"),
        list_versions=GET("configs/{}/d/{}/e/{}/v/"),
        get_version=GET("configs/{}/d/{}/e/{}/v/{}/"),
        create=POST("configs/{}/d/{}/e/"),
        update=PUT("configs/{}/d/{}/e/{}/"),
        edit=PUT("configs/{}/d/{}/e/{}/edit/"),
        delete=DELETE("configs/{}/d/{}/e/{}/"),
        revert=PUT("configs/{}/d/{}/e/{}/v/{}/revert/"),
    )


class DBResource(Resource):
    actions = dict(
        list=GET("db/"),
        list_versions=GET("db/v/"),
        get=GET("db/{}/"),
        get_version=GET("db/{}/v/{}/"),
        create=POST("db/{}/"),
        update=PUT("db/{}/"),
        delete=DELETE("db/{}/"),
        revert=PUT("db/{}/v/{}/"),
        query=POST("db/{}/q/"),
    )


class KeyResource(Resource):
    actions = dict(
        list=GET("db/{}/k/"),
        get=GET("db/{}/k/{}/"),
        list_versions=GET("db/{}/k/{}/v/"),
        get_version=GET("db/{}/k/{}/v/{}/"),
        set=PUT("db/{}/k/{}/"),
        delete=DELETE("db/{}/k/{}/"),
    )


class ToolsResource(Resource):
    actions = dict(
        publish=PUT("tools/publish/{}/"),
        publish_version=PUT("tools/publish/{}/v/{}/"),
    )


def get_api(
    api_root_url="http://localhost:5000/api/v3/", json_encode_body=True, **kargs
):
    api = API(
        api_root_url=api_root_url,
        json_encode_body=json_encode_body,
        timeout=60,
        **kargs
    )

    api.add_resource(resource_name="configs", resource_class=ConfigsResource)
    api.add_resource(resource_name="blobs", resource_class=BlobsResource)
    api.add_resource(resource_name="documents", resource_class=DocumentsResource)
    api.add_resource(resource_name="entries", resource_class=EntriesResource)
    api.add_resource(resource_name="db", resource_class=DBResource)
    api.add_resource(resource_name="key", resource_class=KeyResource)
    api.add_resource(resource_name="tools", resource_class=ToolsResource)

    return api
