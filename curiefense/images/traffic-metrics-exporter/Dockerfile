FROM python:3.8.12-slim-buster
EXPOSE 8911

RUN pip install --upgrade pip

COPY traffic-metrics-exporter/src /traffic-metrics-exporter
COPY traffic-metrics-exporter/setup.py /traffic-metrics-exporter
COPY traffic-metrics-exporter/setup.cfg /traffic-metrics-exporter
RUN cd /traffic-metrics-exporter ; pip3 install --no-cache-dir .

WORKDIR /traffic-metrics-exporter
CMD ["python3", "./traffic_metrics_exporter.py"]
