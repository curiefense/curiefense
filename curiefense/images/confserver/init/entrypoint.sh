#!/bin/bash

# make sure git trusts the persistennt directories.
git config --global --add safe.directory /cf-persistent-config/confdb
git config --global --add safe.directory /cf-persistent-config/bootstrap-repo

if [ "$INIT_GIT_ON_STARTUP" = "yes" ]; then
	# used when running with docker-compose, which does not have initContainers or similar features
	/bootstrap/bootstrap_config.sh
fi

/bootstrap/initial-bucket-export.sh &

exec "$@"
