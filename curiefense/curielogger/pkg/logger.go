package pkg

import (
	"github.com/curiefense/curiefense/curielogger/pkg/entities"
	"github.com/curiefense/curiefense/curielogger/pkg/outputs"
	"github.com/spf13/viper"
	"go.uber.org/atomic"
)

type LogSender struct {
	output  outputs.LogCloser
	metrics *Metrics

	closed *atomic.Bool
}

func NewLogSender(v *viper.Viper, output outputs.LogCloser, metrics *Metrics) *LogSender {
	return &LogSender{output: output, metrics: metrics, closed: atomic.NewBool(false)}
}

func (ls *LogSender) Write(l *entities.LogEntry) error {
	ls.metrics.add(l)
	return ls.output.Write(l.CfLog)
}

func (ls *LogSender) Close() error {
	ls.closed.Store(true)
	return ls.output.Close()
}

func (ls *LogSender) Closed() bool {
	return ls.closed.Load()
}
