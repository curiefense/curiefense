#! /usr/bin/env python

import os
import datetime
import logging
from curieconf import confclient
from . import tasker


def start(options):
    tasker.start(options)


def runtask(options):
    tasker.runtask(options)


def main(args=None):
    import argparse

    parser = argparse.ArgumentParser()

    parser.add_argument(
        "--base-url",
        default=os.environ.get("CURIECONF_BASE_URL", "http://confserver/api/v2/"),
    )
    parser.add_argument(
        "--task-db-name", default=os.environ.get("CURIETASKER_DB_NAME", "tasks")
    )
    parser.add_argument("--task-file", type=argparse.FileType("r"), default=None)
    parser.add_argument("--verbose", "-v", action="count", default=3)
    parser.add_argument("--quiet", "-q", action="count", default=0)
    parser.add_argument("--debug", "-d", action="store_true", default=False)

    subparsers = parser.add_subparsers(dest="commandd", required=True)

    p_start = subparsers.add_parser("start", help="start the tasker loop")
    p_start.set_defaults(func=start)
    p_start.add_argument("--timemark", default=datetime.datetime.now())

    p_runtask = subparsers.add_parser("runtask", help="run a single task")
    p_runtask.set_defaults(func=runtask)
    p_runtask.add_argument("taskid")

    options = parser.parse_args(args)

    options.api = confclient.get_api(options.base_url)
    options.verbosity = (
        1 if options.debug else max(1, 50 + 10 * (options.quiet - options.verbose))
    )
    logging.basicConfig(
        format="%(asctime)s: %(levelname)-5s: %(name)s: %(message)s",
        level=options.verbosity,
    )

    options.func(options)


if __name__ == "__main__":
    main()
