FROM curiefense/envoy-cf:0250ad715e8c0463d0d4389cbfab340bc7dd7587 AS envoy-cf
FROM envoyproxy/envoy:v1.16-latest

RUN apt-get update && \
    apt-get -qq -y --no-install-recommends install jq luarocks libpcre2-dev libgeoip-dev \
    python gcc g++ make unzip libhyperscan4 libhyperscan-dev && \
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
COPY --from=envoy-cf /envoy /usr/local/bin/envoy

COPY init/start_curiefense.sh /start_curiefense.sh

COPY curieproxy/lua /lua
COPY curieproxy/lua/shared-objects/*.so /usr/local/lib/lua/5.1/

COPY curieproxy/config /bootstrap-config/config
COPY curieproxy/envoy.yaml.* /etc/envoy/

RUN mkdir /config && chmod a+rwxt /config

ENTRYPOINT ["/start_curiefense.sh"]

