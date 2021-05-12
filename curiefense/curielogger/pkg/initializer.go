package pkg

import (
	"io"

	"github.com/spf13/viper"

	"github.com/curiefense/curiefense/curielogger/pkg/outputs"
)

const (
	FLUENTD_ENABLED  = `CURIELOGGER_USES_FLUENTD`
	STDOUT_ENABLED   = `CURIELOGGER_OUTPUTS_STDOUT_ENABLED`
	BUCKET_ENABLED   = `CURIELOGGER_OUTPUTS_BUCKET_ENABLED`
	LOGSTASH_ENABLED = `CURIELOGGER_OUTPUTS_LOGSTASH_ENABLED`
	ES_ENABLED       = `CURIELOGGER_OUTPUTS_ELASTICSEARCH_ENABLED`
)

type OutputsConfig struct {
	Elasticsearch outputs.ElasticsearchConfig `mapstructure:"elasticsearch,omitempty"`
	Logstash      outputs.LogstashConfig      `mapstructure:"logstash,omitempty"`
	Stdout        outputs.StdoutConfig        `mapstructure:"stdout,omitempty"`
	Bucket        outputs.BucketConfig        `mapstructure:"bucket,omitempty"`
}

func InitOutputs(v *viper.Viper, cfg Config) io.WriteCloser {
	output := make([]io.WriteCloser, 0)
	if v.GetBool(STDOUT_ENABLED) || cfg.Outputs.Stdout.Enabled {
		output = append(output, outputs.NewStdout(v))
	}

	if v.GetBool(BUCKET_ENABLED) || cfg.Outputs.Bucket.Enabled {
		output = append(output, outputs.NewBucket(v, cfg.Outputs.Bucket))
	}

	if v.GetBool(FLUENTD_ENABLED) {
		output = append(output, outputs.NewFluentD(v))
	}
	if v.GetBool(LOGSTASH_ENABLED) || cfg.Outputs.Logstash.Enabled {
		output = append(output, outputs.NewLogstash(v, cfg.Outputs.Logstash))
	}
	if v.GetBool(ES_ENABLED) || cfg.Outputs.Elasticsearch.Enabled {
		if es := outputs.NewElasticSearch(v, cfg.Outputs.Elasticsearch); es != nil {
			output = append(output, es)
		}
	}
	return outputs.NewTee(output)
}
