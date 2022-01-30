package outputs

import (
	"sync"

	"github.com/curiefense/curiefense/curielogger/pkg/entities"
	"go.uber.org/atomic"
)

type LogCloser interface {
	Write(log entities.CuriefenseLog) error
	Close() error
}

type Tee struct {
	fanOuts []LogCloser
	closed  *atomic.Bool
}

func NewTee(drivers []LogCloser) LogCloser {
	return &Tee{fanOuts: drivers, closed: atomic.NewBool(false)}
}

func (b *Tee) Write(log entities.CuriefenseLog) (err error) {
	if b.closed.Load() {
		return nil
	}
	wg := sync.WaitGroup{}
	wg.Add(len(b.fanOuts))
	for _, d := range b.fanOuts {
		go func(w LogCloser) {
			defer wg.Done()
			if e := w.Write(log); e != nil {
				err = e
			}
		}(d)
	}
	wg.Wait()
	return err
}

func (b *Tee) Close() error {
	b.closed.Store(true)
	var err error
	for _, d := range b.fanOuts {
		if e := d.Close(); e != nil {
			err = e
		}
	}
	return err
}
