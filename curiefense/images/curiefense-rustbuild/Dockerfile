ARG UBUNTU_VERSION=focal
FROM ubuntu:${UBUNTU_VERSION} as builder

RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get -yq --no-install-recommends install \
        curl ca-certificates libhyperscan-dev \
        gcc pkg-config libssl-dev python2.7 clang-10 libclang1-10 \
        gcc libluajit-5.1-dev make unzip git luajit
RUN ln -s /usr/bin/python2.7 /usr/bin/python2 && mkdir /build
COPY curieproxy/rust/static/libgrasshopper.so /usr/lib/
COPY curieproxy/rust /build/rust
WORKDIR /build/rust
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    cargo test --release && \
    cargo build --release && \
    cp target/release/libcuriefense_lua.so /root/curiefense.so && \
    cp target/release/cf-externalprocessing /root/ && \
    rm -rf target /root/.cargo

FROM ubuntu:${UBUNTU_VERSION} as tester
RUN apt-get update && \
    apt-get -yq --no-install-recommends install redis-server luarocks \
    luajit gcc libssl-dev libhyperscan[45] &&  \
    luarocks install lua-cjson && \
    luarocks install luafilesystem && \
    luarocks install luasocket && \
    luarocks install redis-lua && \
    apt-get remove -y jq luarocks gcc &&\
    apt-get autoremove -y &&\
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /root/curiefense.so /usr/local/lib/lua/5.1/curiefense.so
COPY curieproxy/rust/static/libgrasshopper.so /usr/lib/
RUN ln -s /usr/local/lib/lua/5.1/curiefense.so /root/curiefense.so && mkdir -p /cf-config/current
COPY confdb-initial-data/master/config /cf-config/current/config
COPY curieproxy/rust/luatests/config/json /cf-config/current/config/json

RUN useradd -m -s /bin/bash builder
USER builder
COPY curieproxy/lua /home/builder/lua
COPY curieproxy/rust/luatests/redis.lua /home/builder/lua/redis.lua
COPY curieproxy/rust/luatests/test.lua /home/builder/test.lua
COPY curieproxy/rust/luatests /home/builder/luatests
WORKDIR /home/builder
RUN sh luatests/run.sh

FROM scratch
COPY --from=builder /root/curiefense.so /root/curiefense.so
COPY --from=builder /root/cf-externalprocessing /root/cf-externalprocessing