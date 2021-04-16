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
    PARAMS+=("--set" "global.imagePullPolicy=IfNotPresent")
fi

if [ -n "$JWT_WORKAROUND" ]; then
    PARAMS+=("-f" "charts/first-party-jwt.yaml")
fi
# Install the Istio base chart which contains cluster-wide resources used by the Istio control plane
helm upgrade istio-base charts/base -n istio-system --install \
    --wait --timeout 600s --create-namespace

# Install the Istio discovery chart which deploys the istiod service
helm upgrade istiod charts/istio-control/istio-discovery \
    "${PARAMS[@]}" \
    --wait --timeout 600s \
    -n istio-system --install

# shellcheck disable=SC2086
if ! helm upgrade istio-ingress charts/gateways/istio-ingress \
    --install --namespace istio-system --reuse-values --debug \
    -f charts/enable-waf-ingress.yaml \
    -f charts/first-party-jwt.yaml \
    "${PARAMS[@]}" \
    --wait --timeout 600s \
    --set "global.proxy.gw_image=curiefense/curieproxy-istio:$DOCKER_TAG" \
    --set "global.proxy.curiesync_image=curiefense/curiesync:$DOCKER_TAG" "$@"
then
    echo "istio deployment failure... "
    kubectl --namespace istio-system describe pods
    # TODO(flaper87): Print logs from failed PODs
fi
