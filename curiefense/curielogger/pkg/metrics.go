package pkg

import (
	"fmt"
	"net/http"
	"strconv"
	"strings"

	"github.com/curiefense/curiefense/curielogger/pkg/entities"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
	"github.com/prometheus/client_golang/prometheus/promhttp"
	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
)

const (
	PROMETHEUS_EXPORT_PORT    = `CURIELOGGER_PROMETHEUS_LISTEN`
	PROMETHEUS_EXPORT_ENABLED = `CURIELOGGER_METRICS_PROMETHEUS_ENABLED`
	namespace                 = "curiemetric" // For Prometheus metrics.
)

var (
	staticTags = map[string]bool{
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
)

type Metrics struct {
	requestCounter prometheus.Counter
	sessionDetails *prometheus.CounterVec
	requestBytes   prometheus.Counter
	responseBytes  prometheus.Counter
	requestTags    *prometheus.CounterVec
	loggerLatency  *prometheus.HistogramVec

	on bool
}

func NewMetrics(v *viper.Viper) *Metrics {
	if !v.GetBool(PROMETHEUS_EXPORT_ENABLED) {
		return &Metrics{on: false}
	}
	port := v.GetString(PROMETHEUS_EXPORT_PORT)
	if port == `` {
		port = `2112`
	}
	// set up prometheus server
	http.Handle("/metrics", promhttp.Handler())
	go http.ListenAndServe(fmt.Sprintf(`:%s`, port), nil)
	log.Infof("Prometheus exporter listening on %v", port)

	return &Metrics{
		on: true,
		requestCounter: promauto.NewCounter(
			prometheus.CounterOpts{
				Namespace: namespace,
				Name:      "http_request_total",
				Help:      "Total number of HTTP requests",
			},
		),
		sessionDetails: promauto.NewCounterVec(prometheus.CounterOpts{
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
		}),
		requestBytes: promauto.NewCounter(prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "request_bytes",
			Help:      "The total number of request bytes",
		}),
		responseBytes: promauto.NewCounter(prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "response_bytes",
			Help:      "The total number of response bytes",
		}),
		requestTags: promauto.NewCounterVec(prometheus.CounterOpts{
			Namespace: namespace,
			Name:      "session_tags_total",
			Help:      "Number of requests per label",
		}, []string{"tag"}),
		loggerLatency: promauto.NewHistogramVec(prometheus.HistogramOpts{
			Namespace: namespace,
			Name:      "logger_latency",
			Help:      "latency per logger",
		}, []string{"logger"}),
	}
}

func (m *Metrics) add(e *entities.LogEntry) {
	if !m.on {
		return
	}
	m.requestCounter.Inc()

	m.requestBytes.Add(float64(e.CfLog.Request.HeadersBytes + e.CfLog.Request.BodyBytes))
	m.responseBytes.Add(float64(e.CfLog.Response.HeadersBytes + e.CfLog.Response.BodyBytes))
	tags := e.CfLog.Tags
	blocked := strconv.FormatBool(e.CfLog.Blocked)
	var method string
	if e.CfLog.Request.Attributes.Method != "" {
		method = e.CfLog.Request.Attributes.Method
	}

	var uri string
	if e.CfLog.Request.Attributes.URI != "" {
		uri = e.CfLog.Request.Attributes.URI
	}

	labels := makeLabels(e.CfLog.Response.Code, method, uri, e.CfLog.Upstream.RemoteAddress, blocked, tags)

	m.sessionDetails.With(labels).Inc()

	for _, name := range e.CfLog.Tags {
		if !isStaticTag(name) {
			m.requestTags.WithLabelValues(name).Inc()
		}
	}
}

func isStaticTag(tag string) bool {
	if tag == "all" {
		return true
	}
	parts := strings.Split(tag, ":")
	if len(parts) > 1 {
		return staticTags[parts[0]]
	}
	return false
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

func makeLabels(statusCode int, method, path, upstream, blocked string, tags []string) prometheus.Labels {
	// classes and specific response code
	// icode := int(statusCode)
	classLabel := "status_Nxx"
	if statusCode/100 >= 2 && statusCode/100 <= 5 {
		classLabel = fmt.Sprintf("status_%dxx", statusCode/100)
	}
	statusCodeStr := strconv.Itoa(statusCode)

	origin := "N/A"
	originStatusCode := "N/A"
	originStatusClass := "N/A"

	if len(upstream) > 0 {
		origin = upstream
		originStatusCode = fmt.Sprintf("origin_%s", statusCodeStr)
		originStatusClass = fmt.Sprintf("origin_%s", classLabel)
	}

	tm := makeTagMap(tags)

	return prometheus.Labels{
		"status_code":         statusCodeStr,
		"status_class":        classLabel,
		"origin":              origin,
		"origin_status_code":  originStatusCode,
		"origin_status_class": originStatusClass,
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
