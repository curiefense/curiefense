FROM debian:buster

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get -yq --no-install-recommends install \
	dumb-init python3-pip python3-setuptools python3-wheel python3-magic \
	python3-xattr python3-netifaces curl && \
	apt-get autoremove --purge -y && \
	rm -rf /var/lib/apt/lists/*

COPY curieconf-server/curieconf/confserver/json/tag-rules.schema /tag-rules.schema
COPY curieconf-utils /curieconf-utils
RUN cd /curieconf-utils ; pip3 install --no-cache-dir .
COPY curieconf-client /curieconf-client
RUN cd /curieconf-client ; pip3 install --no-cache-dir .
COPY curietasker /curietasker
RUN cd /curietasker ; pip3 install --no-cache-dir .
COPY init /init

ENTRYPOINT ["/usr/bin/dumb-init", "/bin/bash", "/init/run.sh"]

