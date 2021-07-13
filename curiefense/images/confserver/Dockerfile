FROM tiangolo/uwsgi-nginx-flask:python3.8

RUN ln -sf /bootstrap/initial-bucket-export.conf /etc/supervisor/conf.d/initial-bucket-export.conf
RUN ln -sf /init/nginx.conf /etc/nginx/conf.d/nginx.conf
RUN ln -sf /init/uwsgi.conf /etc/uwsgi/uwsgi.ini
RUN ln -sf /run/secrets/s3cfg/s3cfg /root/.s3cfg
RUN ln -sf /init/prestart.sh /app/prestart.sh
RUN ln -sf /curieconf-server/app/main.py /app/main.py
RUN mkdir /uwsgi
RUN chown nginx /uwsgi

RUN /usr/bin/apt-get update && \
    /usr/bin/apt-get install -yq --no-install-recommends dumb-init && \
	rm -rf /var/lib/apt/lists/*

COPY bootstrap /bootstrap
COPY init /init
COPY curieconf-utils /curieconf-utils
RUN cd /curieconf-utils ; pip3 install --no-cache-dir .
COPY curieconf-server /curieconf-server
RUN cd /curieconf-server ; pip3 install --no-cache-dir .
