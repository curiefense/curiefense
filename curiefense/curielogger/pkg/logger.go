package pkg

import (
	"encoding/json"
	"github.com/curiefense/curiefense/curielogger/pkg/entities"
	"github.com/spf13/viper"
	"go.uber.org/atomic"
	"io"
)

type LogSender struct {
	exportDrivers io.WriteCloser
	encoder       *json.Encoder
	metrics       *Metrics

	closed *atomic.Bool
}

func NewLogSender(v *viper.Viper, drivers io.WriteCloser, metrics *Metrics) *LogSender {
	return &LogSender{exportDrivers: drivers, encoder: json.NewEncoder(drivers), metrics: metrics, closed: atomic.NewBool(false)}
}

func (ls *LogSender) Write(l *entities.LogEntry) error {
	l.CfLog.Tags = append(l.CfLog.Tags, "curieaccesslog")
	ls.metrics.add(l)
	b, _ := json.Marshal(l)
	_, err := ls.exportDrivers.Write(b)
	return err
}

func (ls *LogSender) Close() error {
	ls.closed.Store(true)
	return ls.exportDrivers.Close()
}

func (ls *LogSender) Closed() bool {
	return ls.closed.Load()
}
