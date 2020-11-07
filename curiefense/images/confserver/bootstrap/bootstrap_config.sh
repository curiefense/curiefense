#!/bin/bash

# to be run in an initContainer
# Will deploy specified configuration as a bootstrap config, if there is no config in /config/confdb
TARGETDIR="/config/confdb"

if [ -e "$TARGETDIR" ]; then
	echo "Config already present in $TARGETDIR, exiting"
	exit 0
fi
echo "Config directory $TARGETDIR is empty"

if [ -n "$IF_NO_CONFIG_PULL_FROM" ]; then
	echo "Cloning configuration from $IF_NO_CONFIG_PULL_FROM"
	git clone --mirror "$IF_NO_CONFIG_PULL_FROM" "$TARGETDIR"
else
	echo "No configuration found in $TARGETDIR, IF_NO_CONFIG_PULL_FROM is not defined: exiting"
	exit 1
fi
