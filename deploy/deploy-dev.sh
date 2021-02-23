#!/bin/bash
#set -e
# shellcheck disable=SC1090

BASEDIR="$(dirname "$(readlink -f "$0")")" 
DATE="$(date --iso=m)"
BRANCH=${BRANCH:-dev}

# This script is used for running nightly tests
# running e2e tests needs s3 credentials until https://github.com/curiefense/curiefense/issues/48 is fixed
source "$BASEDIR/aws-secrets.txt"
# aws-secrets.txt should contain:
# export CURIE_S3_ACCESS_KEY=XXXXXXXXXXXXXXXXXXXX
# export CURIE_S3_SECRET_KEY=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

mkdir -p "$BASEDIR/test-reports/"

echo "-- Update git repo --"
rm -rf "$BASEDIR/curiefense"
cd "$BASEDIR" || exit 1
git clone https://github.com/curiefense/curiefense -b "$BRANCH"
cd curiefense || exit 1
git pull

echo "-- Build images --"
eval "$(minikube docker-env)"
cd "$BASEDIR/curiefense/curiefense/images/" || exit 1
if ! ./build-docker-images.sh; then
	echo "Image build failed"
	exit 1;
fi

echo "-- Purge installed release --"
if helm ls -a | grep -q curiefense; then
	helm delete --purge curiefense
fi
if helm ls -a | grep -q istio-cf; then
	helm delete --purge istio-cf
fi

for i in persistent-confdb-confserver-0 persistent-grafana-grafana-0 persistent-logdb-logdb-0 persistent-prometheus-prometheus-0 persistent-redis-redis-0 persistent-elasticsearch-elasticsearch-0; do 
	if kubectl get pvc -n curiefense | grep -q "$i"; then 
		kubectl delete pvc -n curiefense --grace-period=0 --force $i
	fi
done

cd "$BASEDIR/curiefense" || exit 1
VERSION=$(git rev-parse --short=12 HEAD)

echo "-- Deploy istio --"
cd "$BASEDIR/curiefense/deploy/istio-helm/" || exit 1
NOPULL=1 ./deploy.sh
sleep 10
# reduce cpu requests for istio components so that this fits on a 4-CPU node
kubectl patch -n istio-system deployment istio-telemetry --patch '{"spec": {"template": {"spec": {"containers": [{"name": "mixer", "resources": {"requests": {"cpu": "10m"}}}]}}}}'
kubectl patch -n istio-system deployment istio-telemetry --patch '{"spec": {"template": {"spec": {"containers": [{"name": "istio-proxy", "resources": {"requests": {"cpu": "10m"}}}]}}}}'
kubectl patch -n istio-system deployment istio-pilot --patch '{"spec": {"template": {"spec": {"containers": [{"name": "discovery", "resources": {"requests": {"cpu": "10m"}}}]}}}}'
sleep 5
kubectl delete -n istio-system pods -l istio-mixer-type=telemetry

echo "-- Deploy curiefense --"
cd "$BASEDIR/curiefense/deploy/curiefense-helm/" || exit 1
NOPULL=1 ./deploy.sh

echo "-- Install curieconfctl --"
rm -rf "$BASEDIR/venv"
python3 -m venv "$BASEDIR/venv"
source "$BASEDIR/venv/bin/activate"
pip install requests pytest pytest-html wheel

cd "$BASEDIR/curiefense/curiefense/curieconf/utils" || exit 1
pip install -e .

cd "$BASEDIR/curiefense/curiefense/curieconf/client" || exit 1
pip install -e .


echo "-- Run e2e tests --"
cd "$BASEDIR/curiefense/e2e/" || exit 1
IP=172.17.0.2
pytest --log-level INFO --base-protected-url http://$IP:30081 --base-conf-url http://$IP:30000/api/v1/ --base-ui-url http://$IP:30080 --html="$BASEDIR/test-reports/test-report-$BRANCH-$DATE-$VERSION.html" --self-contained-html .


echo "-- Look for unknown or abnormal log messages --"
cd "$BASEDIR/curiefense/e2e/logs-smoke-test/" || exit 1
./checklogs-helm.sh
grep . ./*log


