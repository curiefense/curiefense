#! /bin/bash

if [ ! -e /cf-config/bootstrap ]
then
	cp -va /bootstrap-config /cf-config/bootstrap
fi

if [ ! -e /cf-config/current ]
then
	ln -s bootstrap /cf-config/current
fi

XFF="${XFF_TRUSTED_HOPS:-1}"
LOGLEVEL="${EXTPROC_LOG_LEVEL:-debug}"

while true
do
	# shellcheck disable=SC2086
	/usr/local/bin/cf-externalprocessing --handle-replies --loglevel "$LOGLEVEL" --configpath /cf-config/current/config --trustedhops "$XFF" $ELASTICSEARCH
	sleep 1
done