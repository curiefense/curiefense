import json
from curieconf.utils import DOCUMENTS_PATH, BLOBS_PATH, BLOBS_BOOTSTRAP


bootstrap_config_json = json.load(open("config.batch.json"))

vec_action = {
        "id": "default",
        "type": "custom"
}

vec_limit = {
    "id": "f971e92459e2",
    "name": "New Rate Limit Rule rrt",
    "description": "New Rate Limit Rule",
    "timeframe": "180",
    "key": [{"attrs": "remote_addr"}],
    "thresholds": [{"limit": "3", "action": "default"}],
    "include": ["blacklist"],
    "exclude": ["whitelist"],
    "pairwith": {"self": "self"},
}


vec_securitypolicy = {
    "id": "__default__",
    "name": "default entry",
    "match": "__default__",
    "map": [
        {
            "limit_ids": [],
            "content_filter_active": True,
            "acl_active": True,
            "content_filter_profile": "__default__",
            "acl_profile": "__default__",
            "name": "default",
            "match": "/",
        }
    ],
}


vec_contentfilterrule = {
    "id": "100000",
    "name": "100000",
    "msg": "SQLi Attempt (Conditional Operator Detected)",
    "operand": "\\s(and|or)\\s+\\d+\\s+.*between\\s.*\\d+\\s+and\\s+\\d+.*",
    "severity": 5,
    "certainity": 5,
    "category": "sqli",
    "subcategory": "statement injection",
}


vec_contentfilterprofile = {
    "id": "__default__",
    "name": "default contentfilter",
    "ignore_alphanum": True,
    "max_header_length": 1024,
    "max_cookie_length": 1024,
    "max_arg_length": 1024,
    "max_headers_count": 42,
    "max_cookies_count": 42,
    "max_args_count": 512,
    "args": {
        "names": [
            {
                "key": "optnamearg",
                "reg": "^[A-F]+$",
                "restrict": False,
                "exclusions": {},
            },
        ],
        "regex": [
            {
                "key": "optregexarg",
                "reg": "^[G-J]{3}$",
                "restrict": False,
                "exclusions": {},
            },
        ],
    },
    "headers": {
        "names": [
            {
                "key": "optnamehdr",
                "reg": "^[A-F]+$",
                "restrict": False,
                "exclusions": {},
            },
        ],
        "regex": [
            {
                "key": "optregexhdr",
                "reg": "^[G-J]{3}$",
                "restrict": False,
                "exclusions": {},
            },
        ],
    },
    "cookies": {
        "names": [
            {
                "key": "optnameck",
                "reg": "^[A-F]+$",
                "restrict": False,
                "exclusions": {},
            },
        ],
        "regex": [
            {
                "key": "optregexck",
                "reg": "^[G-J]{3}$",
                "restrict": False,
                "exclusions": {},
            },
        ],
    },
}


vec_aclprofile = {
    "id": "__default__",
    "name": "default-acl",
    "allow": ["allow-change"],
    "allow_bot": ["office", "qa", "devops", "sadasff"],
    "deny_bot": ["datacenter", "graylist", "vpn", "tor"],
    "passthrough": ["internalip"],
    "deny": ["blocked-countries"],
    "force_deny": ["blacklist"],
}


vec_globalfilter = {
    "id": "ed8f6efb",
    "active": True,
    "name": "Spamhaus DROP",
    "source": "https://www.spamhaus.org/drop/drop.txt",
    "mdate": "2020-05-31T05:28:47.410Z",
    "notes": "; notes",
    "tags": ["blacklists", "spamhaus"],
    "rule": {
        "entries": [
            {
                "relation": "OR",
                "entries": [
                    ["ip", "1.10.16.0/20"],
                    ["ip", "1.19.0.0/16"],
                ],
            }
        ]
    },
}


vec_geolite2asn = {"format": "base64", "blob": "AAAABBBB"}
vec_geolite2country = {"format": "base64", "blob": "AAAABBBB"}


vec_documents = {
    "actions": vec_action,
    "ratelimits": vec_limit,
    "securitypolicies": vec_securitypolicy,
    "contentfilterrules": vec_contentfilterrule,
    "contentfilterprofiles": vec_contentfilterprofile,
    "aclprofiles": vec_aclprofile,
    "globalfilters": vec_globalfilter,
}

vec_blobs = {
    "geolite2asn": vec_geolite2asn,
    "geolite2country": vec_geolite2country,
}
bootstrap_small_config_json = {
    "config": {"id": "small_test_config", "date": "2020-04-10T09:54:15"},
    "documents": {k: [v] for k, v in vec_documents.items()},
    "blobs": vec_blobs,
}
