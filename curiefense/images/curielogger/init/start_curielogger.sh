#!/bin/bash

CURL="curl --silent --write-out %{http_code}\n -H Content-Type:application/json --output /dev/null"

wait_for_es () {
	if $CURL -X GET "${ELASTICSEARCH_URL}_cluster/health?wait_for_status=yellow&timeout=10s"|grep -qv 200; then
		sleep 5
		wait_for_es
	fi
}

if [ -n "$CURIELOGGER_WAIT_FOR_ES" ]; then
	>&2 echo "Waiting for Elasticsearch"
	wait_for_es
fi

cd /init || exit 1
/bin/curielogger
