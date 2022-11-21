#!/bin/bash
# export FILEBEAT="${FILEBEAT}"
# export AGGREGATED_STATS_LOG_FILE="${AGGREGATED_LOG_FILE}"
# export ACCESS_LOG="${NGINX_ACCESS_LOG}"
# export NGINX_ERROR_LOG="${NGINX_ERROR_LOG}"
# export CF_LOG_LEVEL="${CF_LOG_LEVEL}"
# export NGINX_LOG_LEVEL="${NGINX_LOG_LEVEL}"
# shellcheck disable=SC2016
# envsubst '${TARGET_ADDRESS_A},${TARGET_PORT_A},${TARGET_ADDRESS_B},${TARGET_PORT_B},${AGGREGATED_STATS_LOG_FILE},${NGINX_ACCESS_LOG},${NGINX_ERROR_LOG},${CF_LOG_LEVEL},${NGINX_LOG_LEVEL}' < /etc/nginx/conf.d/default.template > /etc/nginx/conf.d/default.conf
envsubst '${TARGET_ADDRESS_A},${TARGET_PORT_A},${TARGET_ADDRESS_B},${TARGET_PORT_B},${AGGREGATED_STATS_LOG_FILE},${NGINX_ACCESS_LOG},${NGINX_ERROR_LOG},${CF_LOG_LEVEL},${NGINX_LOG_LEVEL}' < /usr/local/openresty/nginx/conf/nginx.conf > /usr/local/openresty/nginx/conf/nginx.conf.1
mv /usr/local/openresty/nginx/conf/nginx.conf.1 /usr/local/openresty/nginx/conf/nginx.conf

/usr/local/bin/nginx-conf-watch.sh&

if [ "$FILEBEAT" = "yes" ]
then
  /usr/local/openresty/bin/openresty -g "daemon off;" | grep -v '^.$' | /usr/bin/filebeat --path.config /etc
else
  /usr/local/openresty/bin/openresty -g "daemon off;"
fi
