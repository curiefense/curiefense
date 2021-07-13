#!/bin/bash

helm repo add curiefense https://helm.curiefense.io/
helm repo update

if [ -z "$DOCKER_TAG" ]; then
    if ! GITTAG="$(git describe --tag --long --exact-match 2> /dev/null)"; then
        GITTAG="$(git describe --tag --long --dirty)"
        echo "This commit is not tagged; use this for testing only"
    fi
    DOCKER_DIR_HASH="$(git rev-parse --short=12 HEAD:curiefense)"
    DOCKER_TAG="$GITTAG-$DOCKER_DIR_HASH"
fi

PARAMS=()

if [ -n "$NOPULL" ]; then
    PARAMS+=("--set" "global.imagePullPolicy=IfNotPresent")
fi

if [ -n "$TESTIMG" ]; then
    PARAMS+=("--set" "global.images.$TESTIMG=curiefense/$TESTIMG:test")
    echo "Deploying version \"test\" for image $TESTIMG, and version $GITTAG for all others"
else
    echo "Deploying version $DOCKER_TAG for all images"
fi

# shellcheck disable=SC2086
if ! helm upgrade --install --namespace curiefense --reuse-values \
    --set "global.settings.docker_tag=$DOCKER_TAG" \
    --wait --timeout 600s --create-namespace \
    "${PARAMS[@]}" "$@" curiefense curiefense/curiefense
then
    echo "curiefense deployment failure... "
    kubectl --namespace curiefense describe pods

    # Template generation
    helm template --debug --set "global.settings.docker_tag=$DOCKER_TAG" "${PARAMS[@]}" "$@" curiefense curiefense/curiefense
    # TODO(flaper87): Print logs from failed PODs
fi
