ARG RUSTBIN_TAG=latest
FROM curiefense/envoy-cf:8218a88a1ae76b7657ae226e5542e6f4058d9921 AS envoy-cf
FROM envoyproxy/envoy:v1.21.1

# Overwrite stripped envoy with full symbol
COPY --from=envoy-cf /envoy /usr/local/bin/envoy

COPY init/start_curiefense.sh /start_curiefense.sh
COPY envoy.yaml.* /etc/envoy/

ENTRYPOINT ["/start_curiefense.sh"]
