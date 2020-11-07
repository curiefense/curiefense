FROM docker.io/istio/proxyv2:1.5.10

RUN apt-get update && \
    apt-get -yq --no-install-recommends install jq luarocks libpcre2-dev libgeoip-dev python dumb-init gcc make unzip && \
	rm -rf /var/lib/apt/lists/*

RUN luarocks install lrexlib-pcre2 && \
    luarocks install lua-cjson && \
    luarocks install lua-resty-string && \
    luarocks install luafilesystem && \
    luarocks install luasocket && \
    luarocks install redis-lua && \
    luarocks install compat53 && \
    luarocks install mmdblua && \
    luarocks install luaipc

# Overwrite stripped envoy with full symbol
COPY curieproxy/envoy-1.13.4 /usr/local/bin/envoy

COPY init/start_curiefense_pilot.sh /start_curiefense_pilot.sh
COPY curieproxy/lua /lua
COPY curieproxy/lua/grasshopper.so /usr/local/lib/lua/5.1/grasshopper.so
COPY curieproxy/config /bootstrap-config/config

RUN mkdir /config && chmod a+rwxt /config

ENTRYPOINT ["/usr/bin/dumb-init", "/start_curiefense_pilot.sh"]

