#!/bin/bash

if [ "$INIT_GIT_ON_STARTUP" = "yes" ]; then
	# used when running with docker-compose, which does not have initContainers or similar features
	/bootstrap/bootstrap_config.sh
fi

/bootstrap/initial-bucket-export.sh &

exec "$@"
