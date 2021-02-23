#!/bin/bash

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
    PARAMS+=("--set" "global.imagePullPolicy=Never")
fi

if [ -n "$TESTIMG" ]; then
    PARAMS+=("--set" "global.images.$TESTIMG=curiefense/$TESTIMG:test")
    echo "Deploying version \"test\" for image $TESTIMG, and version $GITTAG for all others"
else
    echo "Deploying version $DOCKER_TAG for all images"
fi

helm upgrade --install --namespace curiefense --reuse-values --atomic --debug \
    --set "global.settings.docker_tag=$DOCKER_TAG" \
    "${PARAMS[@]}" "$@" curiefense curiefense/
