package main

import (
	"context"
	"encoding/json"
	"fmt"

	"bufio"
	"bytes"
	"flag"
	"log"
	"net"
	"os"
	"regexp"
	"strconv"
	"strings"
	"time"

	"google.golang.org/grpc"

	ald "github.com/envoyproxy/go-control-plane/envoy/data/accesslog/v2"
	als "github.com/envoyproxy/go-control-plane/envoy/service/accesslog/v2"

	//	ptypes "github.com/golang/protobuf/ptypes"
	duration "github.com/golang/protobuf/ptypes/duration"
	timestamp "github.com/golang/protobuf/ptypes/timestamp"

	"github.com/jackc/pgx/pgtype"
	"github.com/jackc/pgx/v4"

	"net/http"
	neturl "net/url"

	"github.com/hashicorp/logutils"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
	"github.com/prometheus/client_golang/prometheus/promhttp"

	"errors"

	elasticsearch "github.com/elastic/go-elasticsearch/v7"
)

//   ___ ___  __  __ __  __  ___  _  _
//  / __/ _ \|  \/  |  \/  |/ _ \| \| |
// | (_| (_) | |\/| | |\/| | (_) | .` |
//  \___\___/|_|  |_|_|  |_|\___/|_|\_|
// COMMON

type CurieProxyLog struct {
	Headers     map[string]string      `json:"headers"`
	Cookies     map[string]string      `json:"cookies"`
	Geo         map[string]float64     `json:"geo"`
	Arguments   map[string]string      `json:"arguments"`
	Attributes  map[string]interface{} `json:"attributes"`
	Blocked     bool                   `json:"blocked"`
	BlockReason map[string]interface{} `json:"block_reason"`
	Tags        []string               `json:"tags"`
}

type RXTimer struct {
	FirstUpstreamByte float64 `json:"firstupstreambyte"`
	LastUpstreamByte  float64 `json:"lastupstreambyte"`
	LastByte          float64 `json:"lastbyte"`
}

type TXTimer struct {
	FirstUpstreamByte   float64 `json:"firstupstreambyte"`
	LastUpstreamByte    float64 `json:"lastupstreambyte"`
	FirstDownstreamByte float64 `json:"firstdownstreambyte"`
	LastDownstreamByte  float64 `json:"lastdownstreambyte"`
}

type DownstreamData struct {
	ConnectionTermination   bool   `json:"connectiontermination"`
	DirectRemoteAddress     string `json:"directremoteaddress"`
	DirectRemoteAddressPort uint32 `json:"directremoteaddressport"`
	LocalAddress            string `json:"localaddress"`
	LocalAddressPort        uint32 `json:"localaddressport"`
	ProtocolError           bool   `json:"protocolerror"`
	RemoteAddress           string `json:"remoteaddress"`
	RemoteAddressPort       uint32 `json:"remoteaddressport"`
}

type UpstreamData struct {
	Cluster                string `json:"cluster"`
	ConnectionFailure      bool   `json:"connectionfailure"`
	ConnectionTermination  bool   `json:"connectiontermination"`
	LocalAddress           string `json:"localaddress,omitempty"`
	LocalAddressPort       uint32 `json:"localaddressport,omitempty"`
	Overflow               bool   `json:"overflow"`
	RemoteAddress          string `json:"remoteaddress,omitempty"`
	RemoteAddressPort      uint32 `json:"remoteaddressport,omitempty"`
	RemoteReset            bool   `json:"remotereset"`
	RequestTimeout         bool   `json:"requesttimeout"`
	RetryLimitExceeded     bool   `json:"retrylimitexceeded"`
	TransportFailureReason string `json:"transportfailurereason"`
}

type CertificateData struct {
	Properties         string   `json:"properties"`
	PropertiesAltNames []string `json:"propertiesaltnames"`
}

type TLSData struct {
	LocalCertificate CertificateData `json:"localcertificate"`
	PeerCertificate  CertificateData `json:"peercertificate"`
	CipherSuite      string          `json:"ciphersuite"`
	SessionId        string          `json:"sessionid"`
	SNIHostname      string          `json:"snihostname"`
	Version          string          `json:"version"`
}

type NameValue struct {
	Name  string `json:"name"`
	Value string `json:"value"`
}

type RequestData struct {
	BodyBytes    uint64                 `json:"bodybytes"`
	HeadersBytes uint64                 `json:"headersbytes"`
	OriginalPath string                 `json:"originalpath"`
	Headers      map[string]string      `json:"headers"`
	Cookies      map[string]string      `json:"cookies"`
	Arguments    map[string]string      `json:"arguments"`
	Geo          map[string]float64     `json:"geo"`
	Attributes   map[string]interface{} `json:"attributes"`
}

type ResponseData struct {
	BodyBytes    uint64            `json:"bodybytes"`
	Code         int               `json:"code"`
	CodeDetails  string            `json:"codedetails"`
	Headers      map[string]string `json:"headers"`
	HeadersBytes uint64            `json:"headersbytes"`
	Trailers     map[string]string `json:"trailers"`
}

type MetadataData struct {
	DelayInjected              bool    `json:"delayinjected"`
	FailedLocalHealthCheck     bool    `json:"failedlocalhealthcheck"`
	FaultInjected              bool    `json:"faultinjected"`
	InvalidEnvoyRequestHeaders bool    `json:"invalidenvoyrequestheaders"`
	LocalReset                 bool    `json:"localreset"`
	NoHealthyUpstream          bool    `json:"nohealthyupstream"`
	NoRouteFound               bool    `json:"noroutefound"`
	RateLimited                bool    `json:"ratelimited"`
	RateLimitServiceError      bool    `json:"ratelimitserviceerror"`
	RouteName                  string  `json:"routename"`
	SampleRate                 float64 `json:"samplerate"`
	StreamIdleTimeout          bool    `json:"streamidletimeout"`
	UnauthorizedDetails        string  `json:"unauthorizeddetails"`
}

type CuriefenseLog struct {
	RequestId string `json:"requestid"`
	Timestamp string `json:"timestamp"`
	Scheme    string `json:"scheme"`
	Authority string `json:"authority"`
	Port      uint32 `json:"port"`
	Method    string `json:"method"`
	Path      string `json:"path"`

	Blocked     bool                   `json:"blocked"`
	BlockReason map[string]interface{} `json:"block_reason"`
	Tags        []string               `json:"tags"`

	RXTimers RXTimer `json:"timers"`
	TXTimers TXTimer `json:"timers"`

	Upstream   UpstreamData   `json:"upstream"`
	Downstream DownstreamData `json:"downstream"`

	TLS      TLSData      `json:"tls"`
	Request  RequestData  `json:"request"`
	Response ResponseData `json:"response"`
	Metadata MetadataData `json:"metadata"`
}

type LogEntry struct {
	fullEntry     *ald.HTTPAccessLogEntry
	cfLog         *CuriefenseLog
	curieProxyLog *CurieProxyLog
}

type Logger interface {
	ConfigureFromEnv(name string, envVar string, channel_capacity int) error
	Start()
	GetLogEntry() LogEntry
	SendEntry(e LogEntry)
	InsertEntry(e LogEntry) bool
}

type logger struct {
	name      string
	channel   chan LogEntry
	url       string
	do_insert func(LogEntry) bool
}

func (l logger) SendEntry(e LogEntry) {
	if len(l.channel) >= cap(l.channel) {
		metric_dropped_log_entry.With(prometheus.Labels{"logger": l.name}).Inc()
		log.Printf("[WARNING] [%s] buffer full (%v/%v). Log entry dropped", l.name, len(l.channel), cap(l.channel))
	} else {
		l.channel <- e
	}
}

func (l *logger) ConfigureFromEnv(name string, envVar string, channel_capacity int) error {
	url, ok := os.LookupEnv(envVar)
	if !ok {
		return errors.New(fmt.Sprintf("Did not find %s in environment", envVar))
	}
	ch := make(chan LogEntry, channel_capacity)
	l.name = name
	l.url = url
	l.channel = ch
	if l.do_insert == nil {
		l.do_insert = l.InsertEntry
	}
	return nil
}

func (l logger) Start() {
	log.Printf("[INFO]: %s logging routine started", l.name)
	for {
		entry := l.GetLogEntry()
		now := time.Now()
		l.do_insert(entry)
		metric_logger_latency.With(prometheus.Labels{"logger": l.name}).Observe(time.Since(now).Seconds())
	}
}

func (l logger) GetLogEntry() LogEntry {
	return <-l.channel
}

func (l logger) InsertEntry(e LogEntry) bool {
	log.Printf("[ERROR] entry insertion not implemented")
	return false
}

type grpcServerParams struct {
	loggers []Logger
}

//  ___ ___  ___  __  __ ___ _____ _  _ ___ _   _ ___
// | _ \ _ \/ _ \|  \/  | __|_   _| || | __| | | / __|
// |  _/   / (_) | |\/| | _|  | | | __ | _|| |_| \__ \
// |_| |_|_\\___/|_|  |_|___| |_| |_||_|___|\___/|___/
// PROMETHEUS

const (
	namespace = "curiemetric" // For Prometheus metrics.
)

/** Prometheus metrics **/

var (
	metric_requests = promauto.NewCounter(
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

	metric_dropped_log_entry = promauto.NewCounterVec(prometheus.CounterOpts{
		Namespace: namespace,
		Name:      "dropped_log_entries",
		Help:      "number of dropped log entries per logger",
	}, []string{"logger"})

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

	metric_requests_tags = promauto.NewCounterVec(prometheus.CounterOpts{
		Namespace: namespace,
		Name:      "session_tags_total",
		Help:      "Number of requests per label",
	}, []string{"tag"})

	metric_logger_latency = promauto.NewHistogramVec(prometheus.HistogramOpts{
		Namespace: namespace,
		Name:      "logger_latency",
		Help:      "latency per logger",
	}, []string{"logger"})
)

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

func makeTagMap(tags []string) map[string]string {
	res := make(map[string]string)
	for _, k := range tags {
		tspl := strings.Split(k, ":")
		if len(tspl) == 2 {
			res[tspl[0]] = tspl[1]
		}
	}
	return res
}

func makeLabels(status_code int, method, path, upstream, blocked string, tags []string) prometheus.Labels {

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

	tm := makeTagMap(tags)

	return prometheus.Labels{
		"status_code":         status_code_str,
		"status_class":        class_label,
		"origin":              origin,
		"origin_status_code":  origin_status_code,
		"origin_status_class": origin_status_class,
		"method":              method,
		"path":                path,
		"blocked":             blocked,
		"asn":                 tm["asn"],
		"geo":                 tm["geo"],
		"aclid":               tm["aclid"],
		"aclname":             tm["aclname"],
		"wafid":               tm["wafid"],
		"wafname":             tm["wafname"],
		"urlmap":              tm["urlmap"],
		"urlmap_entry":        tm["urlmap-entry"],
		"container":           tm["container"],
	}
}

type promLogger struct {
	logger
}

func (l promLogger) Start() {
	log.Printf("[INFO]: Prometheus (%s) metrics updating routine started", l.name)

	for {
		e := l.GetLogEntry()
		log.Printf("[DEBUG] new log entry cflog=%v", *e.cfLog)
		// **** Update prometheus metrics ****
		metric_requests.Inc()
		metric_request_bytes.Add(float64(e.cfLog.Request.HeadersBytes + e.cfLog.Request.BodyBytes))
		metric_response_bytes.Add(float64(e.cfLog.Response.HeadersBytes + e.cfLog.Response.BodyBytes))

		tags := e.cfLog.Tags
		blocked := strconv.FormatBool(e.cfLog.Blocked)

		labels := makeLabels(e.cfLog.Response.Code, e.cfLog.Method, e.cfLog.Path,
			e.cfLog.Upstream.RemoteAddress, blocked, tags)
		metric_session_details.With(labels).Inc()

		for _, name := range tags {
			if !isStaticTag(name) {
				metric_requests_tags.WithLabelValues(name).Inc()
			}
		}
	}
}

//  ___  ___  ___ _____ ___ ___ ___ ___  ___  _
// | _ \/ _ \/ __|_   _/ __| _ \ __/ __|/ _ \| |
// |  _/ (_) \__ \ | || (_ |   / _|\__ \ (_) | |__
// |_|  \___/|___/ |_| \___|_|_\___|___/\__\_\____|
// POSTGRESQL

type pgLogger struct {
	logger
	host string
	db   *pgx.Conn
}

func makeJsonb(v interface{}) *pgtype.JSON {
	j, err := json.Marshal(v)
	if err != nil {
		j = []byte("{}")
	}
	return &pgtype.JSON{Bytes: j, Status: pgtype.Present}
}

func (l pgLogger) getDB() *pgx.Conn {
	for l.db == nil || l.db.IsClosed() {
		conn, err := pgx.Connect(context.Background(), l.url)
		if err == nil {
			l.db = conn
			log.Printf("[DEBUG] Connected to database %v\n", l.host)
			break
		}
		log.Printf("[ERROR] Could not connect to database %v: %v\n", l.host, err)
		time.Sleep(time.Second)
	}
	return l.db
}

func (l *pgLogger) InsertEntry(e LogEntry) bool {
	db := l.getDB() // needed to ensure db cnx is not closed and reopen it if needed

	cfl := e.cfLog

	_, err := db.Exec(context.Background(), `insert into logs (
		RequestId, Timestamp, Scheme, Authority, Port, Method, Path,
		RXTimers, TXTimers, Upstream, Downstream, TLS, Request, Response, Metadata)

	    values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)`,

		cfl.RequestId,
		cfl.Timestamp, // StartTime
		cfl.Scheme,
		cfl.Authority,
		cfl.Port,
		cfl.Method,
		cfl.Path,
		makeJsonb(cfl.RXTimers),
		makeJsonb(cfl.TXTimers),
		makeJsonb(cfl.Upstream),
		makeJsonb(cfl.Downstream),
		makeJsonb(cfl.TLS),
		makeJsonb(cfl.Request),
		makeJsonb(cfl.Response),
		makeJsonb(cfl.Metadata),
	)
	if err == nil {
		log.Printf("[DEBUG] insert into pg ok")
	} else {
		log.Printf("[ERROR] insert into pg: Error: %v", err)
	}
	return err != nil
	return true
}

//  ___ _      _   ___ _____ ___ ___ ___ ___   _   ___  ___ _  _
// | __| |    /_\ / __|_   _|_ _/ __/ __| __| /_\ | _ \/ __| || |
// | _|| |__ / _ \\__ \ | |  | | (__\__ \ _| / _ \|   / (__| __ |
// |___|____/_/ \_\___/ |_| |___\___|___/___/_/ \_\_|_\\___|_||_|
// ELASTICSEARCH

type esLogger struct {
	logger
	client *elasticsearch.Client
}

func (l esLogger) getESClient() *elasticsearch.Client {
	for l.client == nil {
		cfg := elasticsearch.Config{
			Addresses: []string{l.url},
		}
		conn, err := elasticsearch.NewClient(cfg)
		if err == nil {
			l.client = conn
			log.Printf("[DEBUG] Connected to elasticsearch %v\n", l.url)
			break
		}
		log.Printf("[ERROR] Could not connect to elasticsearch [%v]: %v\n", l.url, err)
		time.Sleep(time.Second)
	}
	return l.client
}

func (l *esLogger) InsertEntry(e LogEntry) bool {
	log.Printf("[DEBUG] ES insertion!")
	client := l.getESClient() // needed to ensure ES cnx is not closed and reopen it if needed
	j, err := json.Marshal(e.cfLog)
	if err == nil {
		client.Index(
			"log",
			strings.NewReader(string(j)),
			client.Index.WithRefresh("true"),
			client.Index.WithPretty(),
			client.Index.WithFilterPath("result", "_id"),
		)
	} else {
		log.Printf("[ERROR] Could not convert protobuf entry into json for ES insertion.")
		return false
	}
	return true
}

//  _    ___   ___ ___ _____ _   ___ _  _
// | |  / _ \ / __/ __|_   _/_\ / __| || |
// | |_| (_) | (_ \__ \ | |/ _ \\__ \ __ |
// |____\___/ \___|___/ |_/_/ \_\___/_||_|
// LOGSTASH

type logstashLogger struct {
	logger
}

func (l *logstashLogger) InsertEntry(e LogEntry) bool {
	log.Printf("[DEBUG] LogStash insertion!")
	j, err := json.Marshal(e.cfLog)
	if err == nil {
		_, err := http.Post(l.url, "application/json", bytes.NewReader(j))
		if err != nil {
			log.Printf("ERROR: could not POST log entry: %v", err)
			return false
		}
	} else {
		log.Printf("[ERROR] Could not convert protobuf entry into json for ES insertion.")
		return false
	}
	return true
}

//  ___ _   _   _ ___ _  _ _____ ___
// | __| | | | | | __| \| |_   _|   \
// | _|| |_| |_| | _|| .` | | | | |) |
// |_| |____\___/|___|_|\_| |_| |___/
// FLUENTD

type fluentdLogger struct {
	logger
}

func (l *fluentdLogger) InsertEntry(e LogEntry) bool {
	log.Printf("[DEBUG] Fluentd insertion!")
	j, err := json.Marshal(e.cfLog)
	if err == nil {
		_, err := http.PostForm(l.url+"curiefense.log", neturl.Values{"json": {string(j)}})
		if err != nil {
			log.Printf("ERROR: could not POST log entry: %v", err)
		}
	} else {
		log.Printf("[ERROR] Could not convert protobuf entry into json for ES insertion.")
	}
	return true
}

//   ___ ___ ___  ___     _   ___ ___ ___ ___ ___   _    ___   ___ ___
//  / __| _ \ _ \/ __|   /_\ / __/ __| __/ __/ __| | |  / _ \ / __/ __|
// | (_ |   /  _/ (__   / _ \ (_| (__| _|\__ \__ \ | |_| (_) | (_ \__ \
//  \___|_|_\_|  \___| /_/ \_\___\___|___|___/___/ |____\___/ \___|___/
// GRPC ACCESS LOGS

func DurationToFloat(d *duration.Duration) float64 {
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

func MapToNameValue(m map[string]string) []NameValue {
	var res []NameValue
	for k, v := range m {
		res = append(res, NameValue{k, v})
	}
	return res
}

func (s grpcServerParams) StreamAccessLogs(x als.AccessLogService_StreamAccessLogsServer) error {
	msg, err := x.Recv()
	if err != nil {
		log.Printf("[ERROR] Error receiving grpc stream message: %v", err)
	} else {
		log.Printf("[DEBUG] ====>[%v]", msg.LogEntries)
		hl := msg.GetHttpLogs()
		http_entries := hl.GetLogEntry()
		for _, entry := range http_entries {

			common := entry.GetCommonProperties()
			// Decode curiefense metadata

			curiefense_meta, got_meta := common.GetMetadata().GetFilterMetadata()["com.reblaze.curiefense"]
			if !got_meta { /* This log line was not generated by curiefense */
				log.Printf("[DEBUG] No curiefense metadata => drop log entry")
				continue
			}

			var curieProxyLog CurieProxyLog

			cfm := curiefense_meta.GetFields()
			log.Printf("%v", cfm)
			if rqinfo_s, ok := cfm["request.info"]; ok {
				curiefense_json_string := rqinfo_s.GetStringValue()
				err := json.Unmarshal([]byte(curiefense_json_string), &curieProxyLog)
				if err != nil {
					log.Printf("[ERROR] Error unmarshalling metadata json string [%v]: %v", curiefense_json_string, err)
					continue
				}
			} else {
				log.Printf("[ERROR] did not find request.info in curiefense medatada")
				continue
			}

			log.Printf("[DEBUG] XXXXXXXX curieproxylog=%v", curieProxyLog)

			// Shortcuts

			req := entry.GetRequest()
			resp := entry.GetResponse()
			respflags := common.GetResponseFlags()
			tls := common.GetTlsProperties()
			lan := []string{}
			for _, san := range tls.GetLocalCertificateProperties().GetSubjectAltName() {
				lan = append(lan, san.String())
			}
			pan := []string{}
			for _, san := range tls.GetPeerCertificateProperties().GetSubjectAltName() {
				pan = append(pan, san.String())
			}

			// Create canonical curiefense log structure

			cflog := CuriefenseLog{
				RequestId:   req.GetRequestId(),
				Timestamp:   TimestampToRFC3339(common.GetStartTime()),
				Scheme:      req.GetScheme(),
				Authority:   req.GetAuthority(),
				Port:        req.GetPort().GetValue(),
				Method:      req.GetRequestMethod().String(),
				Path:        req.GetPath(),
				Blocked:     curieProxyLog.Blocked,
				BlockReason: curieProxyLog.BlockReason,
				Tags:        curieProxyLog.Tags,

				RXTimers: RXTimer{
					FirstUpstreamByte: DurationToFloat(common.GetTimeToFirstUpstreamRxByte()),
					LastUpstreamByte:  DurationToFloat(common.GetTimeToLastUpstreamRxByte()),
					LastByte:          DurationToFloat(common.GetTimeToLastRxByte()),
				},
				TXTimers: TXTimer{
					FirstUpstreamByte:   DurationToFloat(common.GetTimeToFirstUpstreamTxByte()),
					LastUpstreamByte:    DurationToFloat(common.GetTimeToLastUpstreamTxByte()),
					FirstDownstreamByte: DurationToFloat(common.GetTimeToFirstDownstreamTxByte()),
					LastDownstreamByte:  DurationToFloat(common.GetTimeToLastDownstreamTxByte()),
				},
				Downstream: DownstreamData{
					ConnectionTermination:   respflags.GetDownstreamConnectionTermination(),
					DirectRemoteAddress:     common.GetDownstreamDirectRemoteAddress().GetSocketAddress().GetAddress(),
					DirectRemoteAddressPort: common.GetDownstreamDirectRemoteAddress().GetSocketAddress().GetPortValue(),
					LocalAddress:            common.GetDownstreamLocalAddress().GetSocketAddress().GetAddress(),
					LocalAddressPort:        common.GetDownstreamLocalAddress().GetSocketAddress().GetPortValue(),
					ProtocolError:           respflags.GetDownstreamProtocolError(),
					RemoteAddress:           common.GetDownstreamRemoteAddress().GetSocketAddress().GetAddress(),
					RemoteAddressPort:       common.GetDownstreamRemoteAddress().GetSocketAddress().GetPortValue(),
				},
				Upstream: UpstreamData{
					Cluster:                common.GetUpstreamCluster(),
					ConnectionFailure:      respflags.GetUpstreamConnectionFailure(),
					ConnectionTermination:  respflags.GetUpstreamConnectionTermination(),
					LocalAddress:           common.GetUpstreamLocalAddress().GetSocketAddress().GetAddress(),
					LocalAddressPort:       common.GetUpstreamLocalAddress().GetSocketAddress().GetPortValue(),
					Overflow:               respflags.GetUpstreamOverflow(),
					RemoteAddress:          common.GetUpstreamRemoteAddress().GetSocketAddress().GetAddress(),
					RemoteAddressPort:      common.GetUpstreamRemoteAddress().GetSocketAddress().GetPortValue(),
					RemoteReset:            respflags.GetUpstreamRemoteReset(),
					RequestTimeout:         respflags.GetUpstreamRequestTimeout(),
					RetryLimitExceeded:     respflags.GetUpstreamRetryLimitExceeded(),
					TransportFailureReason: common.GetUpstreamTransportFailureReason(),
				},
				TLS: TLSData{
					LocalCertificate: CertificateData{
						Properties:         tls.GetLocalCertificateProperties().GetSubject(),
						PropertiesAltNames: lan,
					},
					PeerCertificate: CertificateData{
						Properties:         tls.GetPeerCertificateProperties().GetSubject(),
						PropertiesAltNames: pan,
					},
					CipherSuite: tls.GetTlsCipherSuite().String(),
					SessionId:   tls.GetTlsSessionId(),
					SNIHostname: tls.GetTlsSniHostname(),
					Version:     tls.GetTlsVersion().String(),
				},
				Request: RequestData{
					BodyBytes:    req.GetRequestBodyBytes(),
					HeadersBytes: req.GetRequestHeadersBytes(),
					OriginalPath: req.GetOriginalPath(),
					Headers:      curieProxyLog.Headers,
					Cookies:      curieProxyLog.Cookies,
					Arguments:    curieProxyLog.Arguments,
					Geo:          curieProxyLog.Geo,
					Attributes:   curieProxyLog.Attributes,
				},
				Response: ResponseData{
					BodyBytes:    resp.GetResponseBodyBytes(),
					Code:         int(resp.GetResponseCode().GetValue()),
					CodeDetails:  resp.GetResponseCodeDetails(),
					Headers:      resp.GetResponseHeaders(),
					HeadersBytes: resp.GetResponseHeadersBytes(),
					Trailers:     resp.GetResponseTrailers(),
				},
				Metadata: MetadataData{
					DelayInjected:              respflags.GetDelayInjected(),
					FailedLocalHealthCheck:     respflags.GetFailedLocalHealthcheck(),
					FaultInjected:              respflags.GetFaultInjected(),
					InvalidEnvoyRequestHeaders: respflags.GetInvalidEnvoyRequestHeaders(),
					LocalReset:                 respflags.GetLocalReset(),
					NoHealthyUpstream:          respflags.GetNoHealthyUpstream(),
					NoRouteFound:               respflags.GetNoRouteFound(),
					RateLimited:                respflags.GetRateLimited(),
					RateLimitServiceError:      respflags.GetRateLimitServiceError(),
					RouteName:                  common.GetRouteName(),
					SampleRate:                 common.GetSampleRate(),
					StreamIdleTimeout:          respflags.GetStreamIdleTimeout(),
					UnauthorizedDetails:        respflags.GetUnauthorizedDetails().GetReason().String(),
				},
			}

			log.Printf("[DEBUG] ---> [ %v:%v %v:%v ] <---", cflog.Downstream.RemoteAddress, cflog.Downstream.RemoteAddressPort,
				cflog.Downstream.LocalAddress, cflog.Downstream.LocalAddressPort)

			log.Printf("[DEBUG] cflog=%v", cflog)

			log_entry := LogEntry{
				fullEntry:     entry,
				cfLog:         &cflog,
				curieProxyLog: &curieProxyLog,
			}

			for _, l := range s.loggers {
				l.SendEntry(log_entry)
			}
		}
	}
	return nil
}

//  __  __   _   ___ _  _
// |  \/  | /_\ |_ _| \| |
// | |\/| |/ _ \ | || .` |
// |_|  |_/_/ \_\___|_|\_|
// MAIN

func getEnv(key, fallback string) string {
	if value, ok := os.LookupEnv(key); ok {
		return value
	}
	return fallback
}

func check_env_flag(env_var string) bool {
	value, ok := os.LookupEnv(env_var)
	return ok && value != "" && value != "0" && strings.ToLower(value) != "false"
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
	log.Print("Starting curielogger v0.3-dev1")

	var debug_mode = flag.Bool("d", false, "Debug mode")
	var channel_capacity = flag.Int("b", 65536, "log channel capacity")

	flag.Parse()

	// configure log level
	var min_level string
	if *debug_mode {
		min_level = "DEBUG"
	} else {
		min_level = "ERROR"
	}
	filter := &logutils.LevelFilter{
		Levels:   []logutils.LogLevel{"DEBUG", "ERROR"},
		MinLevel: logutils.LogLevel(min_level),
		Writer:   os.Stderr,
	}
	log.SetOutput(filter)

	log.Printf("[INFO] Channel capacity set at %v", *channel_capacity)

	// set up prometheus server
	http.Handle("/metrics", promhttp.Handler())
	log.Printf("Prometheus exporter listening on %v", prom_addr)
	go http.ListenAndServe(prom_addr, nil)

	////////////////////
	// set up loggers //
	////////////////////

	var loggers []Logger

	// Prometheus
	prom := promLogger{logger{name: "prometheus", channel: make(chan LogEntry, *channel_capacity)}}
	prom.do_insert = prom.InsertEntry // manual OOP :( thanks Go.
	loggers = append(loggers, &prom)

	// PostgreSQL
	if check_env_flag("CURIELOGGER_USES_POSTGRES") {
		db_url, ok := os.LookupEnv("DATABASE_URL")
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
			db_url = fmt.Sprintf(
				"host=%s dbname=curiefense user=%s password=%s",
				host,
				os.Getenv("CURIELOGGER_DBUSER"),
				password,
			)
		} else {
			re := regexp.MustCompile(`host=([^ ]+)`)
			host = re.FindStringSubmatch(db_url)[1]
		}
		pg := pgLogger{logger: logger{name: "postgres", url: db_url, channel: make(chan LogEntry, *channel_capacity)}, host: host}
		pg.do_insert = pg.InsertEntry // manual OOP :( thanks Go.
		loggers = append(loggers, &pg)
	}

	// ElasticSearch
	if check_env_flag("CURIELOGGER_USES_ELASTICSEARCH") {
		es := esLogger{}
		es.do_insert = es.InsertEntry // manual OOP :( thanks Go.
		es.ConfigureFromEnv("ElasticSearch", "CURIELOGGER_ELASTICSEARCH_URL", *channel_capacity)
		loggers = append(loggers, &es)
	}

	// Logstash
	if check_env_flag("CURIELOGGER_USES_LOGSTASH") {
		ls := logstashLogger{}
		ls.do_insert = ls.InsertEntry // manual OOP :( thanks Go.
		ls.ConfigureFromEnv("LogStash", "CURIELOGGER_LOGSTASH_URL", *channel_capacity)
		loggers = append(loggers, &ls)
	}

	// Fluentd
	if check_env_flag("CURIELOGGER_USES_FLUENTD") {
		fd := fluentdLogger{}
		fd.do_insert = fd.InsertEntry // manual OOP :( thanks Go.
		fd.ConfigureFromEnv("Fluentd", "CURIELOGGER_FLUENTD_URL", *channel_capacity)
		loggers = append(loggers, &fd)
	}

	for _, l := range loggers {
		go l.Start()
	}

	////////////////////////
	// set up GRPC server //
	////////////////////////

	sock, err := net.Listen("tcp", grpc_addr)
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	log.Printf("GRPC server listening on %v", grpc_addr)
	s := grpc.NewServer()

	als.RegisterAccessLogServiceServer(s, &grpcServerParams{loggers: loggers})
	if err := s.Serve(sock); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
