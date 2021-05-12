package outputs

import (
	"bytes"
	"io"
)

type Stub struct {
	buff *bytes.Buffer
}

func NewStub() io.WriteCloser {
	return &Stub{buff: bytes.NewBuffer(nil)}
}

func (b *Stub) Write(p []byte) (n int, err error) {
	return b.buff.Write(p)
}

func (b *Stub) Close() error {
	return nil
}
