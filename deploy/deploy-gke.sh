#!/bin/bash

# Pre-requisites:
# * images built & pushed to the registry
# * gcloud access is set up
# * the curiefense/curiefense-helm repository is checked out in $HOME/curiefense-helm

BASEDIR="$(dirname "$(readlink -f "$0")")" 
if [ -z "$KUBECONFIG" ]; then
	KUBECONFIG="$(readlink -f "$(mktemp kubeconfig.XXXXX)")"
	export KUBECONFIG
	echo "KUBECONFIG is set to $KUBECONFIG"
fi
CLUSTER_NAME="${CLUSTER_NAME:-curiefense-perftest-gks}"
DATE="$(date --iso=m)"
VERSION="${DOCKER_TAG:-$(git rev-parse --short=12 HEAD)}"
REGION=${REGION:-us-central1-a}

create_cluster () {
	echo "-- Create cluster $CLUSTER_NAME --"
	# 4 CPUs, 16GB
	gcloud container clusters create "$CLUSTER_NAME" --num-nodes="$nbnodes" --machine-type=n2-standard-8 --region="$REGION" --cluster-version=1.20
	gcloud container clusters get-credentials --region="$REGION" "$CLUSTER_NAME"

	if [ "$nbnodes" -gt 1 ]; then
		# Label nodes
		readarray -t NODES < <(kubectl get nodes -o name|sed 's!node/!!')
		GROUP_NAMES=(curiefense ingress perf)
		for i in 0 1 2; do
			kubectl label node "${NODES[$i]}" nodegroup="${GROUP_NAMES[$i]}"
		done
	fi
}

install_helm () {
	echo "-- Install helm --"
	curl -fsSL -o get_helm.sh https://raw.githubusercontent.com/helm/helm/master/scripts/get-helm-3
	chmod 700 get_helm.sh
	./get_helm.sh
}

deploy_curiefense () {
	echo "-- Deploy curiefense --"
	kubectl create namespace curiefense
	kubectl create namespace istio-system
	kubectl apply -f "$BASEDIR/curiefense-helm/example-miniocfg.yaml"
	kubectl apply -f "$BASEDIR/curiefense-helm/example-uiserver-tls.yaml"
	if [ "$jaeger" = "y" ] || [ "$all" = "y" ]; then
		kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.8/samples/addons/jaeger.yaml
		kubectl apply -f "$BASEDIR/../e2e/latency/jaeger-service.yml"
	fi
	export JWT_WORKAROUND=yes
	cd "$HOME/curiefense-helm/istio-helm/" || exit 1
	./deploy.sh --set 'global.tracer.zipkin.address=zipkin.istio-system:9411' --set 'gateways.istio-ingressgateway.autoscaleMax=1' -f "$BASEDIR/curiefense-helm/use-minio-istio.yaml" --set 'global.proxy.curiefense_minio_insecure=true' --set 'gateways.istio-ingressgateway.resources.limits.cpu=4'
	sleep 5
	cd "$HOME/curiefense-helm/curiefense-helm/" || exit 1
	./deploy.sh -f "$BASEDIR/curiefense-helm/use-minio-curiefense.yaml" --set 'global.settings.curiefense_minio_insecure=true'
	kubectl apply -f "$BASEDIR/curiefense-helm/expose-services.yaml"
	if [ "$nbnodes" -gt 1 ]; then
		# assign ingressgateway to the "ingress" node
		kubectl patch deployment -n istio-system istio-ingressgateway -p \
			'{"spec":{"template":{"spec":{"nodeSelector": {"nodegroup": "ingress"}}}}}'
		# assign other components to the "curiefense" node
		for deployment in istiod jaeger; do
			kubectl patch deployment -n istio-system "$deployment" -p \
				'{"spec":{"template":{"spec":{"nodeSelector": {"nodegroup": "curiefense"}}}}}'
		done
		for deployment in curielogger curietasker kibana uiserver; do
			kubectl patch deployment -n curiefense "$deployment" -p \
				'{"spec":{"template":{"spec":{"nodeSelector": {"nodegroup": "curiefense"}}}}}'
		done
		for sts in confserver elasticsearch grafana prometheus redis; do
			kubectl patch statefulsets -n curiefense "$sts" -p \
				'{"spec":{"template":{"spec":{"nodeSelector": {"nodegroup": "curiefense"}}}}}'
		done
	fi
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
	if [ "$nbnodes" -gt 1 ]; then
		for deployment in details-v1 productpage-v1 ratings-v1 reviews-v1 reviews-v2 reviews-v3; do
			kubectl patch deployment -n default "$deployment" -p \
				'{"spec":{"template":{"spec":{"nodeSelector": {"nodegroup": "curiefense"}}}}}'
		done
	fi
}

install_fortio () {
	kubectl apply -f "$BASEDIR/../e2e/latency/fortio-deployment.yml"
	kubectl apply -f "$BASEDIR/../e2e/latency/fortio-service.yml"
}

install_locust () {
	kubectl create configmap -n locust cf-locustfile "--from-file=main.py=$BASEDIR/locustfile.py"

	helm repo add deliveryhero https://charts.deliveryhero.io/
	helm install locust -n locust --create-namespace deliveryhero/locust --set worker.replicas=6 --set loadtest.locust_locustfile_configmap=cf-locustfile

	for deployment in locust-master locust-worker; do
		kubectl patch deployment -n locust "$deployment" -p \
			'{"spec":{"template":{"spec":{"nodeSelector": {"nodegroup": "perf"}}}}}'
	done

	kubectl apply -f "$BASEDIR/../e2e/latency/locust-service.yml"
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

locust_perftest () {
	NODE_IP=$(kubectl get nodes -o json|jq '.items[0].status.addresses[]|select(.type=="ExternalIP").address'|tr -d '"')
	CONFSERVER_URL="http://$NODE_IP:30000/api/v2/"

	kubectl apply -f ~/reblaze/lua_filter.yaml
	../e2e/set_config.py -u "$CONFSERVER_URL" defaultconfig
	sleep 10
	for REQSIZE in 0 1 2 4 8 16; do
		./locusttest.sh cf-default-config $REQSIZE
	done

	sleep 60
	../e2e/set_config.py -u "$CONFSERVER_URL" denyall
	for REQSIZE in 0 1 2 4 8 16; do
		./locusttest.sh cf-denyall-acl $REQSIZE
	done

	sleep 60
	../e2e/set_config.py -u "$CONFSERVER_URL" contentfilter-and-acl
	for REQSIZE in 0 1 2 4 8 16; do
		./locusttest.sh cf-contenfilter-and-acl $REQSIZE
	done

	sleep 60
	kubectl delete -n istio-system envoyfilter curiefense-lua-filter
	sleep 60
	for REQSIZE in 0 1 2 4 8 16; do
		./locusttest.sh istio-only $REQSIZE
	done
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
		-l|--deploy-locust) locust="y"; nbnodes=3; shift ;;
		-p|--perf-test) perftest="y"; shift ;;
		-L|--locust-perf-test) locustperftest="y"; shift ;;
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
if [ "$locust" = "y" ]; then
	install_locust
fi
if [ "$locustperftest" = "y" ]; then
	locust_perftest
fi
if [ "$perftest" = "y" ] || [ "$all" = "y" ]; then
	perftest
fi
if [ "$cleanup" = "y" ] || [ "$all" = "y" ]; then
	cleanup
fi
