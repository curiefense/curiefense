#!/bin/bash

/bootstrap/initial-bucket-export.sh 1>/dev/stdout 2>/dev/stderr &

/usr/sbin/nginx -g 'daemon off;'
