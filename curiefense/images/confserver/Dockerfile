FROM python:3.8.15-slim-buster

COPY init/requirements.txt /root/requirements.txt

RUN apt-get update && apt-get install --no-install-recommends --no-install-suggests -y \
    gnupg1 ca-certificates libpcre3 git curl libmagic1 dumb-init  \
    build-essential gcc python-dev libpcre3-dev &&  \
    rm -rf /var/lib/apt/lists/* && \
    pip install --no-cache-dir -r /root/requirements.txt && \
    apt-get -y purge build-essential gcc python-dev libpcre3-dev  && apt-get autoremove --purge -y && \
    rm -rf /var/lib/apt/lists/*

COPY bootstrap /bootstrap
COPY init /init
COPY curieconf-utils /curieconf-utils
COPY curieconf-server /curieconf-server

RUN mkdir /uwsgi /etc/uwsgi /app && ln -sf /init/uwsgi.conf /etc/uwsgi/uwsgi.ini && \
    ln -sf /run/secrets/s3cfg/s3cfg /root/.s3cfg && \
    ln -sf /init/entrypoint.sh /entrypoint.sh && \
    ln -sf /curieconf-server/app/main.py /app/main.py &&  \
    cd /curieconf-utils && pip3 install --no-cache-dir . && \
    cd /curieconf-server && pip3 install --no-cache-dir .

WORKDIR /app

ENTRYPOINT ["/usr/bin/dumb-init", "--", "/entrypoint.sh"]

CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "80"]
