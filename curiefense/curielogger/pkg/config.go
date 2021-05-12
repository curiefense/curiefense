package pkg

import (
	"strings"

	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
	"gopkg.in/dealancer/validate.v2"
)

type Config struct {
	LogLevel string        `mapstructure:"log_level" validate:"one_of=info,debug,error"`
	Outputs  OutputsConfig `mapstructure:"outputs,omitempty"`
}

func NewConfig() (*viper.Viper, Config) {
	v := viper.New()
	v.AutomaticEnv()
	viper.AddConfigPath(".")
	viper.AddConfigPath("./../")
	viper.AddConfigPath("/etc/curielogger/")
	viper.SetConfigName("curielogger")
	viper.SetConfigType("yaml")
	viper.SetEnvPrefix("CURIELOGGER")
	viper.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))

	var cfg Config
	err := viper.ReadInConfig()
	if err != nil {
		panic(err)
	}

	err = viper.Unmarshal(&cfg)
	if err != nil {
		panic(err)
	}

	err = validate.Validate(&cfg)

	lvl, err := log.ParseLevel(cfg.LogLevel)
	log.SetLevel(lvl)
	return v, cfg
}
