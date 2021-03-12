FROM golang as builder

COPY curielogger /curielogger
WORKDIR /curielogger
RUN go get -u github.com/skx/implant
RUN implant -input files/ -output curielogger/static.go
RUN go build -o build/curielogger ./curielogger/

FROM debian:buster
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get -yq --no-install-recommends install dumb-init curl && \
	apt-get autoremove --purge -y && \
	rm -rf /var/lib/apt/lists/*

COPY --from=builder /curielogger/build/curielogger /bin
COPY init /init/
COPY init/curielogger.yml /etc/curielogger/

ENTRYPOINT ["/usr/bin/dumb-init", "/bin/bash", "/init/start_curielogger.sh"]
