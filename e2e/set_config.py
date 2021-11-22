#!/usr/bin/env python3
# Used for e2e tests to set configuration

import test_e2e
import argparse

parser = argparse.ArgumentParser()
parser.add_argument(
    "-u", "--base-url", help="Base url for API", default="http://localhost:5000/api/v2/"
)
parser.add_argument(
    "CONFIGNAME", choices=["denyall", "defaultconfig", "contentfilter-and-acl"]
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
