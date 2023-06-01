import base64
import json
from urllib.parse import urlencode
from typing import Set, Any, Tuple

tags = ["allow", "allowbot", "deny", "denybot", "forcedeny", "passthrough"]

reqs = []


def make_request(st: Set[str], b64: bool = False) -> Tuple[Any, str]:
    name = " + ".join(st)
    if b64:
        name += " (b64)"
    r = {
        "name": name,
        "headers": {
            ":path": "/direct?"
            + urlencode(
                [(x, base64.b64encode(x.encode("utf-8")) if b64 else x) for x in st]
            ),
            ":method": "GET",
            "x-forwarded-for": "23.129.64.253",
            "user-agent": "dummy",
            ":authority": "localhost:30081",
        },
        "response": {
            "tags": [
                "host:localhost:30081",
                "cookies:0",
                "headers:2",
                "args:%d" % (len(st) * (2 if b64 else 1)),
                "geo-city:nil",
                "geo-continent-name:north-america",
                "geo-continent-code:na",
                "geo-region:nil",
                "geo-subregion:nil",
                "geo-org:emerald-onion",
                "all",
                "geo-country:united-states",
                "ip:23-129-64-253",
                "aclname:from-tags",
                "aclid:fromtags",
                "contentfiltername:default-contentfilter",
                "contentfilterid:--default--",
                "securitypolicy:default-entry",
                "geo-asn:396507",
                "securitypolicy-entry:direct-association",
                "bot",
                "sante",
            ],
            "action": "pass",
        },
    }
    for x in st:
        r["response"]["tags"].append(x)
    if "forcedeny" in st:
        r["response"]["action"] = "custom_response"
        r["response"]["block_mode"] = True
        r["response"]["status"] = 403
    elif "passthrough" in st:
        pass
    elif "deny" in st and "allow" not in st:
        r["response"]["action"] = "custom_response"
        r["response"]["block_mode"] = True
        r["response"]["status"] = 403
    elif "allowbot" in st:
        pass
    elif "denybot" in st:
        r["response"]["action"] = "custom_response"
        r["response"]["block_mode"] = True
        r["response"]["status"] = 247
        # todo: add "challenge" tag too?
        r["response"]["tags"].append("challenge-phase01")
    if "status" in r["response"]:
        r["response"]["tags"].append("status:%d" % r["response"]["status"])
        r["response"]["tags"].append(
            "status-class:%dxx" % (r["response"]["status"] / 100)
        )

    return (r, name)


seen: Set[str] = set()

for x in tags:
    for y in tags:
        for z in tags:
            (r, nm) = make_request({x, y, z})
            if nm not in seen:
                seen.add(nm)
                reqs.append(r)
            (r, nm) = make_request({x, y, z}, b64=True)
            if nm not in seen:
                seen.add(nm)
                reqs.append(r)

print(json.dumps(reqs))
