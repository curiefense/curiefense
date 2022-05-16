ARG RUSTBIN_TAG=latest
FROM curiefense/curiefense-rustbuild-bionic:${RUSTBIN_TAG} as rustbin
FROM ubuntu:bionic

RUN apt-get update && \
    apt-get -qq -y --no-install-recommends install jq libhyperscan4 libhyperscan-dev libssl1.1 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=rustbin /root/cf-externalprocessing /usr/local/bin
COPY static/libgrasshopper.so /usr/lib/

COPY start_ef.sh /start_ef.sh
RUN mkdir /cf-config && chmod a+rwxt /cf-config

ENTRYPOINT ["/start_ef.sh"]