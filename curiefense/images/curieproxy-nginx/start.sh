#!/bin/bash
FILEBEAT="${FILEBEAT:-yes}"
# shellcheck disable=SC2016
envsubst '${TARGET_ADDRESS_A},${TARGET_PORT_A},${TARGET_ADDRESS_B},${TARGET_PORT_B}' < /etc/nginx/conf.d/default.template > /etc/nginx/conf.d/default.conf
if [ "$FILEBEAT" = "yes" ]
then
  /usr/local/openresty/bin/openresty -g "daemon off;" | grep -v '^.$' | /usr/bin/filebeat --path.config /etc
else
  /usr/local/openresty/bin/openresty -g "daemon off;"
fi