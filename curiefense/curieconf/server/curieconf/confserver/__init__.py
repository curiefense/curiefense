#! /usr/bin/env python3
import json
import os

from .backend import Backends
import uvicorn
import logging
from curieconf.confserver.v3 import api
from curieconf.utils.config import SWAGGER_BASE_PATH, CURIECONF_HOST, CURIECONF_PORT, CURIECONF_TRUSTED_USERNAME_HEADER, CURIECONF_TRUSTED_EMAIL_HEADER

from fastapi import FastAPI, Request, HTTPException
from fastapi.exceptions import RequestValidationError
from fastapi.responses import PlainTextResponse
from prometheus_fastapi_instrumentator import Instrumentator
from werkzeug.exceptions import HTTPException as WerkzeugHTTPException

app = FastAPI(docs_url=SWAGGER_BASE_PATH)
app.include_router(api.router)


@app.on_event("startup")
async def startup():
    Instrumentator().instrument(app).expose(app)


## Import all versions
from .v3 import api as api_v3

logging.basicConfig(
    handlers=[logging.StreamHandler()],
    level=logging.INFO,
    format="[%(asctime)s] {%(pathname)s:%(lineno)d} %(levelname)s - %(message)s",
    datefmt="%H:%M:%S",
)
logger = logging.getLogger("confserver")


@app.exception_handler(RequestValidationError)
async def validation_exception_handler(request: Request, exc: RequestValidationError):
    return PlainTextResponse(str(exc), status_code=409)


# this is catching flasks' "abort" from the gitbackend
@app.exception_handler(WerkzeugHTTPException)
async def werkzeug_exception_handler(request: Request, exc: WerkzeugHTTPException):
    return PlainTextResponse(str(exc), status_code=exc.code)


@app.exception_handler(HTTPException)
async def http_exception_exception_handler(request: Request, exc: HTTPException):
    return PlainTextResponse(str(exc.detail), status_code=exc.status_code)


def drop_into_pdb(app, exception):
    import sys
    import pdb
    import traceback

    traceback.print_exc()
    pdb.post_mortem(sys.exc_info()[2])


def main(args=None):
    # only called when running manually, not through uwsgi
    global mongo
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--dbpath", "--db", help="API server db path", required=True)
    parser.add_argument("-d", "--debug", action="store_true", default=False)
    parser.add_argument("--pdb", action="store_true", default=False)
    parser.add_argument(
        "-H", "--host", default=CURIECONF_HOST
    )
    parser.add_argument(
        "-p", "--port", type=int, default=int(CURIECONF_PORT)
    )
    parser.add_argument(
        "--trusted-username-header",
        type=str,
        default=CURIECONF_TRUSTED_USERNAME_HEADER,
    )
    parser.add_argument(
        "--trusted-email-header",
        type=str,
        default=CURIECONF_TRUSTED_EMAIL_HEADER,
    )

    options = parser.parse_args(args)

    # TODO - find replacements for got_request_exception
    # if options.pdb:
    #     flask.got_request_exception.connect(drop_into_pdb)

    try:
        app.backend = Backends.get_backend(app, options.dbpath)
        app.options = options.__dict__
        uvicorn.run(app, host=options.host, port=options.port)
    finally:
        pass


if __name__ == "__main__":
    main()
