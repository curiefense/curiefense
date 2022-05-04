#!/bin/bash

NAME="$1"
REQSIZEKB="$2"
NODE_IP=$(kubectl get nodes -o json|jq '.items[0].status.addresses[]|select(.type=="ExternalIP").address'|tr -d '"')
JAEGER_URL="http://$NODE_IP:30686/jaeger/api/"
LOCUST_URL="http://$NODE_IP:30400"
OUTDIR="${RESULTS_DIR:stats-locust}/$NAME"
mkdir -p "$OUTDIR"

for UC in 10 50 100 200 400 500 600; do
	# 1 user is about 6 RPS
	echo -en "\n=== $UC concurrent users, request size $REQSIZEKB kB ===\n"
	OUTNAME="uc-$UC-$REQSIZEKB"
	TESTID="$OUTNAME-$NAME"
	curl "${LOCUST_URL}/swarm" -X POST -H 'Content-Type: application/x-www-form-urlencoded; charset=UTF-8' --data-raw "user_count=$UC&spawn_rate=1000&host=http%3A%2F%2Fistio-ingressgateway.istio-system%2Fratings&cf_reqsize=$REQSIZEKB&cf_testid=$TESTID"
	sleep 60

	curl "${JAEGER_URL}traces?limit=1500&lookback=1h&service=istio-ingressgateway.istio-system&tags=%7B%22http.url%22%3A%22http%3A%2F%2Fistio-ingressgateway.istio-system%2Fratings%2Finvalid%2F${TESTID}%22%7D" --output "$OUTDIR/jaeger-$OUTNAME.json"
	kubectl top --namespace istio-system pod |grep ingress | awk '{print "{\"cpu\":\"" $2 "\", \"ram\":\"" $3 "\"}"}' > "$OUTDIR/resources-$OUTNAME.json"
	curl "${LOCUST_URL}/stats/requests" --output "$OUTDIR/locust-$OUTNAME.json"
	curl "${LOCUST_URL}/stop"
done

