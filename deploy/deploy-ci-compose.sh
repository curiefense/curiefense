#!/bin/bash

set +x

eval "$(minikube docker-env)"

GITTAG="$(git describe --tag --long --dirty)"
DOCKER_DIR_HASH="$(git rev-parse --short=12 HEAD:curiefense)"
export DOCKER_TAG="$GITTAG-$DOCKER_DIR_HASH"
export HELM_ARGS="--wait --timeout=10m"

ROOT_DIR=$(git rev-parse --show-toplevel)
WORKDIR=$(mktemp -d -t ci-XXXXXXXXXX)
LOGS_DIR="$WORKDIR/logs"

mkdir -p "$LOGS_DIR"

# Let's run the script from the root directory
pushd "$ROOT_DIR" || exit

pushd curiefense/images || exit
./build-docker-images.sh
popd || exit

cat <<EOF > "$WORKDIR/ci-env"
XFF_TRUSTED_HOPS=2
ENVOY_UID=0
DOCKER_TAG=$DOCKER_TAG

CURIE_BUCKET_LINK=file:///bucket/prod/manifest.json
EOF

cat "$WORKDIR/ci-env"

pushd deploy/compose || exit
docker-compose --env-file "$WORKDIR/ci-env" up -d

# Some debug information
docker-compose top
docker-compose logs
