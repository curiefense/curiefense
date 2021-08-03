#! /usr/bin/env python
import os
import sys
import argparse
import datetime
import io
import json
from enum import Enum

import typer
import yaml

import simple_rest_client.exceptions


from curieconf import confclient
from curieconf import utils
from curieconf.utils import cloud
from cloudstorage.exceptions import NotFoundError, CloudStorageError

state = argparse.Namespace()


class OutputFormats(object):
    def json(x):
        return json.dumps(x, indent=4)

    def yaml(x):
        return yaml.dump(x)

    default = json


def output(raw, err=False):
    try:
        formatted = getattr(OutputFormats, state.output, OutputFormats.default)(raw)
    except:
        typer.echo(raw, err=err)
    else:
        typer.echo(formatted, err=err)


BlobsEnum = Enum("Blobs", {name: name for name in utils.BLOBS_PATH})
DocsEnum = Enum("Docs", {name: name for name in utils.DOCUMENTS_PATH})


###############
### CONFIGS ###
###############

configs = typer.Typer()


@configs.command()
def list():
    output(state.api.configs.list().body)


@configs.command()
def list_versions(config: str):
    output(state.api.configs.list_versions(config).body)


@configs.command()
def get(
    config: str,
    version: str = typer.Option(None, "--version", "-v", help="Get a specific version"),
):
    if version is None:
        output(state.api.configs.get(config).body)
    else:
        output(state.api.configs.get_version(config, version).body)


@configs.command()
def delete(config: str):
    output(state.api.configs.delete(config).body)


@configs.command()
def create(
    fname: str = typer.Argument(None),
    name: str = typer.Option(
        None, "--name", "-n", help="Override configuration name from configuration file"
    ),
):
    f = open(fname) if fname else sys.stdin
    if name is None:
        output(state.api.configs.create(body=json.load(f)).body)
    else:
        output(state.api.configs.create_name(name, body=json.load(f)).body)


@configs.command()
def update(config: str, fname: str = typer.Argument(None)):
    f = open(fname) if fname else sys.stdin
    output(state.api.configs.update(config, body=json.load(f)).body)


@configs.command()
def clone(config: str, new_name: str):
    output(state.api.configs.clone_name(config, new_name, body={}).body)


@configs.command()
def revert(config: str, version: str):
    output(state.api.configs.revert(config, version).body)


#############
### Blobs ###
#############

blobs = typer.Typer()


@blobs.command()
def list(config: str):
    output(state.api.blobs.list(config).body)


@blobs.command()
def get(
    config: str,
    blob: BlobsEnum,
    fname: str = typer.Argument(None),
    version: str = typer.Option(None, "--version", "-v"),
):
    f = open(fname, "wb") if fname else sys.stdout.buffer
    if version is None:
        b = state.api.blobs.get(config, blob.value).body
    else:
        b = state.api.blobs.get_version(config, blob.value, version).body
    f.write(utils.jblob2bytes(b))


@blobs.command()
def delete(config: str, blob: BlobsEnum):
    output(state.api.blobs.delete(config, blob.value).body)


@blobs.command()
def revert(config: str, blob: BlobsEnum, version: str):
    output(state.api.blobs.revert(config, blob.value, version).body)


@blobs.command()
def create(config: str, blob: BlobsEnum, fname: str = typer.Argument(None)):
    f = open(fname, "rb") if fname else sys.stdin.buffer
    jb = utils.bytes2jblob(f.read())
    output(state.api.blobs.create(config, blob.value, body=jb).body)


@blobs.command()
def update(config: str, blob: BlobsEnum, fname: str = typer.Argument(None)):
    f = open(fname, "rb") if fname else sys.stdin.buffer
    jb = utils.bytes2jblob(f.read())
    output(state.api.blobs.update(config, blob.value, body=jb).body)


@blobs.command()
def list_versions(config: str, blob: BlobsEnum):
    output(state.api.blobs.list_versions(config, blob.value).body)


#################
### Documents ###
#################

docs = typer.Typer()


@docs.command()
def list(config: str):
    output(state.api.documents.list(config).body)


@docs.command()
def get(config: str, doc: DocsEnum, fname: str = typer.Argument(None), version=None):
    f = open(fname, "w") if fname else sys.stdout
    if version is None:
        r = state.api.documents.get(config, doc.value)
    else:
        r = state.api.documents.get_version(config, doc.value, version)
    output(r.body)


@docs.command()
def delete(config: str, doc: DocsEnum):
    output(state.api.documents.delete(config, doc.value).body)


@docs.command()
def revert(config: str, doc: DocsEnum, version: str):
    output(state.api.documents.revert(config, doc.value, version).body)


@docs.command()
def create(config: str, doc: DocsEnum, fname: str = typer.Argument(None)):
    f = open(fname, "r") if fname else sys.stdin
    output(state.api.documents.create(config, doc.value, body=json.load(f)).body)


@docs.command()
def update(config: str, doc: DocsEnum, fname: str = typer.Argument(None)):
    f = open(fname, "r") if fname else sys.stdin
    output(state.api.documents.update(config, doc.value, body=json.load(f)).body)


@docs.command()
def list_versions(config: str, doc: DocsEnum):
    output(state.api.documents.list_versions(config, doc.value).body)


###############
### Entries ###
###############

entries = typer.Typer()


@entries.command()
def list(config: str, doc: DocsEnum):
    output(state.api.entries.list(config, doc.value).body)


@entries.command()
def get(
    config: str,
    doc: DocsEnum,
    entry: str,
    fname: str = typer.Argument(None),
    version=None,
):
    f = open(fname, "w") if fname else sys.stdout
    if version is None:
        r = state.api.entries.get(config, doc.value, entry)
    else:
        r = state.api.entries.get_version(config, doc.value, entry, version)
    output(r.body)


@entries.command()
def delete(config: str, doc: DocsEnum, entry: str):
    output(state.api.entries.delete(config, doc.value, entry).body)


@entries.command()
def revert(config: str, doc: DocsEnum, entry: str, version: str):
    output(state.api.entries.revert(config, doc.value, entry, version).body)


@entries.command()
def create(config: str, doc: DocsEnum, fname: str = typer.Argument(None)):
    f = open(fname, "r") if fname else sys.stdin
    output(state.api.entries.create(config, doc.value, body=json.load(f)).body)


@entries.command()
def update(config: str, doc: DocsEnum, entry: str, fname: str = typer.Argument(None)):
    f = open(fname, "r") if fname else sys.stdin
    output(state.api.entries.update(config, doc.value, entry, body=json.load(f)).body)


@entries.command()
def edit(config: str, doc: DocsEnum, entry: str, fname: str = typer.Argument(None)):
    f = open(fname, "r") if fname else sys.stdin
    output(state.api.entries.edit(config, doc.value, entry, body=json.load(f)).body)


@entries.command()
def edit_one(config: str, doc: DocsEnum, entry: str, jsonpath: str, jsonvalue: str):
    value = json.loads(jsonvalue)
    edit = {"path": jsonpath, "value": value}
    output(state.api.entries.edit(config, doc.value, entry, body=edit).body)


@entries.command()
def list_versions(config: str, doc: DocsEnum, entry: str):
    output(state.api.entries.list_versions(config, doc.value, entry).body)


###############
### CONFIGS ###
###############

db = typer.Typer()


@db.command()
def list():
    output(state.api.db.list().body)


@db.command()
def list_versions():
    output(state.api.db.list_versions().body)


@db.command()
def get(
    db: str,
    version: str = typer.Option(None, "--version", "-v", help="Get a specific version"),
):
    if version is None:
        output(state.api.db.get(db).body)
    else:
        output(state.api.db.get_version(db, version).body)


@db.command()
def delete(db: str):
    output(state.api.db.delete(db).body)


@db.command()
def create(name: str, fname: str = typer.Argument(None)):
    f = open(fname) if fname else sys.stdin
    output(state.api.db.create(name, body=json.load(f)).body)


@db.command()
def update(db: str, fname: str = typer.Argument(None)):
    f = open(fname) if fname else sys.stdin
    output(state.api.db.update(db, body=json.load(f)).body)


@db.command()
def query(db: str, fname: str = typer.Argument(None)):
    f = open(fname) if fname else sys.stdin
    output(state.api.db.query(db, body=json.load(f)).body)


@db.command()
def revert(db: str, version: str):
    output(state.api.db.revert(db, version).body)


key = typer.Typer()


@key.command()
def list(db: str):
    output(state.api.key.list(db).body)


@key.command()
def get(db: str, key: str):
    output(state.api.key.get(db, key).body)


@key.command()
def list_versions(db: str, key: str):
    output(state.api.key.list_versions(db, key).body)


@key.command()
def set(db: str, key: str, fname: str = typer.Argument(None)):
    f = open(fname) if fname else sys.stdin
    output(state.api.key.set(db, key, body=json.load(f)).body)


@key.command()
def delete(db: str, key: str):
    output(state.api.key.delete(db, key).body)


###############
### PUBLISH ###
###############

tool = typer.Typer()


@tool.command("publish")
def publish(config: str, url: str, version: str = None):
    """
    Request the API server to publish a given config version to a cloud bucket
    URL
    """
    body = [{"name": "url from CLI", "url": url}]
    if version is None:
        output(state.api.tools.publish(config, body=body).body)
    else:
        output(state.api.tools.publish_version(config, version, body=body).body)


############
### SYNC ###
############


sync = typer.Typer()


@sync.command()
def export(config: str, url: str):
    """
    Download a given config version from API server, then export it to a cloud
    bucket URL
    """
    conf = state.api.configs.get(config).body
    cloud.export(conf, url, prnt=typer.echo)


@sync.command("import")
def import_():
    """bucket -> mongodb"""
    pass


@sync.command()
def dump(version: str, target_path: str):
    """mongodb -> fs"""
    conf = state.api.batch.retrieve(version).body

    os.makedirs(target_path, exist_ok=True)
    for tbl, fname in utils.DOCUMENT_PATHS.items():
        if os.path.isabs(fname):
            fname = fname[1:]
        dst = os.path.join(target_path, fname)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        json.dump(conf[tbl], open(dst, "w"), indent=4)


@sync.command()
def load(source_path: str, version: str):
    """fs -> Web API(mongodb)"""

    batch = {
        tbl: json.load(open(os.path.join(source_path, fname)))
        for tbl, fname in utils.DOCUMENT_PATHS.items()
    }
    output(state.api.batch.create(version, body=batch).body)


@sync.command()
def push(source_path: str, target_url: str, version: str = ""):
    """fs -> bucket"""
    try:
        bucket, target_path = cloud.get_bucket(target_url)
    except CloudStorageError:
        typer.echo(f"ERROR: Could not access bucket {target_url}")
        raise typer.Exit(code=1)
    manifest = {}
    for root, dirs, files in os.walk(source_path):
        if root.endswith("pool_"):
            continue
        for f in files:
            if f.endswith(".json") or f.endswith("mmdb"):
                fpath = os.path.join(root, f)
                fname = os.path.join(os.path.relpath(root, source_path), f)
                h = cloud.hash(open(fpath, "rb").read())
                target_name = os.path.join(target_path, "_pool", h)
                manifest[fname] = h
                if not bucket.exists(target_name):
                    bucket.upload_blob(fpath, target_name, meta_data={})
    cloud.upload_manifest(manifest, bucket, target_path, version)


@sync.command()
def pull(
    manifest_url: str,
    target_path: str,
    on_conf_change: str = typer.Option(
        "", help="Shell command to execute if configuration changed"
    ),
):
    """bucket -> fs"""
    try:
        bucket, manifest_path = cloud.get_bucket(manifest_url)
    except CloudStorageError:
        typer.echo(f"ERROR: Could not access bucket {manifest_url}")
        raise typer.Exit(code=1)
    bucketroot = os.path.dirname(manifest_path)

    pool_path = os.path.join(target_path, "_pool")
    os.makedirs(pool_path, exist_ok=True)

    changes = 0

    # download manifest
    manif = io.BytesIO()
    try:
        bucket.get_blob(manifest_path).download(manif)
    except NotFoundError:
        typer.echo(f"ERROR: Manifest {manifest_path} does not exist")
        raise typer.Exit(code=1)
    manifest_str = manif.getvalue().decode("utf-8")
    manifest = json.loads(manifest_str)

    conf_path = os.path.join(
        target_path, "configs", manifest["meta"]["id"], manifest["meta"]["version"]
    )
    os.makedirs(conf_path, exist_ok=True)
    typer.echo("Creating configuration [%s]" % conf_path)

    open(os.path.join(conf_path, "manifest.json"), "w").write(manifest_str)

    # synchronize pool
    for fname, h in manifest["files"].items():
        pool_file = os.path.join(pool_path, h)
        if not os.path.exists(pool_file):
            typer.echo("Pulling new version of [%s]" % fname)
            try:
                blobpath = os.path.join(bucketroot, "_pool", h)
                blob = bucket.get_blob(blobpath)
            except NotFoundError:
                typer.echo(f"ERROR: Manifest {blobpath} does not exist")
                raise typer.Exit(code=1)
            blob.download(pool_file)
            changes += 1
        if os.path.isabs(fname):
            fname = fname[1:]
        conf_file = os.path.join(conf_path, fname)
        conf_file_dir = os.path.dirname(conf_file)
        os.makedirs(conf_file_dir, exist_ok=True)
        rel_link = os.path.relpath(pool_file, conf_file_dir)
        try:
            old_link = os.readlink(conf_file)
        except OSError:
            try:
                os.unlink(conf_file)
            except FileNotFoundError:
                pass
            os.symlink(rel_link, conf_file)
            changes += 1
        else:
            if old_link != rel_link:
                # ~atomic symlink replacement
                os.symlink(rel_link, conf_file + ".tmp")
                os.replace(conf_file + ".tmp", conf_file)
                changes += 1
    current_conf = os.path.join(target_path, "current")
    new_conf = os.path.relpath(conf_path, target_path)
    try:
        old_conf = os.readlink(current_conf)
    except OSError:
        try:
            os.unlink(current_conf)
        except FileNotFoundError:
            pass
        os.symlink(new_conf, current_conf)
        changes += 1
    else:
        if old_conf != new_conf:
            # ~atomic symlink replacement
            os.symlink(new_conf, current_conf + ".tmp")
            os.replace(current_conf + ".tmp", current_conf)
            changes += 1

    if changes and on_conf_change:
        os.system(on_conf_change)


###########
## Main ###
###########

app = typer.Typer()

app.add_typer(configs, name="conf")
app.add_typer(blobs, name="blob")
app.add_typer(docs, name="doc")
app.add_typer(entries, name="entry")
app.add_typer(db, name="db")
app.add_typer(key, name="key")
app.add_typer(tool, name="tool")
app.add_typer(sync, name="sync")


@app.callback()
def main_options(
    output: str = typer.Option(
        "json", "--output", "-o", help="Output format: json, yaml"
    ),
    baseurl: str = typer.Option(
        os.environ.get("CURIECONF_BASE_URL", "http://localhost:5000/api/v2/"),
        "--base-url",
        "-u",
        help="Base url for API",
    ),
):
    state.output = output
    state.api = confclient.get_api(baseurl)


def main():
    try:
        app()
    except simple_rest_client.exceptions.ErrorWithResponse as e:
        output(e.response.body, err=True)


if __name__ == "__main__":
    main()
