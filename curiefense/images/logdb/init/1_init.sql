CREATE DATABASE curiefense;
\c curiefense
create extension hstore;

/* https://github.com/envoyproxy/go-control-plane/blob/master/envoy/data/accesslog/v2/accesslog.pb.go*/

create table logs (
    -- ROW ID??
    rowid                          bigserial NOT NULL,


--  ProtocolVersion  HTTPAccessLogEntry_HTTPVersion `protobuf:"varint,2,opt,name=protocol_version,json=protocolVersion,proto3,enum=envoy.data.accesslog.v2.HTTPAccessLogEntry_HTTPVersion" json:"protocol_version,omitempty"`
    ProtocolVersion                TEXT,

/*******************/
/* AccessLogCommon */
/*******************/


--  SampleRate                     float64              `protobuf:"fixed64,1,opt,name=sample_rate,json=sampleRate,proto3" json:"sample_rate,omitempty"`
    SampleRate                     FLOAT8 NOT NULL,

--  DownstreamRemoteAddress        *core.Address        `protobuf:"bytes,2,opt,name=downstream_remote_address,json=downstreamRemoteAddress,proto3" json:"downstream_remote_address,omitempty"`
    DownstreamRemoteAddress        TEXT NOT NULL,
    DownstreamRemoteAddressPort    INTEGER NOT NULL,

--  DownstreamLocalAddress         *core.Address        `protobuf:"bytes,3,opt,name=downstream_local_address,json=downstreamLocalAddress,proto3" json:"downstream_local_address,omitempty"`
    DownstreamLocalAddress         TEXT NOT NULL,
    DownstreamLocalAddressPort     INTEGER NOT NULL,

--  StartTime                      *timestamp.Timestamp `protobuf:"bytes,5,opt,name=start_time,json=startTime,proto3" json:"start_time,omitempty"`
    StartTime                      TIMESTAMPTZ NOT NULL,

--  TimeToLastRxByte               *duration.Duration   `protobuf:"bytes,6,opt,name=time_to_last_rx_byte,json=timeToLastRxByte,proto3" json:"time_to_last_rx_byte,omitempty"`
    TimeToLastRxByte               FLOAT,

--  TimeToFirstUpstreamTxByte      *duration.Duration   `protobuf:"bytes,7,opt,name=time_to_first_upstream_tx_byte,json=timeToFirstUpstreamTxByte,proto3" json:"time_to_first_upstream_tx_byte,omitempty"`
    TimeToFirstUpstreamTxByte      FLOAT,

--  TimeToLastUpstreamTxByte       *duration.Duration   `protobuf:"bytes,8,opt,name=time_to_last_upstream_tx_byte,json=timeToLastUpstreamTxByte,proto3" json:"time_to_last_upstream_tx_byte,omitempty"`
    TimeToLastUpstreamTxByte       FLOAT,

--  TimeToFirstUpstreamRxByte      *duration.Duration   `protobuf:"bytes,9,opt,name=time_to_first_upstream_rx_byte,json=timeToFirstUpstreamRxByte,proto3" json:"time_to_first_upstream_rx_byte,omitempty"`
    TimeToFirstUpstreamRxByte      FLOAT,

--  TimeToLastUpstreamRxByte       *duration.Duration   `protobuf:"bytes,10,opt,name=time_to_last_upstream_rx_byte,json=timeToLastUpstreamRxByte,proto3" json:"time_to_last_upstream_rx_byte,omitempty"`
    TimeToLastUpstreamRxByte       FLOAT,

--  TimeToFirstDownstreamTxByte    *duration.Duration   `protobuf:"bytes,11,opt,name=time_to_first_downstream_tx_byte,json=timeToFirstDownstreamTxByte,proto3" json:"time_to_first_downstream_tx_byte,omitempty"`
    TimeToFirstDownstreamTxByte    FLOAT,

--  TimeToLastDownstreamTxByte     *duration.Duration   `protobuf:"bytes,12,opt,name=time_to_last_downstream_tx_byte,json=timeToLastDownstreamTxByte,proto3" json:"time_to_last_downstream_tx_byte,omitempty"`
    TimeToLastDownstreamTxByte     FLOAT,

--  UpstreamRemoteAddress          *core.Address        `protobuf:"bytes,13,opt,name=upstream_remote_address,json=upstreamRemoteAddress,proto3" json:"upstream_remote_address,omitempty"`
    UpstreamRemoteAddress          TEXT NOT NULL,
    UpstreamRemoteAddressPort      INTEGER NOT NULL,

--  UpstreamLocalAddress           *core.Address        `protobuf:"bytes,14,opt,name=upstream_local_address,json=upstreamLocalAddress,proto3" json:"upstream_local_address,omitempty"`
    UpstreamLocalAddress           TEXT NOT NULL,
    UpstreamLocalAddressPort       INTEGER NOT NULL,

--  UpstreamCluster                string               `protobuf:"bytes,15,opt,name=upstream_cluster,json=upstreamCluster,proto3" json:"upstream_cluster,omitempty"`
    UpstreamCluster                TEXT NOT NULL,

-----  ResponseFlags                  *ResponseFlags       `protobuf:"bytes,16,opt,name=response_flags,json=responseFlags,proto3" json:"response_flags,omitempty"`

--  FailedLocalHealthcheck          bool                        `protobuf:"varint,1,opt,name=failed_local_healthcheck,json=failedLocalHealthcheck,proto3" json:"failed_local_healthcheck,omitempty"`
    FailedLocalHealthcheck          BOOLEAN NOT NULL,

--  NoHealthyUpstream               bool                        `protobuf:"varint,2,opt,name=no_healthy_upstream,json=noHealthyUpstream,proto3" json:"no_healthy_upstream,omitempty"`
    NoHealthyUpstream               BOOLEAN NOT NULL,

--  UpstreamRequestTimeout          bool                        `protobuf:"varint,3,opt,name=upstream_request_timeout,json=upstreamRequestTimeout,proto3" json:"upstream_request_timeout,omitempty"`
    UpstreamRequestTimeout          BOOLEAN NOT NULL,

--  LocalReset                      bool                        `protobuf:"varint,4,opt,name=local_reset,json=localReset,proto3" json:"local_reset,omitempty"`
    LocalReset                      BOOLEAN NOT NULL,

--  UpstreamRemoteReset             bool                        `protobuf:"varint,5,opt,name=upstream_remote_reset,json=upstreamRemoteReset,proto3" json:"upstream_remote_reset,omitempty"`
    UpstreamRemoteReset             BOOLEAN NOT NULL,

--  UpstreamConnectionFailure       bool                        `protobuf:"varint,6,opt,name=upstream_connection_failure,json=upstreamConnectionFailure,proto3" json:"upstream_connection_failure,omitempty"`
    UpstreamConnectionFailure       BOOLEAN NOT NULL,

--  UpstreamConnectionTermination   bool                        `protobuf:"varint,7,opt,name=upstream_connection_termination,json=upstreamConnectionTermination,proto3" json:"upstream_connection_termination,omitempty"`
    UpstreamConnectionTermination   BOOLEAN NOT NULL,

--  UpstreamOverflow                bool                        `protobuf:"varint,8,opt,name=upstream_overflow,json=upstreamOverflow,proto3" json:"upstream_overflow,omitempty"`
    UpstreamOverflow                BOOLEAN NOT NULL,

--  NoRouteFound                    bool                        `protobuf:"varint,9,opt,name=no_route_found,json=noRouteFound,proto3" json:"no_route_found,omitempty"`
    NoRouteFound                    BOOLEAN NOT NULL,

--  DelayInjected                   bool                        `protobuf:"varint,10,opt,name=delay_injected,json=delayInjected,proto3" json:"delay_injected,omitempty"`
    DelayInjected                   BOOLEAN NOT NULL,

--  FaultInjected                   bool                        `protobuf:"varint,11,opt,name=fault_injected,json=faultInjected,proto3" json:"fault_injected,omitempty"`
    FaultInjected                   BOOLEAN NOT NULL,

--  RateLimited                     bool                        `protobuf:"varint,12,opt,name=rate_limited,json=rateLimited,proto3" json:"rate_limited,omitempty"`
    RateLimited                     BOOLEAN NOT NULL,

--  UnauthorizedDetails             *ResponseFlags_Unauthorized `protobuf:"bytes,13,opt,name=unauthorized_details,json=unauthorizedDetails,proto3" json:"unauthorized_details,omitempty"`
    UnauthorizedDetails             TEXT NOT NULL,

--  RateLimitServiceError           bool                        `protobuf:"varint,14,opt,name=rate_limit_service_error,json=rateLimitServiceError,proto3" json:"rate_limit_service_error,omitempty"`
    RateLimitServiceError           BOOLEAN NOT NULL,

--  DownstreamConnectionTermination bool                        `protobuf:"varint,15,opt,name=downstream_connection_termination,json=downstreamConnectionTermination,proto3" json:"downstream_connection_termination,omitempty"`
    DownstreamConnectionTermination BOOLEAN NOT NULL,

--  UpstreamRetryLimitExceeded      bool                        `protobuf:"varint,16,opt,name=upstream_retry_limit_exceeded,json=upstreamRetryLimitExceeded,proto3" json:"upstream_retry_limit_exceeded,omitempty"`
    UpstreamRetryLimitExceeded      BOOLEAN NOT NULL,

--  StreamIdleTimeout               bool                        `protobuf:"varint,17,opt,name=stream_idle_timeout,json=streamIdleTimeout,proto3" json:"stream_idle_timeout,omitempty"`
    StreamIdleTimeout               BOOLEAN NOT NULL,

--  InvalidEnvoyRequestHeaders      bool                        `protobuf:"varint,18,opt,name=invalid_envoy_request_headers,json=invalidEnvoyRequestHeaders,proto3" json:"invalid_envoy_request_headers,omitempty"`
    InvalidEnvoyRequestHeaders      BOOLEAN NOT NULL,

--  DownstreamProtocolError         bool                        `protobuf:"varint,19,opt,name=downstream_protocol_error,json=downstreamProtocolError,proto3" json:"downstream_protocol_error,omitempty"`
    DownstreamProtocolError         BOOLEAN NOT NULL,



--  Metadata                       *core.Metadata       `protobuf:"bytes,17,opt,name=metadata,proto3" json:"metadata,omitempty"`
--  Metadata                       JSONB NOT NULL,

    Curiefense                     JSONB NOT NULL,


--  UpstreamTransportFailureReason string               `protobuf:"bytes,18,opt,name=upstream_transport_failure_reason,json=upstreamTransportFailureReason,proto3" json:"upstream_transport_failure_reason,omitempty"`
    UpstreamTransportFailureReason TEXT NOT NULL,

--  RouteName                      string               `protobuf:"bytes,19,opt,name=route_name,json=routeName,proto3" json:"route_name,omitempty"`
    RouteName                      TEXT NOT NULL,

--  DownstreamDirectRemoteAddress  *core.Address        `protobuf:"bytes,20,opt,name=downstream_direct_remote_address,json=downstreamDirectRemoteAddress,proto3" json:"downstream_direct_remote_address,omitempty"`
    DownstreamDirectRemoteAddress  TEXT NOT NULL,
    DownstreamDirectRemoteAddressPort INTEGER NOT NULL,

--  FilterStateObjects             map[string]*any.Any  `protobuf:"bytes,21,rep,name=filter_state_objects,json=filterStateObjects,proto3" json:"filter_state_objects,omitempty" protobuf_key:"bytes,1,opt,name=key,proto3" protobuf_val:"bytes,2,opt,name=value,proto3"`
--    FilterStateObjects             JSONB NOT NULL,

----- type TLSProperties struct {

--  TlsVersion                 TLSProperties_TLSVersion             `protobuf:"varint,1,opt,name=tls_version,json=tlsVersion,proto3,enum=envoy.data.accesslog.v2.TLSProperties_TLSVersion" json:"tls_version,omitempty"`
    TlsVersion                 TEXT NOT NULL,

--  TlsCipherSuite             *wrappers.UInt32Value                `protobuf:"bytes,2,opt,name=tls_cipher_suite,json=tlsCipherSuite,proto3" json:"tls_cipher_suite,omitempty"`
    TlsCipherSuite             TEXT NOT NULL,

--  TlsSniHostname             string                               `protobuf:"bytes,3,opt,name=tls_sni_hostname,json=tlsSniHostname,proto3" json:"tls_sni_hostname,omitempty"`
    TlsSniHostname             TEXT NOT NULL,

--  LocalCertificateProperties *TLSProperties_CertificateProperties `protobuf:"bytes,4,opt,name=local_certificate_properties,json=localCertificateProperties,proto3" json:"local_certificate_properties,omitempty"`
    LocalCertificateProperties            TEXT NOT NULL,
    LocalCertificatePropertiesAltNames    JSONB NOT NULL,

--  PeerCertificateProperties  *TLSProperties_CertificateProperties `protobuf:"bytes,5,opt,name=peer_certificate_properties,json=peerCertificateProperties,proto3" json:"peer_certificate_properties,omitempty"`
    PeerCertificateProperties             TEXT NOT NULL,
    PeerCertificatePropertiesAltNames     JSONB NOT NULL,

--  TlsSessionId               string                               `protobuf:"bytes,6,opt,name=tls_session_id,json=tlsSessionId,proto3" json:"tls_session_id,omitempty"`
    TlsSessionId               TEXT NOT NULL,

/*************************/
/* HTTPRequestProperties */
/*************************/

----- type HTTPRequestProperties struct {

-- RequestMethod       core.RequestMethod    `protobuf:"varint,1,opt,name=request_method,json=requestMethod,proto3,enum=envoy.api.v2.core.RequestMethod" json:"request_method,omitempty"`
   RequestMethod       TEXT NOT NULL,

-- Scheme              string                `protobuf:"bytes,2,opt,name=scheme,proto3" json:"scheme,omitempty"`
   Scheme              TEXT NOT NULL,

-- Authority           string                `protobuf:"bytes,3,opt,name=authority,proto3" json:"authority,omitempty"`
   Authority           TEXT NOT NULL,

-- Port                *wrappers.UInt32Value `protobuf:"bytes,4,opt,name=port,proto3" json:"port,omitempty"`
   Port                INTEGER NOT NULL,

-- Path                string                `protobuf:"bytes,5,opt,name=path,proto3" json:"path,omitempty"`
   Path                TEXT NOT NULL,

-- UserAgent           string                `protobuf:"bytes,6,opt,name=user_agent,json=userAgent,proto3" json:"user_agent,omitempty"`
   UserAgent           TEXT NOT NULL,

-- Referer             string                `protobuf:"bytes,7,opt,name=referer,proto3" json:"referer,omitempty"`
   Referer             TEXT NOT NULL,

-- ForwardedFor        string                `protobuf:"bytes,8,opt,name=forwarded_for,json=forwardedFor,proto3" json:"forwarded_for,omitempty"`
   ForwardedFor        TEXT NOT NULL,

-- RequestId           string                `protobuf:"bytes,9,opt,name=request_id,json=requestId,proto3" json:"request_id,omitempty"`
   RequestId           TEXT NOT NULL,

-- OriginalPath        string                `protobuf:"bytes,10,opt,name=original_path,json=originalPath,proto3" json:"original_path,omitempty"`
   OriginalPath        TEXT NOT NULL,

-- RequestHeadersBytes uint64                `protobuf:"varint,11,opt,name=request_headers_bytes,json=requestHeadersBytes,proto3" json:"request_headers_bytes,omitempty"`
   RequestHeadersBytes BIGINT NOT NULL,

-- RequestBodyBytes    uint64                `protobuf:"varint,12,opt,name=request_body_bytes,json=requestBodyBytes,proto3" json:"request_body_bytes,omitempty"`
   RequestBodyBytes    BIGINT NOT NULL,

-- RequestHeaders      map[string]string     `protobuf:"bytes,13,rep,name=request_headers,json=requestHeaders,proto3" json:"request_headers,omitempty" protobuf_key:"bytes,1,opt,name=key,proto3" protobuf_val:"bytes,2,opt,name=value,proto3"`
   RequestHeaders      JSONB NOT NULL,

/**************************/
/* HTTPResponseProperties */
/**************************/


----- type HTTPResponseProperties struct {

--  ResponseCode         *wrappers.UInt32Value `protobuf:"bytes,1,opt,name=response_code,json=responseCode,proto3" json:"response_code,omitempty"`
    ResponseCode         INTEGER NOT NULL,

--  ResponseHeadersBytes uint64                `protobuf:"varint,2,opt,name=response_headers_bytes,json=responseHeadersBytes,proto3" json:"response_headers_bytes,omitempty"`
    ResponseHeadersBytes BIGINT NOT NULL,

--  ResponseBodyBytes    uint64                `protobuf:"varint,3,opt,name=response_body_bytes,json=responseBodyBytes,proto3" json:"response_body_bytes,omitempty"`
    ResponseBodyBytes    BIGINT NOT NULL,

--  ResponseHeaders      map[string]string     `protobuf:"bytes,4,rep,name=response_headers,json=responseHeaders,proto3" json:"response_headers,omitempty" protobuf_key:"bytes,1,opt,name=key,proto3" protobuf_val:"bytes,2,opt,name=value,proto3"`
    ResponseHeaders      JSONB NOT NULL,

--  ResponseTrailers     map[string]string     `protobuf:"bytes,5,rep,name=response_trailers,json=responseTrailers,proto3" json:"response_trailers,omitempty" protobuf_key:"bytes,1,opt,name=key,proto3" protobuf_val:"bytes,2,opt,name=value,proto3"`
    ResponseTrailers     JSONB NOT NULL,

--  ResponseCodeDetails  string                `protobuf:"bytes,6,opt,name=response_code_details,json=responseCodeDetails,proto3" json:"response_code_details,omitempty"`
    ResponseCodeDetails  TEXT NOT NULL

);

GRANT CONNECT ON DATABASE curiefense TO logserver_ro;
GRANT USAGE ON SCHEMA public TO logserver_ro;
GRANT SELECT ON TABLE "public"."logs" to logserver_ro;
