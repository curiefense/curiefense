#!/bin/bash
# This script is called by /start/sh

/bin/sed -i 's/tmp/uwsgi/' /etc/nginx/conf.d/nginx.conf
