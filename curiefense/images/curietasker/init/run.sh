#!/bin/bash
set -o pipefail

# initialize db if needed.
URL="${CURIECONF_BASE_URL:-http://confserver/api/v2/}"
NSNAME="${CURIETASKER_NS_NAME:-tasks}"

CURL="curl --silent --show-error --fail"

echo "Determining whether the $NSNAME namespace should be created..."
RETRIES=0
STATUS=1
until [ "$RETRIES" -ge 10 ]
do
	if NSLIST=$($CURL -X GET "${URL}db/" -H  'accept: application/json'); then
		STATUS=0
		break
	fi
	sleep 5
	RETRIES=$((RETRIES+1))
done

if [ "$STATUS" -eq 1 ]; then
	echo "Could not determine whether the $NSNAME namespace should be created, exiting." > /dev/stderr
	exit 1
fi

if echo "$NSLIST" | grep -q "$NSNAME"; then
	echo "The $NSNAME namespace already exists."
else
	echo "Initializing the $NSNAME namespace..."
	RETRIES=0
	STATUS=1
	until [ "$RETRIES" -ge 10 ]
	do
		if $CURL -X POST "${URL}db/$NSNAME/" -H 'accept: application/json' -H 'Content-Type: application/json' -d '{"tasklist":[]}'; then
			STATUS=0
			break
		fi
		sleep 5
		RETRIES=$((RETRIES+1))
	done
	if [ "$STATUS" -eq 1 ]; then
		echo "Error while creating the $NSNAME DB, exiting." > /dev/stderr
		exit 1
	fi
	echo "The $NSNAME namespace has been created."
fi

# run curietasker
/usr/bin/python3 /usr/local/bin/curietasker start
