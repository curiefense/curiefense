#!/bin/bash
export FILEBEAT="${FILEBEAT:-yes}"
export AGGREGATED_STATS_LOG_FILE="${AGGREGATED_LOG_FILE:-/var/log/nginx.metrics.json.log}"
export ACCESS_LOG="${NGINX_ACCESS_LOG:-/var/log/nginx.access.json.log}"
export ERROR_LOG="${NGINX_ERROR_LOG:-/var/log/nginx.error.log}"
export CF_LOG_LEVEL="${CF_LOG_LEVEL:-info}"
export NGINX_LOG_LEVEL="${NGINX_LOG_LEVEL:-info}"
# shellcheck disable=SC2016
mkdir -p /tmp/ngx-cache/current
mkdir -p /tmp/ngx-temp-body
mkdir -p /tmp/ngx-cache-tmp
/usr/local/bin/nginx-conf-watch.sh&

envsubst '${TARGET_ADDRESS_A},${TARGET_PORT_A},${TARGET_ADDRESS_B},${TARGET_PORT_B},${AGGREGATED_STATS_LOG_FILE},${ACCESS_LOG},${ERROR_LOG},${CF_LOG_LEVEL},${NGINX_LOG_LEVEL}' < /etc/nginx/conf.d/default.template > /etc/nginx/conf.d/default.conf
if [ "$FILEBEAT" = "yes" ]
then
  /usr/local/openresty/bin/openresty -g "daemon off;" | grep -v '^.$' | /usr/bin/filebeat --path.config /etc
else
  /usr/local/openresty/bin/openresty -g "daemon off;"
fi

