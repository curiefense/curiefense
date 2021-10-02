#!/bin/bash

BASEDIR="$(dirname "$(readlink -f "$0")")"
rm -f "$BASEDIR"/log*.log

COMPOSEFILE="$BASEDIR/../../deploy/compose/docker-compose.yaml"

docker-compose -f "$COMPOSEFILE" ps -q | while read -r container; do
		service=$(docker inspect -f '{{.Name}}' "$container"|cut -c2-)
		PATTERNFILE="$BASEDIR/patterns/compose-$service-patterns.txt"
		if [ ! -f "$PATTERNFILE" ]; then
			echo "Warning: pattern file missing: $PATTERNFILE, skipping checks for this pod"
			continue
		fi
		docker-compose -f "$COMPOSEFILE" logs "$service" \
			| sed 's/^[^|]*|[^ ]* //' \
			| grep -vE "^Attaching to ${service}$" \
			| grep -vEf "$PATTERNFILE" > "log-$service.log"
done
