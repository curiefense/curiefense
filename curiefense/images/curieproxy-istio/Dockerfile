ARG RUSTBIN_TAG=latest
FROM curiefense/curiefense-rustbuild-bionic:${RUSTBIN_TAG} AS rustbin
FROM curiefense/envoy-istio-cf:66815c263c6b178df7cecacf69f9b374435d1bcd AS envoy-istio-cf
FROM docker.io/istio/proxyv2:1.9.3

RUN apt-get update && \
    apt-get -yq --no-install-recommends install jq luarocks libpcre2-dev \
    libgeoip-dev python dumb-init gcc g++ make unzip libhyperscan4 libhyperscan-dev && \
    rm -rf /var/lib/apt/lists/*

RUN luarocks install lrexlib-pcre2 && \
    luarocks install lua-cjson && \
    luarocks install lua-resty-string && \
    luarocks install luafilesystem && \
    luarocks install luasocket && \
    luarocks install redis-lua && \
    luarocks install compat53 && \
    luarocks install mmdblua && \
    luarocks install luaipc && \
    luarocks install lua-resty-injection

# Overwrite stripped envoy with full symbol
COPY --from=envoy-istio-cf /envoy /usr/local/bin/envoy

COPY init/start_curiefense_pilot.sh /start_curiefense_pilot.sh
COPY curieproxy/lua /lua
COPY curieproxy/lua/shared-objects/*.so /usr/local/lib/lua/5.1/
COPY --from=rustbin /root/curiefense.so /usr/local/lib/lua/5.1/
COPY curieproxy/config /bootstrap-config/config

RUN mkdir /config && chmod a+rwxt /config

ENTRYPOINT ["/usr/bin/dumb-init", "/start_curiefense_pilot.sh"]

