#!/bin/bash

/bootstrap/initial-bucket-export.sh 1>/dev/stdout 2>/dev/stderr &

# Enable TLS if required secrets are present -- k8s environments
if [ -f /run/secrets/confsslcrt/confsslcrt ]; then
	sed -i 's/# TLS_K8S //' /init/nginx.conf
fi

/usr/sbin/nginx -g 'daemon off;'
