#!/usr/bin/env python3

import json
from os import environ
from sys import exit
import requests

host = environ.get("XTARGET_HOST", None)
if not host:
    print(
        f'''
XTARGET_HOST is mandatory.
XTARGET_PORT may also be provided, in case it is other than 80.

e.g.
    XTARGET_HOST=1.2.3.4 XTARGET_PORT=":31081"'''
    )
    exit()

port = environ.get("XTARGET_PORT", "")

ignore_headers = {
    ":authority": True,
    ":method": True,
    ":path": True,
    "x-forwarded-for": True,
    "x-request-id": True,
}
url_prefix = f"http://{host}{port}"


gotestwaf = json.load(open("blob.json", "r"))

gen_reqs = (
    (
        req["headers"].get(":method", "GET"),
        req["headers"].get(":path", "/"),
        req.get("body", None),
        req["headers"],
    )
    for req in gotestwaf
)

code = 0
for method, path, body, allheaders in gen_reqs:
    try:
        method = method.lower()
        headers = {h: allheaders[h] for h in allheaders if h not in ignore_headers}
        url = f"{url_prefix}{path}"

        if not body:
            code = requests.__dict__[method](f"{url}", headers=headers).status_code
        else:
            code = requests.__dict__[method](
                f"{url}", headers=headers, data=body.encode("utf-8")
            ).status_code

        print(f"got:{code}  -  {url}")
    except:
        pass
