package main

import (
	"context"
	"encoding/json"
	"fmt"

	"google.golang.org/grpc"
	"log"
	"net"
	"os"
	"regexp"
	"strconv"
	"strings"
	"time"
	"flag"
	"bufio"

	als "github.com/envoyproxy/go-control-plane/envoy/service/accesslog/v2"
	ptypes "github.com/golang/protobuf/ptypes"
	duration "github.com/golang/protobuf/ptypes/duration"

	"github.com/jackc/pgx/pgtype"
	"github.com/jackc/pgx/v4"
//	"github.com/robteix/protoconv"

//	"google.golang.org/protobuf/types/known/structpb"

	"net/http"

	"github.com/hashicorp/logutils"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

const (
	namespace = "curiemetric" // For Prometheus metrics.
)

/** Prometheus metrics **/

var (
	m_requests = promauto.NewCounter(
		prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "http_request_total",
			Help:      "Total number of HTTP requests",
		},
	)

	metric_session_details = promauto.NewCounterVec(prometheus.CounterOpts{
		Namespace: namespace,
		Name:      "session_details_total",
		Help:      "number of requests per label",
	}, []string{
		"status_code",
		"status_class",
		"origin",
		"origin_status_code",
		"origin_status_class",
		"method",
		"path",
		"blocked",
		"asn",
		"geo",
		"aclid",
		"aclname",
		"wafid",
		"wafname",
		"urlmap",
		"urlmap_entry",
		"container",
	})

	metric_request_bytes = promauto.NewCounter(prometheus.CounterOpts{
		Namespace: namespace,
		Name:      "request_bytes",
		Help:      "The total number of request bytes",
	})
	metric_response_bytes = promauto.NewCounter(prometheus.CounterOpts{
		Namespace: namespace,
		Name:      "response_bytes",
		Help:      "The total number of response bytes",
	})

	m_requests_tags = promauto.NewCounterVec(prometheus.CounterOpts{
		Namespace: namespace,
		Name:      "session_tags_total",
		Help:      "Number of requests per label",
	}, []string{"tag"})
)

/***************/

type server struct {
	db_url string
	host string
	db     *pgx.Conn
}

/**** \\\ auto labeling /// ****/

func isStaticTag(tag string) bool {
	if tag == "all" {
		return true
	}
	parts := strings.Split(tag, ":")
	if len(parts) > 1 {
		prefix := parts[0]
		var static_tags = map[string]bool{
			"ip":           true,
			"asn":          true,
			"geo":          true,
			"aclid":        true,
			"aclname":      true,
			"wafid":        true,
			"wafname":      true,
			"urlmap":       true,
			"urlmap-entry": true,
			"container":    true,
		}
		return static_tags[prefix]
	}
	return false
}

func extractTagByPrefix(prefix string, tags map[string]interface{}) string {

	for name := range tags {
		tagsplit := strings.Split(name, ":")
		if len(tagsplit) == 2 {
			tag_prefix, value := tagsplit[0], tagsplit[1]
			if tag_prefix == prefix {
				return value
			}
		}

	}

	return "N/A"
}

func makeLabels(status_code int, method, path, upstream, blocked string, tags map[string]interface{}) prometheus.Labels {

	// classes and specific response code
	// icode := int(status_code)
	class_label := "status_Nxx"

	switch {
	case status_code < 200:
		class_label = "status_1xx"
	case status_code > 199 && status_code < 300:
		class_label = "status_2xx"
	case status_code > 299 && status_code < 400:
		class_label = "status_3xx"
	case status_code > 399 && status_code < 500:
		class_label = "status_4xx"
	case status_code > 499 && status_code < 600:
		class_label = "status_5xx"
	}

	status_code_str := strconv.Itoa(status_code)

	origin := "N/A"
	origin_status_code := "N/A"
	origin_status_class := "N/A"

	if len(upstream) > 0 {
		origin = upstream
		origin_status_code = fmt.Sprintf("origin_%s", status_code_str)
		origin_status_class = fmt.Sprintf("origin_%s", class_label)
	}

	return prometheus.Labels{
		"status_code":         status_code_str,
		"status_class":        class_label,
		"origin":              origin,
		"origin_status_code":  origin_status_code,
		"origin_status_class": origin_status_class,
		"method":              method,
		"path":                path,
		"blocked":             blocked,
		"asn":                 extractTagByPrefix("asn", tags),
		"geo":                 extractTagByPrefix("geo", tags),
		"aclid":               extractTagByPrefix("aclid", tags),
		"aclname":             extractTagByPrefix("aclname", tags),
		"wafid":               extractTagByPrefix("wafid", tags),
		"wafname":             extractTagByPrefix("wafname", tags),
		"urlmap":              extractTagByPrefix("urlmap", tags),
		"urlmap_entry":        extractTagByPrefix("urlmap-entry", tags),
		"container":           extractTagByPrefix("container", tags),
	}
}

/**** /// auto labeling \\\ ****/

func DurationToFloat(d *duration.Duration) float64 {
	if d != nil {
		return float64(d.GetSeconds()) + float64(d.GetNanos())*1e-9
	}
	return 0
}

func (s server) GetDB() *pgx.Conn {
	for s.db == nil || s.db.IsClosed() {
		conn, err := pgx.Connect(context.Background(), s.db_url)
		if err == nil {
			s.db = conn
			log.Printf("[DEBUG] Connected to database %v\n", s.host)
			break
		}
		log.Printf("[ERROR] Could not connect to database %v: %v\n", s.host, err)
		time.Sleep(time.Second)
	}
	return s.db
}

func makejsonb(v interface{}) *pgtype.JSON {
	j, err := json.Marshal(v)
	if err != nil {
		j = []byte("{}")
	}
	return &pgtype.JSON{Bytes: j, Status: pgtype.Present}
}

func (s server) StreamAccessLogs(x als.AccessLogService_StreamAccessLogsServer) error {
	msg, err := x.Recv()
	if err != nil {
		log.Printf("[ERROR] Error receiving grpc stream message: %v", err)
	} else {
		log.Printf("[DEBUG] ====>[%v]", msg.LogEntries)
		hl := msg.GetHttpLogs()
		http_entries := hl.GetLogEntry()
		for _, entry := range http_entries {
			common := entry.GetCommonProperties()
			curiefense_meta, got_meta := common.GetMetadata().GetFilterMetadata()["com.reblaze.curiefense"]

			upstream_remote_addr := common.GetUpstreamRemoteAddress().GetSocketAddress().GetAddress()
			upstream_remote_port := common.GetUpstreamRemoteAddress().GetSocketAddress().GetPortValue()
			upstream_local_addr := common.GetUpstreamLocalAddress().GetSocketAddress().GetAddress()
			upstream_local_port := common.GetUpstreamLocalAddress().GetSocketAddress().GetPortValue()
			log.Printf("[DEBUG] ---> [ %v:%v %v:%v ] <---", upstream_remote_addr, upstream_remote_port,
				upstream_local_addr, upstream_local_port)
			if !got_meta { /* This log line was not generated by curiefense */
				log.Printf("[DEBUG] No curiefense metadata => drop log entry")
				continue
			}

			respflags := common.GetResponseFlags()
			ts, _ := ptypes.Timestamp(common.GetStartTime())

			tls := common.GetTlsProperties()

			lan := []string{}
			for _, san := range tls.GetLocalCertificateProperties().GetSubjectAltName() {
				lan = append(lan, san.String())
			}
			jsonb_localaltnames := makejsonb(lan)

			pan := []string{}
			for _, san := range tls.GetPeerCertificateProperties().GetSubjectAltName() {
				pan = append(pan, san.String())
			}
			jsonb_peeraltnames := makejsonb(pan)

			req := entry.GetRequest()
			jsonb_reqhdr := makejsonb(req.GetRequestHeaders())

			resp := entry.GetResponse()
			jsonb_resphdr := makejsonb(resp.GetResponseHeaders())
			jsonb_resptrail := makejsonb(resp.GetResponseTrailers())

			method := req.GetRequestMethod().String()
			response_code := resp.GetResponseCode().GetValue()

			json_cf := []byte{}
			json_cf = []byte("{}")
			cf_reqinfo := make(map[string]interface{})

			if got_meta {
				cfm := curiefense_meta.GetFields()
				if rqinfo_s, ok := cfm["request.info"]; ok {
					rqinfo_string := rqinfo_s.GetStringValue()
					json_cf = []byte(rqinfo_string)
					err := json.Unmarshal(json_cf, &cf_reqinfo)
					if err != nil {
						log.Printf("[ERROR] Error unmarshalling metadata json string [%v]", rqinfo_string)
						curiefense_meta = nil
					}
				}

			}

			jsonb_curiefense := &pgtype.JSON{Bytes: json_cf, Status: pgtype.Present}

			// **** Update prometheus metrics ****
			m_requests.Inc()
			metric_request_bytes.Add(float64(req.GetRequestHeadersBytes() + req.GetRequestBodyBytes()))
			metric_response_bytes.Add(float64(resp.GetResponseHeadersBytes() + resp.GetResponseBodyBytes()))


			if got_meta {
				if attrs_i, ok := cf_reqinfo["attrs"] ; ok {
					if attrs, ok := attrs_i.(map[string]interface{}); ok {
						blocked := "0"
						if _, ok := attrs["blocked"]; ok {
							blocked = "1"
						}
						log.Printf("[DEBUG] ~~~ blocked=[%v]", blocked)
						if tags_i, ok := attrs["tags"]; ok {
							if tags, ok := tags_i.(map[string]interface{}); ok {
								if path_i, ok := attrs["path"]; ok {
									if path, ok := path_i.(string); ok {
										log.Printf("[DEBUG] ~~~ path=[%v]", path)
										log.Printf("[DEBUG] ~~~ tags=[%v]", tags)
										labels := makeLabels(int(response_code), method, path, upstream_remote_addr, blocked, tags)
										metric_session_details.With(labels).Inc()
										for name := range tags {
											if !isStaticTag(name) {
												m_requests_tags.WithLabelValues(name).Inc()
											}
										}
									} else { log.Printf("[DEBUG]  @ no path cast :(") }
								} else { log.Printf("[DEBUG]  @ no path :(") }
							} else { log.Printf("[DEBUG]  @ no tags cast :( [%v]", tags_i) }
						} else { log.Printf("[DEBUG]  @ no tags :(") }
					} else { log.Printf("[DEBUG]  @ no attr cast :(") }
				} else { log.Printf("[DEBUG]  @ no attrs :(") }
			} else { log.Printf("[DEBUG]  @ no meta :(") }

			// **** INSERT ****
			if _, err := s.GetDB().Exec(context.Background(), `insert into logs
(
SampleRate,
DownstreamRemoteAddress,
DownstreamRemoteAddressPort,
DownstreamLocalAddress,
DownstreamLocalAddressPort,
StartTime,
TimeToLastRxByte,
TimeToFirstUpstreamTxByte,
TimeToLastUpstreamTxByte,
TimeToFirstUpstreamRxByte,
TimeToLastUpstreamRxByte,
TimeToFirstDownstreamTxByte,
TimeToLastDownstreamTxByte,
UpstreamRemoteAddress,
UpstreamRemoteAddressPort,
UpstreamLocalAddress,
UpstreamLocalAddressPort,
UpstreamCluster,
FailedLocalHealthcheck,
NoHealthyUpstream,
UpstreamRequestTimeout,
LocalReset,
UpstreamRemoteReset,
UpstreamConnectionFailure,
UpstreamConnectionTermination,
UpstreamOverflow,
NoRouteFound,
DelayInjected,
FaultInjected,
RateLimited,
UnauthorizedDetails,
RateLimitServiceError,
DownstreamConnectionTermination,
UpstreamRetryLimitExceeded,
StreamIdleTimeout,
InvalidEnvoyRequestHeaders,
DownstreamProtocolError,
Curiefense,
UpstreamTransportFailureReason,
RouteName,
DownstreamDirectRemoteAddress,
DownstreamDirectRemoteAddressPort,
TlsVersion,
TlsCipherSuite,
TlsSniHostname,
LocalCertificateProperties,
LocalCertificatePropertiesAltNames,
PeerCertificateProperties,
PeerCertificatePropertiesAltNames,
TlsSessionId,
RequestMethod,
Scheme,
Authority,
Port,
Path,
UserAgent,
Referer,
ForwardedFor,
RequestId,
OriginalPath,
RequestHeadersBytes,
RequestBodyBytes,
RequestHeaders,
ResponseCode,
ResponseHeadersBytes,
ResponseBodyBytes,
ResponseHeaders,
ResponseTrailers,
ResponseCodeDetails
) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
          $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38,
          $39, $40, $41, $42, $43, $44, $45, $46, $47, $48, $49, $50, $51, $52, $53, $54, $55, $56,
          $57, $58, $59, $60, $61, $62, $63, $64, $65, $66, $67, $68, $69)`,
				/* SampleRate,                          */
				common.GetSampleRate(),
				/* DownstreamRemoteAddress,             */
				common.GetDownstreamRemoteAddress().GetSocketAddress().GetAddress(),
				/* DownstreamRemoteAddressPort,         */
				common.GetDownstreamRemoteAddress().GetSocketAddress().GetPortValue(),
				/* DownstreamLocalAddress,              */
				common.GetDownstreamLocalAddress().GetSocketAddress().GetAddress(),
				/* DownstreamLocalAddressPort,          */
				common.GetDownstreamLocalAddress().GetSocketAddress().GetPortValue(),
				/* StartTime,                           */
				ts,
				/* TimeToLastRxByte,                    */
				DurationToFloat(common.GetTimeToLastRxByte()),
				/* TimeToFirstUpstreamTxByte,           */
				DurationToFloat(common.GetTimeToFirstUpstreamTxByte()),
				/* TimeToLastUpstreamTxByte,            */
				DurationToFloat(common.GetTimeToLastUpstreamTxByte()),
				/* TimeToFirstUpstreamRxByte,           */
				DurationToFloat(common.GetTimeToFirstUpstreamRxByte()),
				/* TimeToLastUpstreamRxByte,            */
				DurationToFloat(common.GetTimeToLastUpstreamRxByte()),
				/* TimeToFirstDownstreamTxByte,         */
				DurationToFloat(common.GetTimeToFirstDownstreamTxByte()),
				/* TimeToLastDownstreamTxByte,          */
				DurationToFloat(common.GetTimeToLastDownstreamTxByte()),
				/* UpstreamRemoteAddress,               */
				upstream_remote_addr,
				/* UpstreamRemoteAddressPort,           */
				upstream_remote_port,
				/* UpstreamLocalAddress,                */
				upstream_local_addr,
				/* UpstreamLocalAddressPort,            */
				upstream_local_port,
				/* UpstreamCluster,                     */
				common.GetUpstreamCluster(),
				/* FailedLocalHealthcheck,              */
				respflags.GetFailedLocalHealthcheck(),
				/* NoHealthyUpstream,                   */
				respflags.GetNoHealthyUpstream(),
				/* UpstreamRequestTimeout,              */
				respflags.GetUpstreamRequestTimeout(),
				/* LocalReset,                          */
				respflags.GetLocalReset(),
				/* UpstreamRemoteReset,                 */
				respflags.GetUpstreamRemoteReset(),
				/* UpstreamConnectionFailure,           */
				respflags.GetUpstreamConnectionFailure(),
				/* UpstreamConnectionTermination,       */
				respflags.GetUpstreamConnectionTermination(),
				/* UpstreamOverflow,                    */
				respflags.GetUpstreamOverflow(),
				/* NoRouteFound,                        */
				respflags.GetNoRouteFound(),
				/* DelayInjected,                       */
				respflags.GetDelayInjected(),
				/* FaultInjected,                       */
				respflags.GetFaultInjected(),
				/* RateLimited,                         */
				respflags.GetRateLimited(),
				/* UnauthorizedDetails,                 */
				respflags.GetUnauthorizedDetails().GetReason().String(),
				/* RateLimitServiceError,               */
				respflags.GetRateLimitServiceError(),
				/* DownstreamConnectionTermination,     */
				respflags.GetDownstreamConnectionTermination(),
				/* UpstreamRetryLimitExceeded,          */
				respflags.GetUpstreamRetryLimitExceeded(),
				/* StreamIdleTimeout,                   */
				respflags.GetStreamIdleTimeout(),
				/* InvalidEnvoyRequestHeaders,          */
				respflags.GetInvalidEnvoyRequestHeaders(),
				/* DownstreamProtocolError,             */
				respflags.GetDownstreamProtocolError(),
				/* Curiefense,                          */
				jsonb_curiefense,
				/* UpstreamTransportFailureReason,      */
				common.GetUpstreamTransportFailureReason(),
				/* RouteName,                           */
				common.GetRouteName(),
				/* DownstreamDirectRemoteAddress,       */
				common.GetDownstreamDirectRemoteAddress().GetSocketAddress().GetAddress(),
				/* DownstreamDirectRemoteAddressPort,   */
				common.GetDownstreamDirectRemoteAddress().GetSocketAddress().GetPortValue(),
				/* TlsVersion,                          */
				tls.GetTlsVersion().String(),
				/* TlsCipherSuite,                      */
				tls.GetTlsCipherSuite().String(),
				/* TlsSniHostname,                      */
				tls.GetTlsSniHostname(),
				/* LocalCertificateProperties,          */
				tls.GetLocalCertificateProperties().GetSubject(),
				/* LocalCertificatePropertiesAltNames,  */
				jsonb_localaltnames,
				/* PeerCertificateProperties,           */
				tls.GetPeerCertificateProperties().GetSubject(),
				/* PeerCertificatePropertiesAltNames,   */
				jsonb_peeraltnames,
				/* TlsSessionId,                        */
				tls.GetTlsSessionId(),
				/* RequestMethod,                       */
				method,
				/* Scheme,                              */
				req.GetScheme(),
				/* Authority,                           */
				req.GetAuthority(),
				/* Port,                                */
				req.GetPort().GetValue(),
				/* Path,                                */
				req.GetPath(),
				/* UserAgent,                           */
				req.GetUserAgent(),
				/* Referer,                             */
				req.GetReferer(),
				/* ForwardedFor,                        */
				req.GetForwardedFor(),
				/* RequestId,                           */
				req.GetRequestId(),
				/* OriginalPath,                        */
				req.GetOriginalPath(),
				/* RequestHeadersBytes,                 */
				req.GetRequestHeadersBytes(),
				/* RequestBodyBytes,                    */
				req.GetRequestBodyBytes(),
				/* RequestHeaders,                      */
				jsonb_reqhdr,
				/* ResponseCode,                        */
				response_code,
				/* ResponseHeadersBytes,                */
				resp.GetResponseHeadersBytes(),
				/* ResponseBodyBytes                    */
				resp.GetResponseBodyBytes(),
				/* ResponseHeaders,                     */
				jsonb_resphdr,
				/* ResponseTrailers,                    */
				jsonb_resptrail,
				/* ResponseCodeDetails                  */
				resp.GetResponseCodeDetails(),
			); err == nil {
				log.Printf("[DEBUG] insert into pg ok")
			} else {
				log.Printf("[ERROR] insert into pg: Error: %v", err)
			}
		}
	}
	return err
}

func getEnv(key, fallback string) string {
	if value, ok := os.LookupEnv(key); ok {
		return value
	}
	return fallback
}

func readPassword(filename string) string {
	file, err := os.Open(filename)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		return scanner.Text()
	}
	log.Fatal("Could not read password")
	return ""
}

var (
	grpc_addr = getEnv("CURIELOGGER_GRPC_LISTEN", ":9001")
	prom_addr = getEnv("CURIELOGGER_PROMETHEUS_LISTEN", ":2112")
)

func main() {
	log.Print("Starting curielogger v0.2-dev6")
	var debug_mode = flag.Bool("d", false, "Debug mode")
	flag.Parse()
	var min_level string
	if *debug_mode {
		min_level = "DEBUG"
	} else {
		min_level = "ERROR"
	}
	filter := &logutils.LevelFilter{
		Levels: []logutils.LogLevel{"DEBUG", "ERROR"},
		MinLevel: logutils.LogLevel(min_level),
		Writer: os.Stderr,
	}
	log.SetOutput(filter)

	http.Handle("/metrics", promhttp.Handler())
	log.Printf("Prometheus exporter listening on %v", prom_addr)
	go http.ListenAndServe(prom_addr, nil)

	sock, err := net.Listen("tcp", grpc_addr)
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	log.Printf("GRPC server listening on %v", grpc_addr)
	s := grpc.NewServer()

	dburl, ok := os.LookupEnv("DATABASE_URL")
	var host string
	var password string
	if !ok {
		pwfilename, ok := os.LookupEnv("CURIELOGGER_DBPASSWORD_FILE")
		if ok {
			password = readPassword(pwfilename)
		} else {
			password = os.Getenv("CURIELOGGER_DBPASSWORD")
		}
		host = os.Getenv("CURIELOGGER_DBHOST")
		dburl = fmt.Sprintf(
			"host=%s dbname=curiefense user=%s password=%s",
			host,
			os.Getenv("CURIELOGGER_DBUSER"),
			password,
		)
	} else {
		re := regexp.MustCompile(`host=([^ ]+)`)
		host = re.FindStringSubmatch(dburl)[1]
	}
	als.RegisterAccessLogServiceServer(s, &server{db: nil, db_url: dburl, host: host})
	if err := s.Serve(sock); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
