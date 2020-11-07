FROM tiangolo/uwsgi-nginx-flask:python3.8

RUN ln -sf /init/nginx.conf /etc/nginx/conf.d/nginx.conf
RUN ln -sf /init/uwsgi.conf /etc/uwsgi/uwsgi.ini
RUN ln -sf /init/prestart.sh /app/prestart.sh
RUN ln -sf /curielogserver/app/main.py /app/main.py
RUN mkdir /uwsgi
RUN chown nginx /uwsgi

RUN /usr/bin/apt-get update && \
    /usr/bin/apt-get install -yq --no-install-recommends libpq-dev jq python3-dev gcc python3-psycopg2 dumb-init && \
    rm -rf /var/lib/apt/lists/*

COPY init /init
COPY curielogserver /curielogserver
RUN cd /curielogserver ; pip install .
