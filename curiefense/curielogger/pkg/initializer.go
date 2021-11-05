package pkg

import (
	"io"

	"github.com/spf13/viper"

	"github.com/curiefense/curiefense/curielogger/pkg/outputs"
)

const (
	STDOUT_ENABLED = `CURIELOGGER_OUTPUTS_STDOUT_ENABLED`
	BUCKET_ENABLED = `CURIELOGGER_OUTPUTS_BUCKET_ENABLED`
)

type OutputsConfig struct {
	Stdout outputs.StdoutConfig `mapstructure:"stdout,omitempty"`
	Bucket outputs.BucketConfig `mapstructure:"bucket,omitempty"`
}

func InitOutputs(v *viper.Viper, cfg Config) io.WriteCloser {
	output := make([]io.WriteCloser, 0)
	if v.GetBool(STDOUT_ENABLED) || cfg.Outputs.Stdout.Enabled {
		output = append(output, outputs.NewStdout(v))
	}

	if v.GetBool(BUCKET_ENABLED) || cfg.Outputs.Bucket.Enabled {
		output = append(output, outputs.NewBucket(v, cfg.Outputs.Bucket))
	}

	return outputs.NewTee(output)
}
