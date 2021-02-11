set +x

eval $(minikube docker-env)

export DATE="$(date --iso=m)"
export GITTAG="$(git describe --tag --long --dirty)"
export DOCKER_DIR_HASH="$(git rev-parse --short=12 HEAD:curiefense)"
export DOCKER_TAG="$GITTAG-$DOCKER_DIR_HASH"
export BASEDIR="$(dirname "$(readlink -f "$0")")"

export ROOT_DIR=$(git rev-parse --show-toplevel)
export WORKDIR=$(mktemp -d -t ci-XXXXXXXXXX)
export LOGS_DIR=$WORKDIR/logs

mkdir -p $LOGS_DIR

# Let's run the script from the root directory
pushd $ROOT_DIR

pushd curiefense/images
./build-docker-images.sh
popd

# curieconfctl will try to write to this
# path during the tests. This is currently
# not configurable.
mkdir -p $WORKDIR/bucket
chmod 777 $WORKDIR/bucket

# Make sure the *local* bucket directory is mounted on minikube's
# VM. This will make sure that the `/bucket` hostPath mounted in the
# PODs is also shared locally
nohup minikube mount $WORKDIR/bucket:/bucket > $LOGS_DIR/minikube-mount.log &

# Create a tunnel so we can guarantee that the gateway's LoadBalancer will
# get an IP from the host. We could use a different service type for the
# gateway but let's try to keep it as close to production-like as possible.
nohup minikube tunnel > $LOGS_DIR/minikube-tunnel.log &

pushd deploy/istio-helm
./deploy.sh -f chart/use-local-bucket.yaml -f chart/values-istio-ci.yaml
sleep 10
# reduce cpu requests for istio components so that this fits on a 4-CPU node
kubectl patch -n istio-system deployment istio-pilot --patch '{"spec": {"template": {"spec": {"containers": [{"name": "discovery", "resources": {"requests": {"cpu": "10m"}}}]}}}}'
sleep 5
popd

pushd deploy/curiefense-helm
./deploy.sh -f curiefense/use-local-bucket.yaml -f curiefense/e2e-ci.yaml

# Expose services
# No need to pass the namespace as it's already
# specified in the k8s manifest itself. Two namespaces
# are used in this manifest: istio-system, and curiefense
kubectl create -f expose-services.yaml
popd

echo "-- Deploy echoserver (test app) --"
kubectl -n echoserver create -f deploy/echo-server.yaml

runtime="5 minute"
endtime=$(date -ud "$runtime" +%s)

while [[ ! $(curl -fsS "http://$(minikube ip):30081/productpage" | grep "command=GET" ) ]];
do
    if [[ $(date -u +%s) -ge $endtime ]];
    then
        kubectl --namespace echoserver describe pods
        kubectl --namespace echoserver get pods
        echo "Time out waiting for echoserver to respond"
        exit 1
    fi

    echo "Waiting for echoserver: sleeping for 20s"
    sleep 20
done

