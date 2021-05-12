#!/bin/bash

CURL="curl --silent --write-out %{http_code}\n -H Content-Type:application/json --output /dev/null"

configure_links () {
	URL="http://confserver/api/v1/db/system/k/links/"
	DATA="{\"kibana_url\":\"$KIBANA_UI_URL\",\"grafana_url\":\"$GRAFANA_UI_URL\"}"
	if $CURL "$URL" -X PUT -H 'Accept: application/json' --data-raw "$DATA"| grep -qv 200; then
		sleep 5
		configure_links
	fi
}

# Configure links to kibana & grafana
if [ -n "$GRAFANA_UI_URL" ] && [ -n "$KIBANA_UI_URL" ]; then
	echo "Updating links in the curieconf database..."
	configure_links
	echo "Links successfully configured in the curieconf database."
fi


# Enable TLS if required secrets are present -- docker environments
if [ -f /run/secrets/uisslcrt ]; then
	sed -i 's/# TLS-DOCKERCOMPOSE //' /init/nginx.conf
fi

# Enable TLS if required secrets are present -- k8s environments
if [ -f /run/secrets/uisslcrt/uisslcrt ]; then
	sed -i 's/# TLS-K8S //' /init/nginx.conf
fi

/usr/sbin/nginx -g 'daemon off;'
