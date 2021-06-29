# build container
FROM nickblah/luajit:2-lua52compat-luarocks-bionic

RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get -yq --no-install-recommends install \
        curl ca-certificates libhyperscan-dev \
        gcc pkg-config libssl-dev python2.7 clang-10 libclang1-10 \
        make unzip git redis-server
RUN ln -s /usr/bin/python2.7 /usr/bin/python2

RUN luarocks install lua-cjson && \
    luarocks install luafilesystem && \
    luarocks install luasocket && \
    luarocks install redis-lua

RUN useradd -m -s /bin/bash builder
USER builder
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/home/builder/.cargo/bin:${PATH}"
RUN mkdir /home/builder/rust && mkdir /home/builder/lua && mkdir /home/builder/.cargo/registry