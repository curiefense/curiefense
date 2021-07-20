#! /bin/bash

if [ ! -e /config/bootstrap ]
then
	cp -va /bootstrap-config /config/bootstrap
fi

if [ ! -e /config/current ]
then
	ln -s bootstrap /config/current
fi

TADDR="${TARGET_ADDRESS:-echo}"
TPORT="${TARGET_PORT:-8080}"
XFF="${XFF_TRUSTED_HOPS:-1}"
ENVOY_LOG_LEVEL="${ENVOY_LOG_LEVEL:-error}"

sed -e "s/XFF_TRUSTED/$XFF/" /etc/envoy/envoy.yaml.head > /etc/envoy/envoy.yaml
if [ -f /run/secrets/curieproxysslcrt ]; then
	cat /etc/envoy/envoy.yaml.tls >> /etc/envoy/envoy.yaml
fi
sed -e "s/TARGET_ADDRESS/$TADDR/" -e "s/TARGET_PORT/$TPORT/" /etc/envoy/envoy.yaml.tail >> /etc/envoy/envoy.yaml

while true
do
	# shellcheck disable=SC2086
    /usr/local/bin/envoy -c /etc/envoy/envoy.yaml --service-cluster proxy --log-level "$ENVOY_LOG_LEVEL" $ENVOY_ARGS #--concurrency 1 NO concurrency PICKS THE NUM OF CORES
	sleep 1
done

