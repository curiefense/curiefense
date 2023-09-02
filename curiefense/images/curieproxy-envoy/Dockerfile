ARG RUSTBIN_TAG=latest
FROM curiefense/curiefense-rustbuild-focal:${RUSTBIN_TAG} AS rustbin
FROM envoyproxy/envoy:v1.25.4

RUN apt-get update && \
    apt-get -qq -y --no-install-recommends install jq luarocks libpcre2-dev libgeoip-dev \
    python gcc g++ make unzip libhyperscan5 libhyperscan-dev curl && \
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

RUN curl -L -O https://artifacts.elastic.co/downloads/beats/filebeat/filebeat-7.13.3-amd64.deb && dpkg -i filebeat-7.13.3-amd64.deb && rm filebeat-7.13.3-amd64.deb

COPY init/start_curiefense.sh /start_curiefense.sh
COPY curieproxy/lua /lua
COPY curieproxy/lua/shared-objects/*.so /usr/local/lib/lua/5.1/
COPY --from=rustbin /root/curiefense.so /usr/local/lib/lua/5.1/
COPY curieproxy/cf-config /bootstrap-config/config
COPY curieproxy/envoy.yaml.* /etc/envoy/

RUN mkdir /cf-config && chmod a+rwxt /cf-config
COPY curieproxy/rust/static/libgrasshopper.so /usr/lib/
COPY filebeat/filebeat-stdin.yml /etc/filebeat.yml
RUN chmod go-w /etc/filebeat.yml

ENTRYPOINT ["/start_curiefense.sh"]
