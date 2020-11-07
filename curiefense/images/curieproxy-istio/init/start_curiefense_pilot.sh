#!/bin/bash -e

if [ ! -e /config/bootstrap ]
then
	cp -va /bootstrap-config /config/bootstrap
fi

if [ ! -e /config/current ]
then
	ln -s bootstrap /config/current
fi

if [ -n "$BOOTSTRAP_ONLY" ]; then
	exit 0
fi

echo "Curiefense installed."
echo "Now starting istio pilot."

exec /usr/local/bin/pilot-agent "$@"
