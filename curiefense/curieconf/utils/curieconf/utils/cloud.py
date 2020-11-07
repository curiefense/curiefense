import os
import hashlib
import types
from curieconf import utils
import json
import io

def hash(data):
    return hashlib.sha256(data).hexdigest()

def get_bucket(url):
    from cloudstorage import DriverName, get_driver
    from urllib.parse import urlparse
    from cloudstorage.exceptions import NotFoundError
    u = urlparse(url)
    if u.scheme == "file":
        storage = get_driver(DriverName.LOCAL)(os.path.dirname(u.path))
        bucket = storage.get_container("")
        pth = os.path.basename(u.path)
    else:
        pth = u.path
        if pth.startswith("/"):
            pth = pth[1:]
        if u.scheme == "gs":
            storage = get_driver(DriverName.GOOGLESTORAGE)(os.path.dirname(u.path))
        elif u.scheme == "s3":
            akey = os.environ.get("CURIE_S3_ACCESS_KEY")
            skey = os.environ.get("CURIE_S3_SECRET_KEY")
            if not (akey and skey):
                s3pth = os.environ.get("CURIE_S3CFG_PATH")
                if not s3pth:
                    for s3pth in [os.path.expanduser("~/.s3cfg"), "/var/run/secrets/s3cfg/s3cfg", "/var/run/secrets/s3cfg"]:
                        if os.path.isfile(s3pth):
                            break
                    else:
                        raise Exception("Did not find any credential to access %s" % url)
                from configparser import ConfigParser
                c = ConfigParser()
                c.read(s3pth)
                akey = c.get("default", "access_key")
                skey = c.get("default", "secret_key")
            storage = get_driver(DriverName.S3)(
                key = akey, secret=skey) #, url=os.path.dirname(u.path))
        elif u.scheme == "azblob":
            storage = get_driver(DriverName.AZURE)(os.path.dirname(u.path))
        else:
            raise Exception("Unknown or unsupported bucket URL scheme: [%s]" % u.scheme)
        bucket = storage.get_container(u.netloc)

    ## Monkey patching missing method.
    ## This permits to keep cloudstorage imports to this namespace
    def exists(self, key):
        try:
            self.get_blob(key)
        except NotFoundError:
            return False
        return True
    bucket.exists = types.MethodType(exists, bucket)
    return bucket, pth


def upload_manifest(manifest, bucket, root, configname, prnt=None):
    manifest = io.BytesIO(json.dumps(manifest, indent=4).encode("utf-8"))
    manifest.seek(0)
    mkey = os.path.join(root, "manifest.%s.json" % configname if configname else "manifest.json")
    if prnt:
        prnt("Uploading new manifest [%s]" % mkey)
    bucket.upload_blob(manifest, mkey, meta_data={})

def export(conf, url, prnt=None):
    bucket, root = get_bucket(url)
    manifest_files = {}

    def upload(entry, bdata):
        h = hash(bdata)
        entrypath = os.path.join(root, "_pool", h)
        manifest_files[entry] = h
        if not bucket.exists(entrypath):
            iodata = io.BytesIO(bdata)
            iodata.seek(0)
            if prnt:
                prnt("Uploading new version of [%s]" % entry)
            bucket.upload_blob(iodata, entrypath, meta_data={}, content_type="text/json")
    for doc,content in conf["documents"].items():
        upload(utils.DOCUMENTS_PATH[doc], json.dumps(content, indent=4).encode("utf-8"))
    for blob,jblob in conf["blobs"].items():
        upload(utils.BLOBS_PATH[blob], utils.jblob2bytes(jblob))

    manifest = {
        "meta": conf["meta"],
        "files": manifest_files,
    }
    upload_manifest(manifest, bucket, root, None, prnt=prnt)  # no config name; always upload to manifest.json
