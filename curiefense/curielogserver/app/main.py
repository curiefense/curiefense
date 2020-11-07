from curielogserver import app, get_default_dbconfig
import os
import time
import psycopg2.pool
from psycopg2 import OperationalError

retries = 10
while retries > 0:
    retries -= 1
    try:
        app.config['postgreSQL_pool'] = psycopg2.pool.ThreadedConnectionPool(1, 20, get_default_dbconfig())
        break
    except OperationalError:
        if retries == 0:
            raise
        time.sleep(1)
