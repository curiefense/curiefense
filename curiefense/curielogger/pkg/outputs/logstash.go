package outputs

import (
	"bytes"
	"net/http"

	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
)

const (
	CURIELOGGER_OUTPUTS_LOGSTASH_URL = `CURIELOGGER_OUTPUTS_LOGSTASH_URL`
)

type Logstash struct {
	url string
}

type LogstashConfig struct {
	Enabled       bool                `mapstructure:"enabled"`
	Url           string              `mapstructure:"url"`
	Elasticsearch ElasticsearchConfig `mapstructure:"elasticsearch"`
}

func NewLogstash(v *viper.Viper, cfg LogstashConfig) *Logstash {
	log.Info(`initialized logstash`)
	log.Warn("The Logstash output is deprecated and will be removed in the 1.5.0 release. More on the reasoning and discussion here: https://github.com/curiefense/curiefense/issues/317")
	url := v.GetString(CURIELOGGER_OUTPUTS_LOGSTASH_URL)
	if url == `` {
		url = cfg.Url
	}
	if es := NewElasticSearch(v, cfg.Elasticsearch); es != nil { // configures ES client and closes connections
		es.Close()
	}

	return &Logstash{url: url}
}

func (l *Logstash) Write(p []byte) (n int, err error) {
	r, err := http.Post(l.url, "application/json", bytes.NewReader(p))
	if err != nil {
		return 0, err
	}
	return len(p), r.Body.Close()
}

func (l *Logstash) Close() error {
	return nil
}
