ARG RUSTBIN_TAG=latest
FROM curiefense/curiefense-rustbuild-focal:${RUSTBIN_TAG} AS rustbin
FROM docker.io/nginx/nginx-ingress:1.11.3 as nginx-ingress
FROM docker.io/openresty/openresty:focal as openresty

USER root
RUN groupadd --system --gid 101 nginx \
    && useradd --system --gid nginx --no-create-home --home-dir /nonexistent --comment "nginx user" --shell /bin/false --uid 101 nginx

RUN apt-get update \
    && apt-get install -y \
        libcap2-bin \
        libhyperscan5 \
        rsyslog \
	python3-pip \
        python3-setuptools \
        python3-wheel \
        python3-magic \
	python3-xattr \
        python3-netifaces \
    && rm -rf /var/lib/apt/lists/*

# bucket credentials will be provided by docker-compose or k8s secrets, if needed
RUN ln -s /run/secrets/s3cfg/s3cfg /root/.s3cfg

COPY curieconf-utils /curieconf-utils
RUN cd /curieconf-utils ; pip3 install .

COPY curieconf-client /curieconf-client
RUN cd /curieconf-client ; pip3 install .

COPY init /curiesync

COPY --from=nginx-ingress /nginx* /

RUN mkdir -p /var/lib/nginx /etc/nginx/secrets /etc/nginx/stream-conf.d /var/cache/nginx \
    && mkdir -p /var/lib/openresty /var/run/openresty /var/cache/openresty \
    && ln -sf /usr/local/openresty/nginx/sbin/nginx /usr/sbin/nginx \
    && ln -sf /usr/local/openresty/nginx/conf/* /etc/nginx/ \
	&& chown -R nginx:0 /etc/nginx /etc/nginx/secrets /var/cache/nginx /var/lib/nginx /nginx* \
    && chown -R nginx:0 /var/lib/openresty /var/run/openresty /var/cache/openresty /usr/local/openresty \
	&& setcap 'cap_net_bind_service=+ep' /usr/local/openresty/nginx/sbin/nginx \
	&& setcap -v 'cap_net_bind_service=+ep' /usr/local/openresty/nginx/sbin/nginx  \
	&& rm -f /etc/nginx/conf.d/* \
    && mkdir -p /var/lib/syslog \
    && touch /var/log/syslog \
	&& setcap 'cap_net_bind_service=+ep' /usr/sbin/rsyslogd  \
	&& setcap -v 'cap_net_bind_service=+ep' /usr/sbin/rsyslogd  \
    && chown -R nginx:0 /var/lib/syslog /var/log/syslog

RUN sed -i "/^http {/a log_format curiefenselog escape=none '\$request_map';" /nginx.tmpl \
        && sed -i "/^http {/a lua_package_path '/lua/?.lua;;';" /nginx.tmpl

COPY curieproxy/lua/shared-objects/*.so /usr/local/lib/lua/5.1/
COPY --from=rustbin /root/curiefense.so /usr/local/lib/lua/5.1/

COPY curieproxy/config /bootstrap-config/config
COPY curieproxy/lua /lua
RUN rm -f /lua/session.lua && ln -s /lua/session_rust_nginx.lua /lua/session.lua

ENTRYPOINT ["/curiefense-nginx.sh"]

RUN mkdir /config && chmod a+rwxt /config
RUN mkfifo /nginx-accesslogs

COPY curiefense-nginx.sh /curiefense-nginx.sh
COPY rsyslog.conf /etc/rsyslog.conf

