#!/bin/bash

# Pre-requisites:
# * images built & pushed to the registry
# * gcloud access is set up
# * ~/s3cfg.yaml contains AWS secrets to be used for S3

BASEDIR="$(dirname "$(readlink -f "$0")")" 
if [ -z "$KUBECONFIG" ]; then
	KUBECONFIG="$(readlink -f "$(mktemp kubeconfig.XXXXX)")"
	export KUBECONFIG
	echo "KUBECONFIG is set to $KUBECONFIG"
fi
CLUSTER_NAME="${CLUSTER_NAME:-curiefense-perftest-gks}"
S3CFG_PATH=${S3CFG_PATH:-~/s3cfg.yaml}
DATE="$(date --iso=m)"
VERSION="${DOCKER_TAG:-$(git rev-parse --short=12 HEAD)}"
REGION=${REGION:-us-central1-a}

create_cluster () {
	echo "-- Create cluster $CLUSTER_NAME --"
	# 4 CPUs, 16GB
	gcloud container clusters create "$CLUSTER_NAME" --num-nodes=1 --machine-type=n2-standard-8 --region="$REGION"
	gcloud container clusters get-credentials --region="$REGION" "$CLUSTER_NAME"
}

install_helm () {
	echo "-- Install helm --"
	curl -fsSL -o get_helm.sh https://raw.githubusercontent.com/helm/helm/master/scripts/get-helm-3
	chmod 700 get_helm.sh
	./get_helm.sh
}

deploy_curiefense () {
	echo "-- Deploy curiefense --"
	if [ ! -f "$S3CFG_PATH" ]; then
		echo "$S3CFG_PATH does not exist. It must contain S3 credentials for curiefense configuration synchronization" > /dev/stderr
		exit 1
	fi
	kubectl create namespace curiefense
	kubectl create namespace istio-system
	kubectl apply -f "$S3CFG_PATH"
	kubectl apply -f "$BASEDIR/curiefense-helm/example-dbsecret.yaml"
	kubectl apply -f "$BASEDIR/curiefense-helm/example-uiserver-tls.yaml"
	if [ "$jaeger" = "y" ] || [ "$all" = "y" ]; then
		kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.8/samples/addons/jaeger.yaml
		kubectl apply -f "$BASEDIR/../e2e/latency/jaeger-service.yml"
	fi
	cd "$BASEDIR/istio-helm/" || exit 1
	./deploy.sh --set 'global.tracer.zipkin.address=zipkin.istio-system:9411' --set 'gateways.istio-ingressgateway.autoscaleMax=1'
	sleep 5
	cd "$BASEDIR/curiefense-helm/" || exit 1
	./deploy.sh
	kubectl apply -f "$BASEDIR/curiefense-helm/expose-services.yaml"
}

deploy_bookinfo () {
	echo "-- Deploy target: bookinfo app --"
	kubectl label namespace default istio-injection=enabled
	if [ ! -d "$BASEDIR/istio-1.9.3/" ]; then
		cd "$BASEDIR" || exit 1
		wget 'https://github.com/istio/istio/releases/download/1.9.3/istio-1.9.3-linux-amd64.tar.gz'
		tar -xf 'istio-1.9.3-linux-amd64.tar.gz'
	fi
	kubectl apply -f "$BASEDIR/istio-1.9.3/samples/bookinfo/platform/kube/bookinfo.yaml"
	kubectl apply -f "$BASEDIR/istio-1.9.3/samples/bookinfo/networking/bookinfo-gateway.yaml"
	# also expose the "ratings" service directly
	kubectl apply -f "$BASEDIR/../e2e/latency/ratings-virtualservice.yml"
	# deploy 5 replicas to handle the test load
	kubectl scale deployment ratings-v1 --replicas 5
}

install_fortio () {
	kubectl apply -f "$BASEDIR/../e2e/latency/fortio-deployment.yml"
	kubectl apply -f "$BASEDIR/../e2e/latency/fortio-service.yml"
}

run_fortio () {
	CONNECTIONS=$1
	QPS=$2
	DURATION=$3
	# use an int, changing for each fortio run; used to query jaeger traces
	TEST_NUMBER=$4
	OUT_PATH=$5
	if [ -z "$FORTIO_URL" ]; then
		NODE_IP=$(kubectl get nodes -o json|jq '.items[0].status.addresses[]|select(.type=="ExternalIP").address'|tr -d '"')
		INGRESS_PORT=$(kubectl -n istio-system get service -n istio-system istio-ingressgateway -o jsonpath='{.spec.ports[?(@.name=="http2")].nodePort}')
		FORTIO_URL="http://$NODE_IP:30100/fortio/"
		JAEGER_URL="http://$NODE_IP:30686/jaeger/api/"
		echo "Waiting for fortio to become reachable..."
		for _ in $(seq 1 30); do
			if curl --silent --fail "$FORTIO_URL" >/dev/null; then
				break
			fi
			sleep 1
		done
		echo "Pre-heat request"
		curl -s "http://$NODE_IP:$INGRESS_PORT/ratings/preheat" > /dev/null
	fi

	# target: http://istio-ingressgateway.istio-system/ratings/invalid-$tag -- JSON response
	# setting the id to "invalid" makes the service quickly return a constant json document 
	DATA_URL=$(curl "$FORTIO_URL?labels=Fortio&url=http%3A%2F%2Fistio-ingressgateway.istio-system%2Fratings%2Finvalid-$TEST_NUMBER&qps=$QPS&t=${DURATION}s&n=&c=$CONNECTIONS&p=50%2C+75%2C+90%2C+99%2C+99.9&r=0.0001&H=User-Agent%3A+fortio.org%2Ffortio-1.11.3&runner=http&resolve=&save=on&load=Start"|grep -o --color "[0-9-]*_Fortio.json"|head -n1)
	OUTNAME="$DURATION-$QPS-$CONNECTIONS"
	curl "${FORTIO_URL}data/$DATA_URL" --output "$OUT_PATH/fortio-$OUTNAME.json"
	sleep 2
	# undocumented, unsupported API -- move to supported GRPC API if needed
	curl "${JAEGER_URL}traces?service=istio-ingressgateway&tags=%7B%22http.url%22%3A%22http%3A%2F%2Fistio-ingressgateway.istio-system%2Fratings%2Finvalid-$TEST_NUMBER%22%7D" --output "$OUT_PATH/jaeger-$OUTNAME.json"
}

perftest () {
	RESULTS_DIR=${RESULTS_DIR:-$BASEDIR/../e2e/latency/results/$DATE}
	mkdir -p "$RESULTS_DIR/with_cf"
	TESTID=$((RANDOM*10000))
	for CONNECTIONS in 10 70 125 250 500; do
	for QPS in 50 250 500 1000; do
		run_fortio "$CONNECTIONS" "$QPS" 30 "$TESTID" "$RESULTS_DIR/with_cf"
		TESTID=$((TESTID+1))
	done
	done

	mkdir -p "$RESULTS_DIR/without_cf"
	kubectl delete -n istio-system envoyfilter curiefense-access-logs-filter
	kubectl delete -n istio-system envoyfilter curiefense-lua-filter
	for CONNECTIONS in 10 70 125 250 500; do
		for QPS in 50 250 500 1000; do
			run_fortio "$CONNECTIONS" "$QPS" 30 "$TESTID" "$RESULTS_DIR/without_cf"
			TESTID=$((TESTID+1))
		done
	done

	export RESULTS_DIR
	jupyter nbconvert --execute "$BASEDIR/../e2e/latency/Curiefense performance report.ipynb" --to html --template classic
	mv "$BASEDIR/../e2e/latency/Curiefense performance report.html" "$BASEDIR/../e2e/latency/Curiefense performance report-$VERSION-$DATE.html"
}


cleanup () {
	echo "-- Cleanup --"
	gcloud container clusters delete --region="$REGION" --quiet "$CLUSTER_NAME"
	rm "$KUBECONFIG"
}

while [[ "$#" -gt 0 ]]; do
	case $1 in
		-c|--create-cluster) create="y"; shift ;;
		-i|--install-helm) helm="y"; shift ;;
		-d|--deploy-curiefense) curiefense="y"; shift ;;
		-b|--deploy-bookinfo) bookinfo="y"; shift ;;
		-j|--deploy-jaeger) jaeger="y"; shift ;;
		-f|--deploy-fortio) fortio="y"; shift ;;
		-p|--perf-test) perftest="y"; shift ;;
		-C|--cleanup) cleanup="y"; shift ;;
		-t|--test-cycle) all="y"; shift ;;
		*) echo "Unknown parameter passed: $1"; exit 1 ;;
	esac
done

if [ "$create" = "y" ] || [ "$all" = "y" ]; then
	create_cluster
fi
if [ "$helm" = "y" ] || [ "$all" = "y" ]; then
	install_helm
fi
if [ "$curiefense" = "y" ] || [ "$all" = "y" ]; then
	deploy_curiefense
fi
if [ "$bookinfo" = "y" ] || [ "$all" = "y" ]; then
	deploy_bookinfo
fi
if [ "$fortio" = "y" ] || [ "$all" = "y" ]; then
	install_fortio
fi
if [ "$perftest" = "y" ] || [ "$all" = "y" ]; then
	perftest
fi
if [ "$cleanup" = "y" ] || [ "$all" = "y" ]; then
	cleanup
fi
