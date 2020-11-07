#! /bin/bash

# Change directory to this script's location
cd "${0%/*}"

# Parameters should be passed as environment variables.
# By default, builds and tags images locally, without pushing
# To push, set `PUSH=1`
# To specify a different repo, set `REPO=my.repo.tld`

REPO=${REPO:-curiefense}
BUILD_OPT=${BUILD_OPT:-}

declare -A status

GITTAG="$(git describe --tag --long --dirty)"
DOCKER_DIR_HASH="$(git rev-parse --short=12 HEAD:curiefense)"
DOCKER_TAG="${DOCKER_TAG:-$GITTAG-$DOCKER_DIR_HASH}"

if [ -n "$TESTIMG" ]; then
    IMAGES=("$TESTIMG")
    OTHER_IMAGES_DOCKER_TAG="$DOCKER_TAG"
    DOCKER_TAG="test"
    echo "Building only image $TESTIMG"
else
    IMAGES=(confserver curielogger curielogserver curieproxy-istio curieproxy-envoy curiesync \
            grafana logdb prometheus redis uiserver)
fi

echo "-------"
echo "Building images: ${IMAGES[@]}"
echo "with tag       : $DOCKER_TAG"
echo "-------"


for image in "${IMAGES[@]}"
do
        IMG=${REPO}/$image
        echo "=================== $IMG:$DOCKER_TAG ====================="
        if tar -C "$image" -czh . | docker build -t "$IMG:$DOCKER_TAG" ${BUILD_OPT} -; then
            STB="ok"
            docker tag "$IMG:$DOCKER_TAG" "$IMG:latest-dev"
            if [ -n "$PUSH" ]; then
                docker push "$IMG:$DOCKER_TAG" && STP="ok" || STP="KO"
                docker push "$IMG:latest-dev"
            else
                STP="SKIP"
            fi
        else
            STB="KO"
            STP="SKIP"
        fi
        status[$image]="build=$STB  push=$STP"
done

for s in "${!status[@]}"
do
        printf "%-25s %s\n" "$s" "${status[$s]}"
done


if [ -n "$TESTIMG" ]; then
    echo "To deploy this test image, export \"TESTIMG=$TESTIMG\" before running deploy.sh or docker-compose up"
    echo "To choose a docker tag for all other images, also export DOCKER_TAG"
    echo "Docker tag of the current working directory is:"
    echo "export DOCKER_TAG=$OTHER_IMAGES_DOCKER_TAG"
else
    echo "To deploy this set of images later, export \"DOCKER_TAG=$DOCKER_TAG\" before running deploy.sh or docker-compose up"
fi

