import json
from typing import Set, Any, Tuple

tags = ["allow", "allowbot", "deny", "denybot", "forcedeny", "bypass"]

reqs = []


def make_request(st: Set[str]) -> Tuple[Any, str]:
    name = " + ".join(st)
    r = {
        "name": name,
        "headers": {
            ":path": "/direct?" + "&".join(["%s=%s" % (x, x) for x in st]),
            ":method": "GET",
            "x-forwarded-for": "23.129.64.253",
            "user-agent": "dummy",
            ":authority": "localhost:30081",
        },
        "response": {
            "tags": [
                "all",
                "geo:united-states",
                "ip:23-129-64-253",
                "aclname:from-tags",
                "aclid:fromtags",
                "wafname:default-waf",
                "wafid:--default--",
                "urlmap:default-entry",
                "asn:396507",
                "urlmap-entry:direct-association",
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
        r["response"]["status"] = 503
    elif "bypass" in st:
        pass
    elif "deny" in st and "allow" not in st:
        r["response"]["action"] = "custom_response"
        r["response"]["block_mode"] = True
        r["response"]["status"] = 503
    elif "allowbot" in st:
        pass
    elif "denybot" in st:
        r["response"]["action"] = "custom_response"
        r["response"]["block_mode"] = True
        r["response"]["status"] = 247
        r["response"]["tags"].append("challenge-phase01")
    return (r, name)


seen: Set[str] = set()

for x in tags:
    for y in tags:
        for z in tags:
            (r, nm) = make_request({x, y, z})
            if nm not in seen:
                seen.add(nm)
                reqs.append(r)

print(json.dumps(reqs))
