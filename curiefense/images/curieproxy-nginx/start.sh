#!/bin/bash

rsyslogd -n &

/usr/local/openresty/bin/openresty -g "daemon off;"
