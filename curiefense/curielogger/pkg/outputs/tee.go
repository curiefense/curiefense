package outputs

import (
	"go.uber.org/atomic"
	"io"
	"sync"
)

type Tee struct {
	fanOuts []io.WriteCloser
	closed  *atomic.Bool
}

func NewTee(drivers []io.WriteCloser) io.WriteCloser {
	return &Tee{fanOuts: drivers, closed: atomic.NewBool(false)}
}

func (b *Tee) Write(p []byte) (n int, err error) {
	if b.closed.Load() {
		return 0, nil
	}
	wg := sync.WaitGroup{}
	wg.Add(len(b.fanOuts))
	for _, d := range b.fanOuts {
		go func(w io.WriteCloser) {
			defer wg.Done()
			if _, e := w.Write(p); e != nil {
				err = e
			}
		}(d)
	}
	wg.Wait()
	return len(p), err
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
