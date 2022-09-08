#!/bin/bash

# Pre-requisites:
# * images built & pushed to the registry
# * gcloud access is set up
# * the curiefense/curiefense-helm repository is checked out in ../curiefense-helm (at the root of the curiefense repository)
# * This is run on a machine or virtualenv that has pytest & curieconfctl installed

# Some parameters can be overridden, such as:
# * DOCKER_TAG determines which image versions get deployed
# * KUBECONFIG determines where temp credentials are saved. Useful if you want to interact with the cluster with kubectl
# * CLUSTER_NAME defines the name of the GKE cluster -- set this to something unique to avoid interference with other users in your GCP project

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
		kubectl apply -f https://raw.githubusercontent.com/istio/istio/1.13.2/samples/addons/jaeger.yaml
		kubectl apply -f "$BASEDIR/../e2e/latency/jaeger-service.yml"
	fi
	export JWT_WORKAROUND=yes
	pushd "$BASEDIR/../curiefense-helm/istio-helm/" || exit 1
	./deploy.sh --set 'global.tracer.zipkin.address=zipkin.istio-system:9411' --set 'gateways.istio-ingressgateway.autoscaleMax=1' -f "$BASEDIR/curiefense-helm/use-minio-istio.yaml" --set 'global.proxy.curiefense_minio_insecure=true' --set 'gateways.istio-ingressgateway.resources.limits.cpu=4'
	popd || exit 1
	sleep 5
	pushd "$BASEDIR/../curiefense-helm/curiefense-helm/" || exit 1
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
		for deployment in curielogger kibana uiserver; do
			kubectl patch deployment -n curiefense "$deployment" -p \
				'{"spec":{"template":{"spec":{"nodeSelector": {"nodegroup": "curiefense"}}}}}'
		done
		for sts in confserver elasticsearch grafana prometheus redis; do
			kubectl patch statefulsets -n curiefense "$sts" -p \
				'{"spec":{"template":{"spec":{"nodeSelector": {"nodegroup": "curiefense"}}}}}'
		done
	fi
	popd || exit 1
	sleep 30
}

deploy_bookinfo () {
	echo "-- Deploy target: bookinfo app --"
	kubectl label namespace default istio-injection=enabled
	if [ ! -d "$BASEDIR/istio-1.13.2/" ]; then
		pushd "$BASEDIR" || exit 1
		wget 'https://github.com/istio/istio/releases/download/1.13.2/istio-1.13.2-linux-amd64.tar.gz'
		tar -xf 'istio-1.13.2-linux-amd64.tar.gz'
		popd || exit 1
	fi
	kubectl apply -f "$BASEDIR/istio-1.13.2/samples/bookinfo/platform/kube/bookinfo.yaml"
	kubectl apply -f "$BASEDIR/istio-1.13.2/samples/bookinfo/networking/bookinfo-gateway.yaml"
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
	sleep 30
}

install_locust () {
	kubectl create namespace locust
	kubectl create configmap -n locust cf-locustfile "--from-file=main.py=$BASEDIR/locustfile.py"

	helm repo add deliveryhero https://charts.deliveryhero.io/
	helm install locust -n locust deliveryhero/locust --set worker.replicas=6 --set loadtest.locust_locustfile_configmap=cf-locustfile

	for deployment in locust-master locust-worker; do
		kubectl patch deployment -n locust "$deployment" -p \
			'{"spec":{"template":{"spec":{"nodeSelector": {"nodegroup": "perf"}}}}}'
	done

	kubectl apply -f "$BASEDIR/../e2e/latency/locust-service.yml"
	sleep 30
}

locust_perftest () {
	RESULTS_DIR=${RESULTS_DIR:-$BASEDIR/../e2e/latency/locust-results/$DATE}
	export RESULTS_DIR
	NODE_IP=$(kubectl get nodes -o json|jq '.items[0].status.addresses[]|select(.type=="ExternalIP").address'|tr -d '"')
	CONFSERVER_URL="http://$NODE_IP:30000/api/v3/"

	kubectl apply -f ./lua_filter.yaml
	../e2e/set_config.py -u "$CONFSERVER_URL" defaultconfig
	sleep 60
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
	../e2e/set_config.py -u "$CONFSERVER_URL" flow-and-ratelimit
	for REQSIZE in 0 1 2 4 8 16; do
		./locusttest.sh flow-and-ratelimit $REQSIZE
	done

	sleep 60
	kubectl delete -n istio-system envoyfilter curiefense-lua-filter
	sleep 60
	for REQSIZE in 0 1 2 4 8 16; do
		./locusttest.sh istio-only $REQSIZE
	done
	
	echo "Generating test report..."
	jupyter nbconvert --execute "$BASEDIR/../e2e/latency/Curiefense performance report locust.ipynb" --to html --template classic
	mv "$BASEDIR/../e2e/latency/Curiefense performance report locust.html" "$BASEDIR/../e2e/latency/Curiefense performance report-$VERSION-$DATE.html"
}


cleanup () {
	echo "-- Cleanup --"
	gcloud container clusters delete --region="$REGION" --quiet "$CLUSTER_NAME"
	rm "$KUBECONFIG"
}

nbnodes=1
while [[ "$#" -gt 0 ]]; do
	case $1 in
		-c|--create-cluster) create="y"; shift ;;
		-i|--install-helm) helm="y"; shift ;;
		-d|--deploy-curiefense) curiefense="y"; shift ;;
		-b|--deploy-bookinfo) bookinfo="y"; shift ;;
		-j|--deploy-jaeger) jaeger="y"; shift ;;
		-l|--deploy-locust) locust="y"; nbnodes=3; shift ;;
		-p|-L|--locust-perf-test) locustperftest="y"; shift ;;
		-C|--cleanup) cleanup="y"; shift ;;
		-t|--test-cycle) all="y"; shift ;;
		*) echo "Unknown parameter passed: $1"; exit 1 ;;
	esac
done

# Checks
if [ "$locustperftest" = "y" ] || [ "$all" = "y" ]; then
	if ! type curieconfctl; then
		echo "The curieconfctl executable is not available. Please install it, or activate the venv where it is installed."
		echo "Exiting."
		exit 1
	fi
fi

if [ ! -d ../curiefense-helm/.git ]; then
	echo "The curiefense-helm repo should be checked out at (a symlink also works)"
	echo "Exiting."
	exit 1
fi
# Run

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
if [ "$locust" = "y" ] || [ "$all" = "y" ]; then
	install_locust
fi
if [ "$locustperftest" = "y" ] || [ "$all" = "y" ]; then
	locust_perftest
fi
if [ "$cleanup" = "y" ] || [ "$all" = "y" ]; then
	cleanup
fi
