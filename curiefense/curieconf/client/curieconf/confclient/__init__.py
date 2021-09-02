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
        list_versions=GET("configs/{}/v"),
        get_version=GET("configs/{}/v/{}/"),
        create=POST("configs/"),
        create_name=POST("configs/{}/"),
        delete=DELETE("configs/{}/"),
        update=PUT("configs/{}/"),
        revert=PUT("configs/{}/v/{}/revert/"),
        clone=POST("configs/{}/clone"),
        clone_name=POST("configs/{}/clone/{}/"),
    )


class BlobsResource(Resource):
    actions = dict(
        list=GET("configs/{}/b"),
        get=GET("configs/{}/b/{}/"),
        list_versions=GET("configs/{}/b/{}/v/"),
        get_version=GET("configs/{}/b/{}/v/{}/"),
        create=POST("configs/{}/b/{}/"),
        update=PUT("configs/{}/b/{}/"),
        delete=DELETE("configs/{}/b/{}/"),
        revert=PUT("configs/{}/b/{}/v/{}/revert/"),
    )


class ConfigTypesResource(Resource):
    actions = dict(
        list=GET("configs/{}/t/"),
        list_versions=GET("configs/{}/t/{}/v/"),
        get_version=GET("configs/{}/t/{}/v/{}/"),
        get=GET("configs/{}/t/{}/"),
        create=POST("configs/{}/t/{}/"),
        update=PUT("configs/{}/t/{}/"),
        delete=DELETE("configs/{}/t/{}/"),
        revert=PUT("configs/{}/t/{}/v/{}/revert/"),
    )


class EntriesResource(Resource):
    actions = dict(
        list=GET("configs/{}/t/{}/e/"),
        get=GET("configs/{}/t/{}/e/{}/"),
        list_versions=GET("configs/{}/t/{}/e/{}/v/"),
        get_version=GET("configs/{}/t/{}/e/{}/v/{}/"),
        create=POST("configs/{}/t/{}/e/"),
        update=PUT("configs/{}/t/{}/e/{}/"),
        edit=PUT("configs/{}/t/{}/e/{}/edit/"),
        delete=DELETE("configs/{}/t/{}/e/{}/"),
        revert=PUT("configs/{}/t/{}/e/{}/v/{}/revert/"),
    )


class DBResource(Resource):
    actions = dict(
        list=GET("db/"),
        list_versions=GET("db/v/"),
        get=GET("db/{}/"),
        get_version=GET("db/{}/v/{}"),
        create=POST("db/{}"),
        update=PUT("db/{}"),
        delete=DELETE("db/{}"),
        revert=PUT("db/{}/v/{}"),
        query=POST("db/{}/q"),
    )


class KeyResource(Resource):
    actions = dict(
        list=GET("db/{}/k/"),
        get=GET("db/{}/k/{}"),
        list_versions=GET("db/{}/k/{}/v/"),
        get_version=GET("db/{}/k/{}/v/{}"),
        set=PUT("db/{}/k/{}"),
        delete=DELETE("db/{}/k/{}"),
    )


class ToolsResource(Resource):
    actions = dict(
        publish=PUT("tools/publish/{}/"),
        publish_version=PUT("tools/publish/{}/v/{}/"),
    )


def get_api(
    api_root_url="http://localhost:5000/api/v2/", json_encode_body=True, **kargs
):
    api = API(
        api_root_url=api_root_url,
        json_encode_body=json_encode_body,
        timeout=60,
        **kargs
    )

    api.add_resource(resource_name="configs", resource_class=ConfigsResource)
    api.add_resource(resource_name="blobs", resource_class=BlobsResource)
    api.add_resource(resource_name="configtypes", resource_class=ConfigTypesResource)
    api.add_resource(resource_name="entries", resource_class=EntriesResource)
    api.add_resource(resource_name="db", resource_class=DBResource)
    api.add_resource(resource_name="key", resource_class=KeyResource)
    api.add_resource(resource_name="tools", resource_class=ToolsResource)

    return api
