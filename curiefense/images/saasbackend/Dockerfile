FROM tiangolo/uwsgi-nginx-flask:python3.8-2021-10-26
ENV LISTEN_PORT 8911
EXPOSE 8911

WORKDIR /

RUN ln -sf /init/uwsgi.ini /app/uwsgi.ini

# copy src files
RUN cp -r /main-repo/services/saasbackend/src/* /app
RUN cp -r /main-repo/services/saasbackend/templates /templates
WORKDIR /app
# install package
RUN cp /main-repo/services/saasbackend/setup.py .
RUN cp /main-repo/services/saasbackend/setup.cfg .
RUN pip3 install --upgrade pip
RUN pip3 install --no-cache-dir .
# Install certbot
RUN pip3 install certbot
RUN rm -rf /main-repo
WORKDIR /app
CMD ["gunicorn", "--bind", "0.0.0.0:8911", "main:app", "--preload", "--access-logfile", "-", \
      "--workers", "2", "--threads", "2"]
