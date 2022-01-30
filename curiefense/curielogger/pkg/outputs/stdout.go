package outputs

import (
	"os"

	"github.com/curiefense/curiefense/curielogger/pkg/entities"
	jsoniter "github.com/json-iterator/go"
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

func (g *Stdout) Write(log entities.CuriefenseLog) error {
	b, _ := jsoniter.Marshal(log)
	rst := append(b, []byte("\n")...)
	_, err := os.Stdout.Write(rst)
	return err
}

func (b *Stdout) Close() error {
	return nil
}
