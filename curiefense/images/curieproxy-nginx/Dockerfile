ARG RUSTBIN_TAG=latest
FROM curiefense/curiefense-rustbuild-focal:${RUSTBIN_TAG} AS rustbin
FROM docker.io/openresty/openresty:focal
COPY conf.d /etc/nginx/conf.d
COPY curieproxy/lua /lua
COPY curieproxy/lua/shared-objects/*.so /usr/local/lib/lua/5.1/
COPY --from=rustbin /root/curiefense.so /usr/local/lib/lua/5.1/

COPY curieproxy/config /bootstrap-config/config

RUN apt-get update && apt-get install -y libhyperscan5 rsyslog
RUN mkdir /config && chmod a+rwxt /config
RUN openssl req -new -newkey rsa:4096 -days 3650 -nodes -x509 -subj "/C=fr/O=curiefense/CN=testsystem" -keyout /etc/ssl/certificate.key -out /etc/ssl/certificate.crt

RUN mkfifo /nginx-accesslogs

COPY start.sh /usr/bin/
COPY rsyslog.conf /etc/rsyslog.conf
CMD /bin/bash /usr/bin/start.sh
