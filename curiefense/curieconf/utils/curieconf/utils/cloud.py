import os
import hashlib
import types
from curieconf import utils
from curieconf.utils.config import (
    GOOGLE_APPLICATION_CREDENTIALS, 
    CURIE_S3_ACCESS_KEY, 
    CURIE_S3_SECRET_KEY, 
    CURIE_S3CFG_PATH, 
    CURIE_MINIO_HOST, 
    CURIE_MINIO_ACCESS_KEY, 
    CURIE_MINIO_SECRET_KEY, 
    CURIE_MINIOCFG_PATH, 
    CURIE_MINIO_SECURE
)

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
            credentials_path = GOOGLE_APPLICATION_CREDENTIALS
            storage = get_driver(DriverName.GOOGLESTORAGE)(credentials_path)
        elif u.scheme == "s3":
            akey = CURIE_S3_ACCESS_KEY
            skey = CURIE_S3_SECRET_KEY
            if not (akey and skey):
                s3pth = CURIE_S3CFG_PATH
                if not s3pth:
                    for s3pth in [
                        os.path.expanduser("~/.s3cfg"),
                        "/var/run/secrets/s3cfg/s3cfg",
                        "/var/run/secrets/s3cfg",
                    ]:
                        if os.path.isfile(s3pth):
                            break
                    else:
                        raise Exception(
                            "Did not find any credential to access %s" % url
                        )
                from configparser import ConfigParser

                c = ConfigParser()
                c.read(s3pth)
                akey = c.get("default", "access_key")
                skey = c.get("default", "secret_key")
            storage = get_driver(DriverName.S3)(
                key=akey, secret=skey
            )  # , url=os.path.dirname(u.path))
        elif u.scheme == "azblob":
            storage = get_driver(DriverName.AZURE)(os.path.dirname(u.path))
        elif u.scheme == "minio":
            endpoint = CURIE_MINIO_HOST
            akey = CURIE_MINIO_ACCESS_KEY
            skey = CURIE_MINIO_SECRET_KEY
            if not (akey and skey):
                miniopth = CURIE_MINIOCFG_PATH
                if not miniopth:
                    for miniopth in [
                        os.path.expanduser("~/.miniocfg"),
                        "/var/run/secrets/miniocfg/miniocfg",
                        "/var/run/secrets/miniocfg",
                    ]:
                        if os.path.isfile(miniopth):
                            break
                    else:
                        raise Exception(
                            "Did not find any credential to access %s" % url
                        )
                from configparser import ConfigParser

                c = ConfigParser()
                c.read(miniopth)
                akey = c.get("default", "access_key")
                skey = c.get("default", "secret_key")
            if CURIE_MINIO_SECURE == "FALSE":
                # testing environment only: no TLS
                msec = False
            else:
                msec = True
            storage = get_driver(DriverName.MINIO)(
                endpoint=endpoint, key=akey, secret=skey, secure=msec
            )
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
    mkey = os.path.join(
        root, "manifest.%s.json" % configname if configname else "manifest.json"
    )
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
            bucket.upload_blob(
                iodata, entrypath, meta_data={}, content_type="text/json"
            )

    for doc, content in conf["documents"].items():
        upload(utils.DOCUMENTS_PATH[doc], json.dumps(content, indent=4).encode("utf-8"))
    for blob, jblob in conf["blobs"].items():
        upload(utils.BLOBS_PATH[blob], utils.jblob2bytes(jblob))

    manifest = {
        "meta": conf["meta"],
        "files": manifest_files,
    }
    upload_manifest(
        manifest, bucket, root, None, prnt=prnt
    )  # no config name; always upload to manifest.json


def upload_file(file_path, url, prnt=None):
    bucket, root = get_bucket(url)
    file_name = os.path.basename(file_path)
    remote_path = os.path.join(root, file_name)

    with open(file_path, "rb") as file:
        file_data = io.BytesIO(file.read())
        file_data.seek(0)

    if prnt:
        prnt(f"Uploading file '{file_path}' to '{remote_path}'")

    bucket.upload_blob(file_data, remote_path, meta_data={})
