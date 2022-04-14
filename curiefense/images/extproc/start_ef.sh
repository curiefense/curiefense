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
	/usr/local/bin/cf-externalprocessing --loglevel "$LOGLEVEL" --configpath /cf-config/current/config --trustedhops "$XFF" --elasticsearch http://elasticsearch:9200/
	sleep 1
done