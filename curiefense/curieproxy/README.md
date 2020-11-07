# Building envoy with symbols for lua

```bash
git clone https://github.com/envoyproxy/envoy.git
cd envoy

./ci/run_envoy_docker.sh "BAZEL_BUILD_EXTRA_OPTIONS='--define exported_symbols=enabled'" './ci/do_ci.sh bazel.release.server_only'

cp  /tmp/envoy-docker-build/envoy/source/exe/envoy .
strip ./envoy
objdump -T envoy | grep lua_checkstack
ls -lh envoy
```


# Building istio envoy 1.12.0 for istio 1.3.6 with symbols for lua


```bash
git clone https://github.com/istio/proxy
cd proxy
git checkout 1.3.6

# modify linker options to add dynamic symbols
sed -ie '/action_env=BAZEL_LINKOPTS=-lm:-static-libgcc/s/$/:-Wl,-E/' .bazelrc

# build using istio build tools image
docker run -ti -v $(realpath .):/work gcr.io/istio-testing/build-tools-proxy:release-1.5-2020-01-27T22-54-35 "cd /work ; make build_envoy"
 ```

