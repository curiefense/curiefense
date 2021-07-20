#!/bin/bash

rm -f /run/rsyslogd.pid
rsyslogd -n &

/usr/local/openresty/bin/openresty -g "daemon off;"
