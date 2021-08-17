import time
import datetime
import requests
import json
import re

from jsonschema import validate, ValidationError

from .task import Task

SCHEMAFILE = "/global-filters.schema"


@Task.register("update")
class TaskUpdate(Task):
    parsers = {
        # "ip": re.compile("^(?P<val>(([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3})|(([0-9a-f]*:+){1,8}))(/[0-9]{1,2})) *([;#] *(?P<comment>.*$))?", re.IGNORECASE),
        # "asn": re.compile(r"^as(?P<val>[0-9]{3,6}) *([#;//?] *(?P<comment>.*$))?", re.IGNORECASE),
        "ip": re.compile(
            r"^[^;#](([0-9a-f]{1,}\:+){1,7}[0-9a-f]{1,}([:]+)?(/\d{1,3})?|(\d{1,3}\.){3}\d{1,3}(/\d{1,2})?)((\s+)?([#;//?].+))?",
            re.IGNORECASE,
        ),
        "asn": re.compile(r"(AS\d{3,6})((\s+)?([#;//?].+))?", re.IGNORECASE),
    }

    def check_args(self, list_ids, branches):
        assert (
            type(list_ids) is list or list_ids == "*"
        ), f"Unrecognized list ids: {list_ids!r}"
        assert (
            type(branches) is list or branches == "*"
        ), f"Unrecognized branch list: {branches!r}"
        self.list_ids = list_ids
        self.branches = branches

    def validate_schema(self, data):
        with open(SCHEMAFILE) as json_file:
            schema = json.load(json_file)
            try:
                validate(instance=data, schema=schema)
                return True
            except Exception as err:
                self.log.error(f"Exception while parsing schema {err!r}")
                return False

    def parse_native(self, data):
        if self.validate_schema(data):
            # return entire document
            return data

    def parse_re(self, data):
        lines = data.splitlines()
        if len(lines) > 0:
            midlist = int(len(lines) / 2)
            ## first,last and one from the middle. at least one must match.
            if any(
                (
                    self.parsers["ip"].match(lines[0]),
                    self.parsers["ip"].match(lines[-1]),
                    self.parsers["ip"].match(lines[midlist]),
                )
            ):

                for line in lines:
                    match = self.parsers["ip"].match(line)
                    if match:
                        g = match.groups()
                        if g:
                            yield ["ip", g[0], g[-1] and g[-1][:128]]

            elif any(
                (
                    self.parsers["asn"].match(lines[0]),
                    self.parsers["asn"].match(lines[-1]),
                    self.parsers["asn"].match(lines[midlist]),
                )
            ):
                for line in lines:
                    match = self.parsers["asn"].match(line)
                    if match:
                        g = match.groups()
                        if g:
                            yield ["asn", g[0], g[-1] and g[-1][:128]]

            else:
                yield None

    def iterate_object(self, obj):
        typename = type(obj).__name__
        if typename == "list":
            return obj

        elif typename == "dict":
            return obj.values()

    def parse_object(self, obj):
        got = self.iterate_object(obj)
        for element in got:
            typename = type(element).__name__
            if typename in ["dict", "list"]:
                for j in self.parse_object(element):
                    yield j

            else:
                match = self.parsers["ip"].match(element)
                if match:
                    g = match.groups()
                    if g:
                        yield ["ip", g[0], g[-1] and g[-1][:128]]
                else:
                    match = self.parsers["asn"].match(element)
                    if match:
                        g = match.groups()
                        if g:
                            yield ["asn", g[0], g[-1] and g[-1][:128]]

    def readurl(self, url):
        try:
            data = requests.get(url)
            data.raise_for_status()
            if "application/json" in data.headers.get(
                "Content-Type", data.headers.get("content-type")
            ):
                self.log.info(f"readurl got JSON")
                return data.json()
            else:
                self.log.info(f"readurl got text")
                return data.text
        except:
            return None

    def parse(self, lst):
        url = lst.get("source")
        data = self.readurl(url)
        if data:
            typename = type(data).__name__
            self.log.info(f"parse results data type {typename}")
            if typename not in ("dict", "list"):
                entries = list(self.parse_re(data))
                if len(entries) > 0 and entries[0]:
                    lst["entries"] = list(entries)
                    lst["mdate"] = datetime.datetime.now().isoformat()

            else:
                native_format = self.parse_native(data)
                if native_format:
                    self.log.info(f"native format found")
                    # native format, update the whole entry
                    lst = native_format
                else:
                    entries = list(self.parse_object(data))
                    if len(entries) > 0 and entries[0]:
                        self.log.info(f"parseobject found entries")
                        lst["entries"] = list(entries)
                        lst["mdate"] = datetime.datetime.now().isoformat()

            return lst

        self.log.error(f"Could not fetch data from: {url}")
        return False

    def action(self):

        branches = self.branches
        if branches == "*":
            l = self.confserver.configs.list().body
            branches = [b["id"] for b in l]
            self.log.info(f"Working on all branches: {branches!r}")
        for branch in branches:
            lstids = self.list_ids
            if lstids == "*":
                lstids = self.confserver.entries.list(branch, "profilinglists").body
                self.log.info(f"Working on lists: {lstids!r}")
            for lstid in lstids:
                self.log.info(f"Downloading {lstid} in branch {branch}")
                try:
                    lst = self.confserver.entries.get(
                        branch, "profilinglists", lstid
                    ).body
                except Exception as e:
                    self.log.error(
                        f"Could not download {lstid} in branch {branch}: {e}"
                    )
                    continue
                source = lst.get("source")
                if not source:
                    self.log.error(
                        f"Profiling list {lstid} is missing 'source' attribute or attribute is empty"
                    )
                    continue
                if source == "self-managed":
                    self.log.info(f"List {lstid} is self-managed")
                    continue

                self.log.info(f"Downloading update from {source}")
                try:
                    lst = self.parse(lst)
                    if lst:
                        self.confserver.entries.update(
                            branch, "profilinglists", lstid, body=lst
                        )
                        self.log.info(f"Updated {lstid} in branch {branch}")

                except Exception as e:
                    self.log.error(
                        f"Could not download url [{source}] for list {lstid}"
                    )
                    continue


@Task.register("publish")
class TaskPublish(Task):
    def check_args(self, branches):
        assert (
            type(branches) is list or branches == "*"
        ), f"Unrecognized branch list: {branches!r}"
        self.branches = branches

    def action(self):
        sysdb = self.confserver.db.get("system").body

        branches = self.branches
        if branches == "*":
            l = self.confserver.configs.list().body
            branches = [b["id"] for b in l]
            self.log.info(f"Working on all branches: {branches!r}")
        for branch in branches:
            for brbuck in sysdb["branch_buckets"]:
                if brbuck["name"] == branch:
                    buckets = [
                        buck
                        for buck in sysdb["buckets"]
                        if buck["name"] in brbuck["buckets"]
                    ]
                    self.log.info(
                        f"Publishing branch [{branch}] to buckets {buckets!r}"
                    )
                    res = self.confserver.tools.publish(branch, body=buckets).body
                    if res["ok"]:
                        self.log.info(f"Publish status: {res!r}")
                    else:
                        self.log.error(f"Publish status: {res!r}")


@Task.register("update_and_publish")
class TaskUpdateAndPublish(TaskUpdate, TaskPublish):
    def check_args(self, list_ids, branches):
        TaskUpdate.check_args(self, list_ids, branches)
        TaskPublish.check_args(self, branches)

    def action(self):
        TaskUpdate.action(self)
        TaskPublish.action(self)
