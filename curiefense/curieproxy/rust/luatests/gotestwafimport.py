"""Convert the YAML data from https://github.com/wallarm/gotestwaf"""
import sys
import os
from typing import Any, List
from yaml import load
import json
import base64

try:
    from yaml import CLoader as Loader
except ImportError:
    from yaml import Loader
from urllib.parse import quote_plus

target = sys.argv[1]


def default_query(name: str) -> Any:
    return {
        "response": {
            "action": "custom_response",
            "block_mode": False,
            "status": 403,
        },
        "name": name,
        "headers": {
            "x-request-id": "e6acdce3-e076-4f0d-9a22-9d82fe01ba60",
            "x-forwarded-for": "23.129.64.253",
            ":path": "/test",
            ":method": "GET",
            ":authority": "localhost:30081",
        },
        "verbose": False,
    }


tests: List[Any] = []
for root, _, files in os.walk(target):
    if files:
        fp = root.endswith("false-pos")
        for f in files:
            fullname = os.path.join(root, f)
            data = load(open(fullname, "r"), Loader)
            for (pid, payload) in enumerate(data["payload"]):
                for encoder in data["encoder"]:
                    if encoder == "Plain":
                        epayload = payload
                    elif encoder == "URL":
                        epayload = quote_plus(payload)
                    elif encoder == "Base64Flat":
                        epayload = (
                            base64.encodebytes(payload.encode(encoding="utf-8"))
                            .decode(encoding="utf-8")
                            .replace("\n", "")
                            .rstrip("=")
                        )
                    else:
                        print("Unsupported encoding for %s: %s" % (data["type"], encoder), file=sys.stderr)
                        continue
                    for placeholder in data["placeholder"]:
                        q = default_query("%s %d/%s/%s" % (f, pid, encoder, placeholder))
                        if placeholder == "URLParam":
                            q["headers"][":path"] = "/test?param=" + epayload
                        elif placeholder == "URLPath":
                            q["headers"][":path"] = "/test/" + epayload
                        elif placeholder == "JSONRequest":
                            q["headers"]["content-type"] = "application/json"
                            q["body"] = json.dumps({"test": True, "content": payload})
                        elif placeholder == "JSONBody":
                            q["headers"]["content-type"] = "application/json"
                            q["body"] = payload
                        elif placeholder == "HTMLForm":
                            q["headers"]["content-type"] = "application/x-www-form-urlencoded"
                            q["body"] = "foo=1&bar=" + payload
                        elif placeholder == "HTMLMultipartForm":
                            q["headers"]["content-type"] = "multipart/form-data; boundary=AaB03x"
                            q["body"] = (
                                """--AaB03x\r\nContent-Disposition: form-data; name="submit-name"\r\n\r\n%s\r\n--AaB03x--"""
                                % payload
                            )
                        elif placeholder == "Header":
                            q["headers"]["myheader"] = epayload
                        else:
                            print("Unsupported placeholder for %s: %s" % (fullname, placeholder), file=sys.stderr)
                            continue
                        if fp:
                            q["response"] = {"action": "pass"}
                        tests.append(q)

print(json.dumps(tests))
