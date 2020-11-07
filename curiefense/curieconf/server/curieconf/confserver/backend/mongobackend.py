import os
from flask_pymongo import PyMongo
from . import Backends, CurieBackend, CurieBackendException
import urllib
from collections import defaultdict

class CurieMongoBackendException(CurieBackendException):
    pass

CONFIG_TABLES = ["limits", "urlmaps", "customsigs", "wafsigs", "wafprofiles", "aclprofiles"]

@Backends.register("mongodb")
class MongoBackend(CurieBackend):
    def __init__(self, app, url):
        CurieBackend.__init__(self, app, url)
        app.config["MONGO_URI"] = url
        self.mongo = PyMongo(app)
        self.parsed_url = urllib.parse.urlparse(url)
        if self.parsed_url.netloc == "" or self.parsed_url.scheme != "mongodb":
            raise CurieMongoBackendException("Bad URL format [%s]. (eg: mongodb://ip:port/db)" % (url,))
        self.db = self.mongo.db

    def configs_list(self):
        return list(self.db.versions.find())
    def configs_create(self, data):
        if self.db.configs.count_documents({"id": data["id"]},limit=1) > 0:
            return {"ok":0, "error": "id already exists"},409
        r = self.db.configs.insert_one(data, check_keys=False)
        return {"ok":1},201

    def configs_get(self, config):
        return self.db.versions.find_one({"id": config})
    def configs_update(self, config, data):
        r = self.db.configs.update({"id": config}, data)
        return r, 201
    def configs_delete(self, config):
        res = { coll: self.db[coll].delete_many({"_config": config}).deleted_count
                for coll in CONFIG_TABLES }
        res["config"] = self.db.configs.delete_one({"id":config}).deleted_count

    def batch_get(self, config, version="latest"):
        if version != "latest":
            raise NotImplementedError
        if self.db.configs.count_documents({"id": config}, limit=1) == 0:
            return {"ok": 0}, 404
        r = { coll: list(self.db[coll].find({"_config": config}))
              for coll in CONFIG_TABLES }
        return r

    def batch_create(self, config, data):
        if self.db.configs.count_documents({"id": config}, limit=1) > 0:
            return {"ok":0, "error": "config already exists"},409
        res = defaultdict(int)
        for coll in CONFIG_TABLES:
            for doc in data.get(coll, []):
                doc["_config"] = config
                self.db[coll].insert(doc, check_keys=False)
                res[coll] += 1
        v = data.get("config",{})
        vdesc = v.get("description", "")
        vdate = v.get("date", datetime.datetime.now().isoformat())
        self.db.configs.insert_one({"id": config, "description": vdesc, "date": vdate})
        res["ok"] = True
        return res

    def batch_update(self, config, data):
        modified = defaultdict(int)
        created = defaultdict(int)
        res = {"created": created, "modified": modified }
        for coll in CONFIG_TABLES:
            for doc in data.get(coll, []):
                doc["_config"] = config
                r = self.db[coll].update({"_config": config, "id": doc["id"]}, 
                                    doc, upsert=True, check_keys=False)
                print(r)
                modified[coll] += r["nModified"]
                created[coll] += r["n"]-r["nModified"]
        if self.db.configs.count_documents({"id": config}, limit=1) == 0:
            v = data.get("config",{})
            vdesc = v.get("description", "")
            vdate = v.get("date", datetime.datetime.now().isoformat())
            self.db.configs.insert_one({"id": config, "description": vdesc, "date": vdate})
            res["configs"] = 1
        res["ok"] = 1
        return res
