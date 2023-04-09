ARG RUSTBIN_TAG=latest
FROM curiefense/curiefense-rustbuild-jammy:${RUSTBIN_TAG} AS rustbin
FROM docker.io/istio/proxyv2:1.17.2

RUN apt-get update && \
    apt-get -yq --no-install-recommends install jq luarocks libpcre2-dev \
    libgeoip-dev python3 dumb-init gcc g++ make unzip libhyperscan[45] libhyperscan-dev && \
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
    apt-get remove -y jq luarocks libpcre2-dev libgeoip-dev python \
    gcc g++ make unzip libhyperscan-dev && apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/*

COPY init/start_curiefense_pilot.sh /start_curiefense_pilot.sh
COPY curieproxy/lua /lua
RUN ln -s /lua/shared-objects/*.so /usr/local/lib/lua/5.1/ && mkdir /cf-config && chmod a+rwxt /cf-config
COPY --from=rustbin /root/curiefense.so /usr/local/lib/lua/5.1/
COPY curieproxy/rust/static/libgrasshopper.so /usr/lib/
COPY curieproxy/cf-config /bootstrap-config/config

ENTRYPOINT ["/usr/bin/dumb-init", "/start_curiefense_pilot.sh"]

