ARG UBUNTU_VERSION=bionic
FROM ubuntu:${UBUNTU_VERSION}

RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get -yq --no-install-recommends install \
        curl ca-certificates libhyperscan-dev \
        pkg-config libssl-dev python2.7 clang-10 libclang1-10 \
        redis-server gcc libluajit-5.1-dev make
RUN ln -s /usr/bin/python2.7 /usr/bin/python2 && mkdir /build
COPY curieproxy/rust /build/rust
WORKDIR /build/rust
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    cargo test && \
    cargo build --release && \
    cp target/release/libcuriefense_lua.so /root/curiefense.so && \
    rm -rf target