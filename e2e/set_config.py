#!/usr/bin/env python3
# Used for e2e tests to set configuration

import test_e2e
import argparse

parser = argparse.ArgumentParser()
parser.add_argument(
    "-u", "--base-url", help="Base url for API", default="http://localhost:5000/api/v3/"
)
parser.add_argument(
    "CONFIGNAME",
    choices=["denyall", "defaultconfig", "contentfilter-and-acl", "flow-and-ratelimit"],
)
args = parser.parse_args()

cli = test_e2e.CliHelper(args.base_url)
acl = test_e2e.ACLHelper(cli)
if args.CONFIGNAME == "denyall":
    acl.reset_and_set_acl({"force_deny": "all"})
elif args.CONFIGNAME == "defaultconfig":
    cli.revert_and_enable(False, False)
    cli.publish_and_apply()
elif args.CONFIGNAME == "contentfilter-and-acl":
    cli.revert_and_enable(True, True)
    cli.publish_and_apply()
elif args.CONFIGNAME == "flow-and-ratelimit":
    # Test case that uses redis for every query. Tag-only/monitor to get the
    # worst possible case: both rate limit & flow control must be evaluated.
    version = cli.initial_version()
    cli.call(f"conf revert {test_e2e.TEST_CONFIG_NAME} {version}")
    # set flow control
    fl_rules = [
        {
            "id": "eff100010001",
            "name": "Flow Control Monitor",
            "description": "New Flow Control Notes and Remarks",
            "active": True,
            "include": ["all"],
            "exclude": [],
            "timeframe": 60,
            "action": {"type": "monitor"},
            "key": [{"attrs": "ip"}],
            "sequence": [
                {
                    "method": "GET",
                    "uri": "/ratings/invalid",
                    "cookies": {},
                    "headers": {"host": "www.example.com"},
                    "args": {},
                },
                {
                    "method": "POST",
                    "uri": "/ratings/invalid",
                    "cookies": {},
                    "headers": {"host": "www.example.com"},
                    "args": {},
                },
            ],
        }
    ]
    cli.call(
        f"doc update {test_e2e.TEST_CONFIG_NAME} flowcontrol /dev/stdin",
        inputjson=fl_rules,
    )
    # set rate limit
    rl_rules = [
        {
            "id": "ef1100010000",
            "name": "Rate Limit Example Rule 5/60",
            "description": "5 requests per minute",
            "timeframe": "60",
            "thresholds": [{"limit": "5", "action": {"type": "monitor"}}],
            "include": ["all"],
            "exclude": [],
            "key": [{"attrs": "ip"}],
            "pairwith": {"self": "self"},
        }
    ]
    cli.call(
        f"doc update {test_e2e.TEST_CONFIG_NAME} ratelimits /dev/stdin",
        inputjson=rl_rules,
    )
    # set securitypolicy
    securitypolicy = cli.call(f"doc get {test_e2e.TEST_CONFIG_NAME} securitypolicies")
    securitypolicy[0]["map"][0]["acl_active"] = False
    securitypolicy[0]["map"][0]["content_filter_active"] = False
    securitypolicy[0]["map"][0]["limit_ids"] = ["ef1100010000"]
    # ## edit
    cli.call(
        f"doc update {test_e2e.TEST_CONFIG_NAME} securitypolicies /dev/stdin",
        inputjson=securitypolicy,
    )
    cli.publish_and_apply()
