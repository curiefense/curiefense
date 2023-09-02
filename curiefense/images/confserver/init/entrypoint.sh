#!/bin/bash
# make sure git trusts the persistennt directories.

git config --global --add safe.directory /cf-persistent-config/confdb
git config --global --add safe.directory /cf-persistent-config/bootstrap-repo

if [ "$INIT_GIT_ON_STARTUP" = "yes" ]; then
  # used when running with docker-compose, which does not have initContainers or similar features
  # due to unexpected failures, bootstrap will run up to 3 times to better eliminate these failures (usually fixed after one failure)
  for i in {1..3}; do
    if /bootstrap/bootstrap_config.sh; then
      break
    fi
    echo "error - entrypoint.sh bootstrap_config failed, attempt number $i, trying again in 5 seconds"
    sleep 5
  done
fi

/bootstrap/initial-bucket-export.sh &

exec "$@"
