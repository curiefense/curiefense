ARG RUSTBIN_TAG=latest
FROM curiefense/envoy-cf:15d1607b3035c900ea943ac8e1fed6f15598a46b AS envoy-cf
FROM envoyproxy/envoy:v1.21.1

# Overwrite stripped envoy with full symbol
COPY --from=envoy-cf /envoy /usr/local/bin/envoy

COPY init/start_curiefense.sh /start_curiefense.sh
COPY envoy.yaml.* /etc/envoy/

ENTRYPOINT ["/start_curiefense.sh"]
