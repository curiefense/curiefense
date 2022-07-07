ARG RUSTBIN_TAG=main
FROM curiefense/curiefense-rustbuild-bionic:${RUSTBIN_TAG} AS rustbin
FROM curiefense/envoy-cf:8218a88a1ae76b7657ae226e5542e6f4058d9921 AS envoy-cf
FROM envoyproxy/envoy:v1.21.1

RUN apt-get update && \
    apt-get -qq -y --no-install-recommends install jq luarocks libpcre2-dev libgeoip-dev \
    python gcc g++ make unzip libhyperscan4 libhyperscan-dev && \
    luarocks install lrexlib-pcre2 && \
    luarocks install lua-cjson && \
    luarocks install lua-resty-string && \
    luarocks install luafilesystem && \
    luarocks install luasocket && \
    luarocks install redis-lua && \
    luarocks install compat53 && \
    luarocks install mmdblua && \
    luarocks install luaipc && \
    luarocks install lua-resty-injection && \
    apt-get purge -y jq luarocks libpcre2-dev libgeoip-dev python \
    gcc g++ make unzip libhyperscan-dev && apt-get autoremove --purge -y && \
    rm -rf /var/lib/apt/lists/*

# Overwrite stripped envoy with full symbol
COPY --from=envoy-cf /envoy /usr/local/bin/envoy

COPY init/start_curiefense.sh /start_curiefense.sh
COPY curieproxy/lua /lua
COPY curieproxy/lua/shared-objects/*.so /usr/local/lib/lua/5.1/
COPY --from=rustbin /root/curiefense.so /usr/local/lib/lua/5.1/
COPY curieproxy/cf-config /bootstrap-config/config
COPY curieproxy/envoy.yaml.* /etc/envoy/

RUN mkdir /cf-config && chmod a+rwxt /cf-config

ENTRYPOINT ["/start_curiefense.sh"]
