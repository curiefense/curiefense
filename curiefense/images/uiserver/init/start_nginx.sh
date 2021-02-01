#!/bin/bash
if [ -f /run/secrets/uisslcrt ]; then
	sed -i 's/# TLS-DOCKERCOMPOSE //' /init/nginx.conf
fi

if [ -f /run/secrets/uisslcrt/uisslcrt ]; then
	sed -i 's/# TLS-K8S //' /init/nginx.conf
fi

if [ -n "$CURIELOGSERVER_DISABLED" ]; then
	cp /init/http.es.conf /init/http.conf
	cp /init/tls-k8s.es.conf /init/tls-k8s.conf
fi

/usr/sbin/nginx -g 'daemon off;'
