import urllib


class CurieBackendException(Exception):
    pass


class Backends(object):
    backends = {}

    @classmethod
    def register(cls, scheme):
        def do_reg(c):
            cls.backends[scheme] = c
            return c

        return do_reg

    @classmethod
    def get_backend(cls, app, url, *args, **kargs):
        pl = urllib.parse.urlparse(url)
        b = cls.backends[pl.scheme]
        return b(app, url, *args, **kargs)


class CurieBackend(object):
    def __init__(self, app, url):
        pass

    def configs_list(self):
        raise NotImplementedError

    def configs_list_versions(self, config):
        raise NotImplementedError

    def configs_get(self, config, version=None):
        raise NotImplementedError

    def configs_create(self, data, name=None):
        raise NotImplementedError

    def configs_update(self, config, data):
        raise NotImplementedError

    def configs_delete(self, config):
        raise NotImplementedError

    def configs_revert(self, config, version):
        raise NotImplementedError

    def configs_clone(self, config, new_name):
        raise NotImplementedError

    def blobs_list(self, config, version=None):
        raise NotImplementedError

    def blobs_list_versions(self, config, blob):
        raise NotImplementedError

    def blob_get(self, config, blob, version=None):
        raise NotImplementedError

    def blobs_create(self, config, blob, data):
        raise NotImplementedError

    def blobs_update(self, config, blob, data):
        raise NotImplementedError

    def blobs_delete(self, config, blob):
        raise NotImplementedError

    def blobs_revert(self, config, blob, version):
        raise NotImplementedError

    def documents_list(self, config, version=None):
        raise NotImplementedError

    def documents_get(self, config, document, version=None):
        raise NotImplementedError

    def documents_list_versions(self, config, document):
        raise NotImplementedError

    def documents_create(self, config, document, data):
        raise NotImplementedError

    def documents_update(self, config, document, data):
        raise NotImplementedError

    def documents_delete(self, config, document):
        raise NotImplementedError

    def documents_revert(self, config, document, version):
        raise NotImplementedError

    def entries_list(self, config, document, version=None):
        raise NotImplementedError

    def entries_list_versions(self, config, document, entry):
        raise NotImplementedError

    def entries_get(self, config, document, entry, version=None):
        raise NotImplementedError

    def entries_create(self, config, document, data):
        raise NotImplementedError

    def entries_update(self, config, document, entry, data):
        raise NotImplementedError

    def entries_edit(self, config, document, entry, data):
        raise NotImplementedError

    def entries_delete(self, config, document, entry):
        raise NotImplementedError

    def entries_revert(self, config, document, entry, version):
        raise NotImplementedError

    def ns_list(self):
        raise NotImplementedError

    def ns_list_versions(self):
        raise NotImplementedError

    def ns_get(self, dbname, version=None):
        raise NotImplementedError

    def ns_create(self, dbname, data):
        raise NotImplementedError

    def ns_update(slef, dbname, data):
        raise NotImplementedError

    def ns_delete(self, dbname):
        raise NotImplementedError

    def ns_revert(self, dbname, version):
        raise NotImplementedError

    def ns_query(self, dbname, query):
        raise NotImplementedError

    def key_list(self, dbname):
        raise NotImplementedError

    def key_get(self, dbname, key, version=None):
        raise NotImplementedError

    def key_list_versions(self, dbname, key):
        raise NotImplementedError

    def key_set(self, dbname, key, value):
        raise NotImplementedError

    def key_delete(self, dbname, key):
        raise NotImplementedError


from . import gitbackend
