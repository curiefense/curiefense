#!/bin/bash

BASEDIR="$(dirname "$(readlink -f "$0")")"
rm -f "$BASEDIR/"log*.log

for ns in curiefense istio-system; do
	PODAPPS=$(kubectl get pods -n $ns -o go-template='{{range .items}} {{ .metadata.labels.app | or (index .metadata.labels "app.kubernetes.io/name")}}!{{.metadata.name}} {{end}}')
	for podapp in $PODAPPS; do
		IFS=! read -r app pod <<< "$podapp"

		PATTERNFILE="$BASEDIR/patterns/$ns-$app-patterns.txt"
		if [ ! -f "$PATTERNFILE" ]; then
			echo "Warning: pattern file missing: $PATTERNFILE, skipping checks for this pod"
			continue
		fi
		kubectl logs -n "$ns" "$pod" --all-containers |grep -vEf "$PATTERNFILE" > "$BASEDIR/log-$ns-$app.log"
	done
done
