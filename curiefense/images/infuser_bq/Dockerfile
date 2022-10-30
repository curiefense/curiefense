FROM python:3.8.13-buster
# we're using buster because of the bq python lib that uses pyarrow that doens't work with alpine
RUN apt-get update -y && apt-get install -y cron

## infuser_bq cronjob
RUN mkdir -p /scripts
COPY /infuser_bq/src/ /scripts/
WORKDIR /scripts/
RUN pip install --no-cache-dir .
COPY /infuser_bq/infuser_bq.cron /etc/cron.d/infuser_bq.cron
RUN chmod +x /scripts/bin/run.sh && crontab /etc/cron.d/infuser_bq.cron

CMD ["cron", "-f"]
