package outputs

import (
	"os"

	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
)

type StdoutConfig struct {
	Enabled bool `mapstructure:"enabled"`
}

type Stdout struct {
}

func NewStdout(v *viper.Viper) *Stdout {
	log.Info("stdout driver started")
	return &Stdout{}
}

func (b *Stdout) Write(p []byte) (n int, err error) {
	rst := append(p, []byte("\n")...)
	return os.Stdout.Write(rst)
}

func (b *Stdout) Close() error {
	return nil
}
