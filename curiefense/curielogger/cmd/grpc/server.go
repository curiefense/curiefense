package main

import (
	"errors"
	"fmt"
	"github.com/curiefense/curiefense/curielogger/pkg"
	"github.com/curiefense/curiefense/curielogger/pkg/entities"
	ald "github.com/envoyproxy/go-control-plane/envoy/data/accesslog/v2"
	als "github.com/envoyproxy/go-control-plane/envoy/service/accesslog/v2"
	"github.com/golang/protobuf/ptypes/duration"
	"github.com/golang/protobuf/ptypes/timestamp"
	jsoniter "github.com/json-iterator/go"
	log "github.com/sirupsen/logrus"
	"time"
)

var (
	RequestInfoNotFoundErr = errors.New(`did not find request.info in curiefense medatada`)
)

type grpcServer struct {
	logger *pkg.LogSender
}

func newGrpcSrv(sender *pkg.LogSender) *grpcServer {
	return &grpcServer{logger: sender}
}

func (s grpcServer) StreamAccessLogs(x als.AccessLogService_StreamAccessLogsServer) error {
	for {
		msg, err := x.Recv()
		if err != nil {
			log.Errorf("Error receiving grpc stream message: %v", err)
			return err
		}
		s.sendLogs(msg)
	}
}

func (s *grpcServer) sendLogs(msg *als.StreamAccessLogsMessage) {
	log.Printf("[DEBUG] ====>[%v]", msg.LogEntries)
	for _, entry := range msg.GetHttpLogs().GetLogEntry() {
		outputLog, err := s.parseRawEntry(entry)
		if err != nil {
			log.Error(err)
			continue
		}
		if outputLog == nil {
			continue
		}
		if err = s.logger.Write(outputLog); err != nil {
			log.Error(err)
		}
	}
}

func (s *grpcServer) parseRawEntry(entry *ald.HTTPAccessLogEntry) (*entities.LogEntry, error) {
	common := entry.GetCommonProperties()
	curieFenseData, ok := common.GetMetadata().GetFilterMetadata()["com.reblaze.curiefense"]
	if !ok { /* This log line was not generated by curiefense */
		log.Debug(`No curiefense metadata => drop log entry`)
		return nil, nil
	}

	cfm := curieFenseData.GetFields()
	rInfo, ok := cfm["request.info"]
	if !ok {
		return nil, RequestInfoNotFoundErr
	}
	var curieProxyLog entities.CurieProxyLog
	err := jsoniter.ConfigFastest.Unmarshal([]byte(rInfo.GetStringValue()), &curieProxyLog)
	if err != nil {
		return nil, fmt.Errorf("error unmarshalling metadata json string [%v]: %v", rInfo.GetStringValue(), err)
	}
	log.Debugf("XXXXXXXX curieproxylog=%v", curieProxyLog)

	req := entry.GetRequest()
	resp := entry.GetResponse()
	respFlags := common.GetResponseFlags()
	tls := common.GetTlsProperties()
	lan := make([]string, 0, len(tls.GetLocalCertificateProperties().GetSubjectAltName()))
	for _, san := range tls.GetLocalCertificateProperties().GetSubjectAltName() {
		lan = append(lan, san.String())
	}
	pan := make([]string, 0, len(tls.GetPeerCertificateProperties().GetSubjectAltName()))
	for _, san := range tls.GetPeerCertificateProperties().GetSubjectAltName() {
		pan = append(pan, san.String())
	}

	// Create canonical curiefense log structure

	cflog := s.buildCfLog(req, common, curieProxyLog, respFlags, tls, lan, pan, resp)

	log.Debugf("---> [ %v:%v %v:%v ] <---", cflog.Downstream.RemoteAddress, cflog.Downstream.RemoteAddressPort,
		cflog.Downstream.LocalAddress, cflog.Downstream.LocalAddressPort)

	log.Debugf("cflog=%v", cflog)
	return &entities.LogEntry{
		FullEntry: entry,
		CfLog:     cflog,
	}, nil
}

func (s *grpcServer) buildCfLog(req *ald.HTTPRequestProperties, common *ald.AccessLogCommon, curieProxyLog entities.CurieProxyLog, respFlags *ald.ResponseFlags, tls *ald.TLSProperties, lan []string, pan []string, resp *ald.HTTPResponseProperties) entities.CuriefenseLog {

	return entities.CuriefenseLog{
		RequestId:   req.GetRequestId(),
		Timestamp:   TimestampToRFC3339(common.GetStartTime()),
		Scheme:      req.GetScheme(),
		Authority:   req.GetAuthority(),
		Port:        req.GetPort().GetValue(),
		Method:      req.GetRequestMethod().String(),
		Path:        req.GetPath(),
		Blocked:     curieProxyLog.Blocked,
		BlockReason: curieProxyLog.BlockReason,
		Tags:        append(curieProxyLog.Tags, "curieaccesslog"),

		RXTimers: entities.RXTimer{
			FirstUpstreamByte: DurationToFloat(common.GetTimeToFirstUpstreamRxByte()),
			LastUpstreamByte:  DurationToFloat(common.GetTimeToLastUpstreamRxByte()),
			LastByte:          DurationToFloat(common.GetTimeToLastRxByte()),
		},
		TXTimers: entities.TXTimer{
			FirstUpstreamByte:   DurationToFloat(common.GetTimeToFirstUpstreamTxByte()),
			LastUpstreamByte:    DurationToFloat(common.GetTimeToLastUpstreamTxByte()),
			FirstDownstreamByte: DurationToFloat(common.GetTimeToFirstDownstreamTxByte()),
			LastDownstreamByte:  DurationToFloat(common.GetTimeToLastDownstreamTxByte()),
		},
		Downstream: entities.Downstream{
			ConnectionTermination:   respFlags.GetDownstreamConnectionTermination(),
			DirectRemoteAddress:     common.GetDownstreamDirectRemoteAddress().GetSocketAddress().GetAddress(),
			DirectRemoteAddressPort: common.GetDownstreamDirectRemoteAddress().GetSocketAddress().GetPortValue(),
			LocalAddress:            common.GetDownstreamLocalAddress().GetSocketAddress().GetAddress(),
			LocalAddressPort:        common.GetDownstreamLocalAddress().GetSocketAddress().GetPortValue(),
			ProtocolError:           respFlags.GetDownstreamProtocolError(),
			RemoteAddress:           common.GetDownstreamRemoteAddress().GetSocketAddress().GetAddress(),
			RemoteAddressPort:       common.GetDownstreamRemoteAddress().GetSocketAddress().GetPortValue(),
		},
		Upstream: entities.Upstream{
			Cluster:                common.GetUpstreamCluster(),
			ConnectionFailure:      respFlags.GetUpstreamConnectionFailure(),
			ConnectionTermination:  respFlags.GetUpstreamConnectionTermination(),
			LocalAddress:           common.GetUpstreamLocalAddress().GetSocketAddress().GetAddress(),
			LocalAddressPort:       common.GetUpstreamLocalAddress().GetSocketAddress().GetPortValue(),
			Overflow:               respFlags.GetUpstreamOverflow(),
			RemoteAddress:          common.GetUpstreamRemoteAddress().GetSocketAddress().GetAddress(),
			RemoteAddressPort:      common.GetUpstreamRemoteAddress().GetSocketAddress().GetPortValue(),
			RemoteReset:            respFlags.GetUpstreamRemoteReset(),
			RequestTimeout:         respFlags.GetUpstreamRequestTimeout(),
			RetryLimitExceeded:     respFlags.GetUpstreamRetryLimitExceeded(),
			TransportFailureReason: common.GetUpstreamTransportFailureReason(),
		},
		TLS: entities.TLS{
			LocalCertificate: entities.CertificateData{
				Properties:         tls.GetLocalCertificateProperties().GetSubject(),
				PropertiesAltNames: lan,
			},
			PeerCertificate: entities.CertificateData{
				Properties:         tls.GetPeerCertificateProperties().GetSubject(),
				PropertiesAltNames: pan,
			},
			CipherSuite: tls.GetTlsCipherSuite().String(),
			SessionId:   tls.GetTlsSessionId(),
			SNIHostname: tls.GetTlsSniHostname(),
			Version:     tls.GetTlsVersion().String(),
		},
		Request: entities.Request{
			BodyBytes:    req.GetRequestBodyBytes(),
			HeadersBytes: req.GetRequestHeadersBytes(),
			OriginalPath: req.GetOriginalPath(),
			Headers:      curieProxyLog.Headers,
			Cookies:      curieProxyLog.Cookies,
			Arguments:    curieProxyLog.Arguments,
			Geo:          curieProxyLog.Geo,
			Attributes:   curieProxyLog.Attributes,
		},
		Response: entities.Response{
			BodyBytes:    resp.GetResponseBodyBytes(),
			Code:         int(resp.GetResponseCode().GetValue()),
			CodeDetails:  resp.GetResponseCodeDetails(),
			Headers:      resp.GetResponseHeaders(),
			HeadersBytes: resp.GetResponseHeadersBytes(),
			Trailers:     resp.GetResponseTrailers(),
		},
		Metadata: entities.Metadata{
			DelayInjected:              respFlags.GetDelayInjected(),
			FailedLocalHealthCheck:     respFlags.GetFailedLocalHealthcheck(),
			FaultInjected:              respFlags.GetFaultInjected(),
			InvalidEnvoyRequestHeaders: respFlags.GetInvalidEnvoyRequestHeaders(),
			LocalReset:                 respFlags.GetLocalReset(),
			NoHealthyUpstream:          respFlags.GetNoHealthyUpstream(),
			NoRouteFound:               respFlags.GetNoRouteFound(),
			RateLimited:                respFlags.GetRateLimited(),
			RateLimitServiceError:      respFlags.GetRateLimitServiceError(),
			RouteName:                  common.GetRouteName(),
			SampleRate:                 common.GetSampleRate(),
			StreamIdleTimeout:          respFlags.GetStreamIdleTimeout(),
			UnauthorizedDetails:        respFlags.GetUnauthorizedDetails().GetReason().String(),
		},
	}
}

func DurationToFloat(d *duration.Duration) float64 {
	d.GetNanos()
	if d != nil {
		return float64(d.GetSeconds()) + float64(d.GetNanos())*1e-9
	}
	return 0
}

func TimestampToRFC3339(d *timestamp.Timestamp) string {
	var v time.Time
	if d != nil {
		v = time.Unix(int64(d.GetSeconds()), int64(d.GetNanos()))
	} else {
		v = time.Now()
	}
	return v.Format(time.RFC3339Nano)
}
