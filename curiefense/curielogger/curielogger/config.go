package main

import (
	"strings"

	"github.com/spf13/viper"
	"gopkg.in/dealancer/validate.v2"
)

type Config struct {
	LogLevel        string        `mapstructure:"log_level" validate:"one_of=info,debug,error"`
	ChannelCapacity int           `mapstructure:"channel_capacity"`
	Outputs         OutputsConfig `mapstructure:"outputs,omitempty"`
}

type OutputsConfig struct {
	Logstash LogstashConfig `mapstructure:"logstash,omitempty"`
}

func LoadConfig() (config Config, err error) {
	viper.AutomaticEnv()
	viper.AddConfigPath(".")
	viper.AddConfigPath("/etc/curielogger/")
	viper.SetConfigName("curielogger")
	viper.SetConfigType("yaml")
	viper.SetEnvPrefix("CURIELOGGER")
	viper.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))

	err = viper.ReadInConfig()
	if err != nil {
		return
	}

	err = viper.Unmarshal(&config)
	if err != nil {
		return
	}

	err = validate.Validate(&config)
	return
}
