FROM grafana/grafana:7.5.9

COPY --chown=grafana:grafana provisioning/dashboards/* /etc/grafana/provisioning/dashboards/

COPY --chown=grafana:grafana provisioning/datasources/* /etc/grafana/provisioning/datasources/
