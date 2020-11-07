#! /usr/bin/env python3

import os
import flask
from flask import Flask, current_app
from .backend import Backends

from flask_cors import CORS


## Import all versions
from .v1 import api as api_v1

app = Flask(__name__)

CORS(app, resources={r'/*': { 'origins': '*'}})

app.register_blueprint(api_v1.api_bp, url_prefix="/api/v1")


def drop_into_pdb(app, exception):
    import sys
    import pdb
    import traceback
    traceback.print_exc()
    pdb.post_mortem(sys.exc_info()[2])


def main(args=None):
    global mongo
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--dbpath", "--db", help="API server db path", required=True)
    parser.add_argument("-d", "--debug", action="store_true", default=False)
    parser.add_argument("--pdb", action="store_true", default=False)
    parser.add_argument("-H", "--host", default=os.environ.get("CURIECONF_HOST", "127.0.0.1"))
    parser.add_argument("-p", "--port", type=int, default=int(os.environ.get("CURIECONF_PORT","5000")))

    options = parser.parse_args(args)

    if options.pdb:
        flask.got_request_exception.connect(drop_into_pdb)

    try:
        with app.app_context():
            current_app.backend = Backends.get_backend(app, options.dbpath)
            current_app.options = options
            app.run(debug=options.debug, host=options.host, port=options.port)
    finally:
        pass

if __name__ == '__main__':
    main()
