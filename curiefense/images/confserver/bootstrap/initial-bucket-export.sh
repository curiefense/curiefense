#!/bin/bash

if [ ! "$BOOTSTRAP_BUCKET_ON_STARTUP" = "yes" ]; then
	exit 0
fi

if [ -e "/bucket/local/manifest.json" ]; then
	echo "Bucket already exists, not exporting"
	exit 0
fi

# Wait a few seconds, then call API to export
RETRIES=0

if echo "$CURIE_BUCKET_LINK" | grep -Eq ':///?[^/]*/prod/manifest.json' ; then
	# case 1: user has kept the "scheme://authority/prod/manifest.json" structure
	# keep only "scheme://authority/", remove the path
	REQUEST_TEMPLATE="/bootstrap/db.conf.json"
	# shellcheck disable=SC2001
	REPL_BUCKET="$(echo "$CURIE_BUCKET_LINK"|sed 's!\(\S\+\:///\?[^/]\+/\).*!\1!')"
	BUCKET="${REPL_BUCKET}prod"
else
	# user has made another choice, do a single branch only
	REQUEST_TEMPLATE="/bootstrap/db-singlebranch.conf.json"
	# shellcheck disable=SC2001
	REPL_BUCKET="$(echo "$CURIE_BUCKET_LINK"|sed 's!\(\S\+\://.*/\).*$!\1!')"
	BUCKET="$REPL_BUCKET"
fi


ESCAPED_REPL_BUCKET="${REPL_BUCKET//\//\\\/}"
URL="http://localhost/api/v3/db/system/k/publishinfo/"

# First, configure a single bucket to match $BUCKET
STATUS=1
until [ "$RETRIES" -ge 10 ]
do
	echo "Trying to configure bucket $BUCKET..."
	sleep 5
	if sed "s/s3.*\//$ESCAPED_REPL_BUCKET/" "$REQUEST_TEMPLATE" | curl --silent --show-error --fail -X PUT "$URL" -H  "accept: application/json" -H  "Content-Type: application/json" -d @-; then
		STATUS=0
		break
	fi
	RETRIES=$((RETRIES+1))
done

if [ "$STATUS" -eq "1" ]; then
	echo "Configuration of bucket $BUCKET unsuccessful, exiting"
	exit 1
else
	echo "Configuration of bucket $BUCKET successful"
fi

URL="http://localhost/api/v3/tools/publish/prod/"

# Export configuration to $BUCKET
STATUS=1
until [ "$RETRIES" -ge 10 ]
do
	echo "Trying to request export to bucket $BUCKET..."
	sleep 5
	if curl --silent --show-error --fail -X PUT "$URL" -H  "accept: application/json" -H  "Content-Type: application/json" -d "[{\"name\": \"prod\", \"url\": \"$BUCKET\"}]"; then
		STATUS=0
		break
	fi
	RETRIES=$((RETRIES+1))
done

if [ "$STATUS" -eq "1" ]; then
	echo "Export to bucket $BUCKET unsuccessful, exiting"
	exit 1
else
	echo "Export to bucket $BUCKET successful"
fi
