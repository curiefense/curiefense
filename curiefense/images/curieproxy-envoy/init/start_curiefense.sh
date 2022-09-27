#! /bin/bash

if [ ! -e /cf-config/bootstrap ]
then
	cp -va /bootstrap-config /cf-config/bootstrap
fi

if [ ! -e /cf-config/current ]
then
	ln -s bootstrap /cf-config/current
fi

TADDRA="${TARGET_ADDRESS_A:-echo}"
TPORTA="${TARGET_PORT_A:-8080}"
TADDRB="${TARGET_ADDRESS_B:-juicebox}"
TPORTB="${TARGET_PORT_B:-3000}"
XFF="${XFF_TRUSTED_HOPS:-1}"
ENVOY_LOG_LEVEL="${ENVOY_LOG_LEVEL:-error}"
FILEBEAT="${FILEBEAT:-yes}"

sed -e "s/XFF_TRUSTED/$XFF/" /etc/envoy/envoy.yaml.head > /etc/envoy/envoy.yaml
if [ -f /run/secrets/curieproxysslcrt ]; then
	cat /etc/envoy/envoy.yaml.tls >> /etc/envoy/envoy.yaml
fi
sed -e "s/TARGET_ADDRESS_A/$TADDRA/" -e "s/TARGET_PORT_A/$TPORTA/" -e "s/TARGET_ADDRESS_B/$TADDRB/" -e "s/TARGET_PORT_B/$TPORTB/" /etc/envoy/envoy.yaml.tail >> /etc/envoy/envoy.yaml

while true
do
	if [ "$FILEBEAT" = "yes" ]
	then
		# shellcheck disable=SC2086
		/usr/local/bin/envoy -c /etc/envoy/envoy.yaml --service-cluster proxy --log-level "$ENVOY_LOG_LEVEL" $ENVOY_ARGS \
			| grep --line-buffered -v '^-$' \
			| /usr/bin/filebeat --path.config /etc
	else
		# shellcheck disable=SC2086
		/usr/local/bin/envoy -c /etc/envoy/envoy.yaml --service-cluster proxy --log-level "$ENVOY_LOG_LEVEL" $ENVOY_ARGS
	fi
	sleep 1
done
