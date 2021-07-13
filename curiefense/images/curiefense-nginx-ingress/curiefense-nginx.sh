#!/bin/bash

rsyslogd -n -iNONE &

/curiesync/pull.sh &

/nginx-ingress "$@"
