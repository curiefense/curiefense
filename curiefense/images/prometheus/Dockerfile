FROM prom/prometheus:v2.30.1
COPY prometheus.yml /etc/prometheus/prometheus.yml
ENTRYPOINT [ "/bin/prometheus" ]
CMD        [ "--config.file=/etc/prometheus/prometheus.yml", \
              "--storage.tsdb.path=/prometheus", \
              "--web.console.libraries=/usr/share/prometheus/console_libraries", \
              "--web.console.templates=/usr/share/prometheus/consoles", \
              "--storage.tsdb.retention.time=200h", \
              "--web.enable-lifecycle" ]

