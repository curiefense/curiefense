#!/bin/bash -e

PERIOD=10

echo "Run mode is [${RUN_MODE}]"
QUIET="${CURIESYNC_QUIET:-false}"


info () {
    if [ "$QUIET" = "true" ]; then
        echo "$*"
    fi
}

if [ -f /etc/curiefense/curiesync.env ]; then
	# shellcheck disable=SC1091
	source /etc/curiefense/curiesync.env
fi

if [ "$RUN_MODE" = "SYNC_ONCE" ]; then
    info "Synchronizing once"
    curieconfctl sync pull "${CURIE_BUCKET_LINK}" /config
    exit 0
fi

if [ "$RUN_MODE" = "COPY_BOOTSTRAP" ]; then
    info "Copying bootstrap config"
    if [ ! -e /config/bootstrap ]
    then
        mkdir -p /config
        cp -va /bootstrap-config /config/bootstrap
    fi

    if [ ! -e /config/current ]
    then
        ln -s bootstrap /config/current
    fi
    exit 0
fi


if [ "$RUN_MODE" = "PERIODIC_SYNC" ] || [ -z "$RUN_MODE" ]; then
    info "Synchronizing conf every $PERIOD seconds"
    while :;
    do
        info "Pulling ${CURIE_BUCKET_LINK}"
        curieconfctl sync pull "${CURIE_BUCKET_LINK}" /config
        info "Sleeping"
        sleep $PERIOD
    done
fi

