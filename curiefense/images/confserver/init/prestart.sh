#!/bin/bash
# This script is called by /start.sh

/bin/sed -i 's/tmp/uwsgi/' /etc/nginx/conf.d/nginx.conf

if [ "$INIT_GIT_ON_STARTUP" = "yes" ]; then
	# used when running with docker-compose, which does not have initContainers or similar features
	/bootstrap/bootstrap_config.sh
fi
