package outputs

import (
	"context"
	"fmt"
	"io"
	"sync"
	"time"

	"github.com/google/uuid"
	"github.com/pierrec/lz4"
	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
	"go.uber.org/atomic"
	"gocloud.dev/blob"
	_ "gocloud.dev/blob/azureblob"
	_ "gocloud.dev/blob/fileblob"
	_ "gocloud.dev/blob/gcsblob"
	_ "gocloud.dev/blob/s3blob"
)

type Bucket struct {
	bucket, prefix string
	storageClient  *blob.Bucket
	w              *io.PipeWriter
	writeCancel    context.CancelFunc
	size           *atomic.Int64

	closed *atomic.Bool
	wg     *sync.WaitGroup
	lock   *sync.Mutex
}

type BucketConfig struct {
	Enabled      bool   `mapstructure:"enabled"`
	URL          string `mapstructure:"url"`
	Prefix       string `mapstructure:"prefix"`
	FlushSeconds int    `mapstructure:"flush_seconds"`
}

func NewBucket(v *viper.Viper, cfg BucketConfig) *Bucket {
	g := &Bucket{
		bucket: cfg.URL,
		prefix: cfg.Prefix,
		closed: atomic.NewBool(false),
		wg:     &sync.WaitGroup{},
		lock:   &sync.Mutex{},
		size:   atomic.NewInt64(0),
	}
	var err error
	g.storageClient, err = blob.OpenBucket(context.Background(), g.bucket)
	if err != nil {
		log.Error(err)
		return nil
	}
	g.rotateUploader()
	go g.flusher(time.Duration(cfg.FlushSeconds) * time.Second)

	log.Infof("initialized bucket export")
	return g
}

func (g *Bucket) rotateUploader() {
	g.lock.Lock()
	defer g.lock.Unlock()
	if g.closed.Load() {
		return
	}
	if g.size.Swap(0) == 0 {
		if g.writeCancel != nil {
			g.writeCancel()
		}
	} else {
		if g.w != nil {
			g.w.Close()
		}
	}
	t := time.Now()
	ctx, cancel := context.WithCancel(context.Background())
	g.writeCancel = cancel
	w, err := g.storageClient.NewWriter(ctx, fmt.Sprintf(`%s/created_date=%s/hour=%s/%s.json.lz4`, g.prefix, t.Format(`2006-01-02`), t.Format(`15`), uuid.New().String()), &blob.WriterOptions{
		ContentEncoding: "lz4",
		ContentType:     "application/json",
	})
	if err != nil {
		log.Error(err)
		return
	}
	pr, pw := io.Pipe()
	g.wg.Add(1)
	go func() {
		defer g.wg.Done()
		defer w.Close()
		gzw := lz4.NewWriter(w)
		defer gzw.Close()
		io.Copy(gzw, pr)
	}()
	g.w = pw
}

func (g *Bucket) flusher(duration time.Duration) {
	if duration.Seconds() < 1 {
		duration = time.Second
	}
	t := time.NewTicker(duration)
	for range t.C {
		g.rotateUploader()
	}
}

func (g *Bucket) Write(p []byte) (n int, err error) {
	g.size.Inc()
	rst := append(p, []byte("\n")...)
	return g.w.Write(rst)
}

func (g *Bucket) Close() error {
	g.closed.Store(true)
	defer g.wg.Wait()
	if g.size.Load() == 0 {
		g.writeCancel()
		return nil
	}
	g.w.Close()
	return nil
}
