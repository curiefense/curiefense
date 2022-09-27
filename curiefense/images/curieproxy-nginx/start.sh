#!/bin/bash
export FILEBEAT="${FILEBEAT:-yes}"
export AGGREGATED_STATS_LOG_FILE="${AGGREGATED_LOG_FILE:-/dev/stdout}"
export ACCESS_LOG="${NGINX_ACCESS_LOG:-/dev/stdout}"
export ERROR_LOG="${NGINX_ERROR_LOG:-/logs/error.log}"
export CF_LOG_LEVEL="${CF_LOG_LEVEL:-info}"
# shellcheck disable=SC2016
envsubst '${TARGET_ADDRESS_A},${TARGET_PORT_A},${TARGET_ADDRESS_B},${TARGET_PORT_B},${AGGREGATED_STATS_LOGFILE},${ACCESS_LOG},${ERROR_LOG},${CF_LOG_LEVEL}' < /etc/nginx/conf.d/default.template > /etc/nginx/conf.d/default.conf
if [ "$FILEBEAT" = "yes" ]
then
  /usr/local/openresty/bin/openresty -g "daemon off;" | grep -v '^.$' | /usr/bin/filebeat --path.config /etc
else
  /usr/local/openresty/bin/openresty -g "daemon off;"
fi