#!/bin/bash -e

if [ -z "$POSTGRES_READONLY_PASSWORD_FILE" ]; then
	echo "Variable POSTGRES_READONLY_PASSWORD_FILE is not defined, exiting" 1>&2;
	exit 1
fi

RO_PASSWORD=$(head -n1 "$POSTGRES_READONLY_PASSWORD_FILE"|tr -d '\n'|tr -d '\r')

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
	CREATE ROLE logserver_ro LOGIN PASSWORD '$RO_PASSWORD';
EOSQL

echo "Password for read-only user has been updated"
