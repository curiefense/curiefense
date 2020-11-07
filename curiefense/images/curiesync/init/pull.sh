#!/bin/bash -e

PERIOD=10

echo "Run mode is [${RUN_MODE}]"


if [ "$RUN_MODE" = "SYNC_ONCE" ]; then
    echo "Synchronizing once"
    curieconf_cli sync pull "${CURIE_BUCKET_LINK}" /config
    exit 0
fi

if [ "$RUN_MODE" = "COPY_BOOTSTRAP" ]; then
    echo "Copying bootstrap config"
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


if [ "$RUN_MODE" = "PERIODIC_SYNC" -o -z "$RUN_MODE" ]; then
    echo "Synchronizing conf every $PERIOD seconds"
    while :;
    do
        echo "Pulling ${CURIE_BUCKET_LINK}"
        curieconf_cli sync pull "${CURIE_BUCKET_LINK}" /config
        echo "Sleeping"
        sleep $PERIOD
    done
fi

