package outputs

import (
	"io"
	"os"

	"github.com/spf13/viper"
)

const (
	STDOUT_ENABLED   = `STDOUT_ENABLED`
	BUCKET_ENABLED   = `EXPORT_BUCKET_ENABLED`
	FLUENTD_ENABLED  = `CURIELOGGER_USES_FLUENTD`
	LOGSTASH_ENABLED = `CURIELOGGER_OUTPUTS_LOGSTASH_ENABLED`
	ES_ENABLED       = `CURIELOGGER_OUTPUTS_ELASTICSEARCH_ENABLED`
)

type Config struct {
	LogLevel string        `mapstructure:"log_level" validate:"one_of=info,debug,error"`
	Outputs  OutputsConfig `mapstructure:"outputs,omitempty"`
}

type OutputsConfig struct {
	Elasticsearch ElasticsearchConfig `mapstructure:"elasticsearch,omitempty"`
	Logstash      LogstashConfig      `mapstructure:"logstash,omitempty"`
}

func InitOutputs(v *viper.Viper, cfg Config) io.WriteCloser {
	output := make([]io.WriteCloser, 0)
	if v.GetBool(STDOUT_ENABLED) {
		output = append(output, os.Stdout)
	}

	if v.GetBool(FLUENTD_ENABLED) {
		output = append(output, NewFluentD(v))
	}
	if v.GetBool(LOGSTASH_ENABLED) || cfg.Outputs.Logstash.Enabled {
		output = append(output, NewLogstash(v, cfg.Outputs.Logstash))
	}
	if v.GetBool(ES_ENABLED) || cfg.Outputs.Elasticsearch.Enabled {
		if es := NewElasticSearch(v, cfg.Outputs.Elasticsearch); es != nil {
			output = append(output, es)
		}
	}
	return NewTee(output)
}
