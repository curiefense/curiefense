#!/bin/bash -e

if [ ! -e /cf-config/bootstrap ]
then
	cp -va /bootstrap-config /cf-config/bootstrap
fi

if [ ! -e /cf-config/current ]
then
	ln -s bootstrap /cf-config/current
fi

if [ -n "$BOOTSTRAP_ONLY" ]; then
	exit 0
fi

echo "Curiefense installed."
echo "Now starting istio pilot."

exec /usr/local/bin/pilot-agent "$@"
