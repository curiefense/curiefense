package pkg

import (
	"github.com/curiefense/curiefense/curielogger/pkg/entities"
	jsoniter "github.com/json-iterator/go"
	"github.com/spf13/viper"
	"go.uber.org/atomic"
	"io"
)

type LogSender struct {
	output  io.WriteCloser
	metrics *Metrics

	closed *atomic.Bool
}

func NewLogSender(v *viper.Viper, output io.WriteCloser, metrics *Metrics) *LogSender {
	return &LogSender{output: output, metrics: metrics, closed: atomic.NewBool(false)}
}

func (ls *LogSender) Write(l *entities.LogEntry) error {
	ls.metrics.add(l)
	b, _ := jsoniter.Marshal(l.CfLog)
	_, err := ls.output.Write(b)
	return err
}

func (ls *LogSender) Close() error {
	ls.closed.Store(true)
	return ls.output.Close()
}

func (ls *LogSender) Closed() bool {
	return ls.closed.Load()
}
