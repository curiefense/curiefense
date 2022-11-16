FROM node:16.17.0 as builder

# grab latest commit 
WORKDIR /
RUN git clone --depth=1 https://github.com/reblaze/ui.git

WORKDIR /ui
RUN npm install
RUN npm run build


FROM nginx:1.23.2

USER root
RUN ln -sf /init/nginx.conf /etc/nginx/nginx.conf
RUN ln -sf /dev/stdout /var/log/nginx/access.log && \
    ln -sf /dev/stderr /var/log/nginx/error.log && \
        ln -sf /init/http.conf /etc/nginx/conf.d/ && \
        ln -sf /init/tls-dockercompose.conf /etc/nginx/conf.d/ && \
        ln -sf /init/tls-k8s.conf /etc/nginx/conf.d/
RUN /usr/bin/apt-get update && \
    /usr/bin/apt-get install -yq --no-install-recommends dumb-init && \
        rm -rf /var/lib/apt/lists/*
COPY --from=builder /ui/dist /app
COPY init /init
EXPOSE 80
EXPOSE 443

ENTRYPOINT ["/usr/bin/dumb-init", "/bin/bash", "/docker-entrypoint.sh", "/init/start_nginx.sh"]
