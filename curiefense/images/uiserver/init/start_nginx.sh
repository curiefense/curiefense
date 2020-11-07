#!/bin/bash
if [ -f /run/secrets/uisslcrt ]; then
	sed -i 's/# TLS-DOCKERCOMPOSE //' /init/nginx.conf
fi

if [ -f /run/secrets/uisslcrt/uisslcrt ]; then
	sed -i 's/# TLS-K8S //' /init/nginx.conf
fi
/usr/sbin/nginx -g 'daemon off;'
