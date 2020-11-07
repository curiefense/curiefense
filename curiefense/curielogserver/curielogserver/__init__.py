from flask import Flask, request, Blueprint, g, abort, got_request_exception

from flask_restplus import Resource, Api, fields, marshal, reqparse
import os
import json
from datetime import datetime, timedelta
import decimal
from . import ratelimitrecommendation

import psycopg2
from psycopg2 import pool

class LogJSONEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, datetime):
            return obj.isoformat()
        if isinstance(obj, decimal.Decimal):
            return float(obj)
        return json.JSONEncoder.default(self, obj)

app = Flask(__name__)

api_bp = Blueprint("ap", "curiefense")
api = Api(api_bp, version="1.0", title="Curiefense log server API v1.0")

app.register_blueprint(api_bp, url_prefix="/api/v1")


app.config["RESTPLUS_JSON"] = {"cls": LogJSONEncoder}

m_exec = api.model("SQL request", {
    "statement": fields.String(required=True),
#    "parameters": fields.List(fields.String),
})

m_analyze = api.model("SQL analysis", {
    "action": fields.String(required=True),
    "parameters": fields.Nested(api.model("SQL analysis parameters", {
        "timeframe": fields.Integer,
        "urlmap": fields.String,
        "urlmapentry": fields.String,
        "include": fields.List(fields.List(fields.String)),
        "exclude": fields.List(fields.List(fields.String)),
        "key": fields.List(fields.List(fields.String)),
    }))
})

analyze_actions = ["rate-limit-recommendation"]

def get_db():
    if 'db' not in g:
        g.db = app.config['postgreSQL_pool'].getconn()
    return g.db


def get_default_dbconfig():
    """
    Get default db config string from environment variables
    """
    defaultdb = os.environ.get("CURIELOGSERVER_DB")
    if defaultdb:
        return defaultdb
    password_filename = os.environ.get("CURIELOGSERVER_DBPASSWORD_FILE", "")
    if password_filename:
        with open(password_filename, "r") as f:
            password = f.readline().rstrip("\r\n")
    else:
        password = os.environ.get("CURIELOGSERVER_DBPASSWORD", "")
    return ("host=%s dbname=curiefense user=%s password=%s" %
            (os.environ.get("CURIELOGSERVER_DBHOST", ""),
             os.environ.get("CURIELOGSERVER_DBUSER", ""),
             password)
            )

def execute_sql_request(stmt, params):
    db = get_db()
    cursor = db.cursor()
    try:
        cursor.execute(stmt, params)
    except Exception as e:
        abort(400, "Cannot execute statement: %s" % e)
    result = cursor.fetchall()
    return result


@app.teardown_appcontext
def close_conn(e):
    db = g.pop('db', None)
    if db is not None:
        app.config['postgreSQL_pool'].putconn(db)


@api.route("/exec/")
class ExecResource(Resource):
    @api.expect(m_exec, validate=True)
    def post(self):
        "Execute an SQL request on the log db"
        stmt = request.json["statement"]
        params = request.json.get("parameters",[])
        return execute_sql_request(stmt, params)

@api.route("/analyze/")
class ExecResource(Resource):
    @api.expect(m_analyze, validate=True)
    def post(self):
        "Execute an analysis SQL request using the log db"
        action = request.json.get("action")
        params = request.json.get("parameters", {})
        if action is None or action not in analyze_actions:
            abort(400, "Unrecognized or no action provided")
        if action == "rate-limit-recommendation":
            input_args = {
                "yaml_file_name": "/curielogserver/curielogserver/ratelimitec_postgresql.yaml",
                "startdate": (datetime.now() - timedelta(days=7)).strftime("%Y-%m-%d %T"),
                "enddate": datetime.now().strftime("%Y-%m-%d %T"),
                "timeframe":  params["timeframe"],
                "urlmap":  params["urlmap"],
                "urlmapentry":  params["mapentry"],
                "include":  params["include"],
                "exclude":  params["exclude"],
                "key_composition":  params["key"],
            }
            stmt = ratelimitrecommendation.rate_limit_recommend(input_args)
        return execute_sql_request(stmt, params)

def drop_into_pdb(app, exception):
    import sys
    import pdb
    import traceback
    traceback.print_exc()
    pdb.post_mortem(sys.exc_info()[2])

def main(args=None):
    defaultdb = get_default_dbconfig()

    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--db", default=defaultdb)
    parser.add_argument("-d", "--debug", action="store_true", default=False)
    parser.add_argument("--pdb", action="store_true", default=False)
    parser.add_argument("-H", "--host", default=os.environ.get("CURIELOGSERVER_HOST", "127.0.0.1"))
    parser.add_argument("-p", "--port", type=int, default=int(os.environ.get("CURIELOGSERVER_PORT","5000")))

    options = parser.parse_args(args)

    app.config['postgreSQL_pool'] = psycopg2.pool.ThreadedConnectionPool(1, 20, options.db)

    if options.pdb:
        got_request_exception.connect(drop_into_pdb)

    app.run(debug=options.debug, host=options.host, port=options.port)


if __name__ == '__main__':
    main()
