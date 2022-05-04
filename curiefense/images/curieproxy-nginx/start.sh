#!/bin/bash

rm -f /run/rsyslogd.pid
rsyslogd -n &

# shellcheck disable=SC2016
envsubst '${TARGET_ADDRESS},${TARGET_PORT}' < /etc/nginx/conf.d/default.template > /etc/nginx/conf.d/default.conf
/usr/local/openresty/bin/openresty -g "daemon off;"
