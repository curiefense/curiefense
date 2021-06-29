ARG UBUNTU_VERSION=bionic
FROM ubuntu:${UBUNTU_VERSION}

RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get -yq --no-install-recommends install \
        curl ca-certificates libhyperscan-dev \
        gcc pkg-config libssl-dev python2.7 clang-10 libclang1-10 \
        redis-server gcc libluajit-5.1-dev make \
        unzip git redis-server luajit luarocks
RUN ln -s /usr/bin/python2.7 /usr/bin/python2 && mkdir /build
RUN luarocks install lua-cjson && \
    luarocks install luafilesystem && \
    luarocks install luasocket && \
    luarocks install redis-lua
COPY curieproxy/rust /build/rust
WORKDIR /build/rust
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    cargo test && \
    cargo build --release && \
    cp target/release/libcuriefense_lua.so /root/curiefense.so && \
    rm -rf target /root/.cargo

RUN cp /root/curiefense.so /usr/local/lib/lua/5.1/curiefense.so
COPY curieproxy/lua/shared-objects/grasshopper.so /usr/local/lib/lua/5.1/grasshopper.so
RUN mkdir -p /config/current
COPY confdb-initial-data/master/config /config/current/config
COPY curieproxy/rust/luatests/config/json /config/current/config/json

RUN useradd -m -s /bin/bash builder
USER builder
COPY curieproxy/lua /home/builder/lua
COPY curieproxy/rust/luatests/redis.lua /home/builder/lua/redis.lua
COPY curieproxy/rust/luatests/test.lua /home/builder/test.lua
COPY curieproxy/rust/luatests /home/builder/luatests
WORKDIR /home/builder
RUN sh luatests/run.sh
