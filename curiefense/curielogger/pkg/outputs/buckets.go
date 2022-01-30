package outputs

import (
	"bytes"
	"context"
	"io"
	"sync"
	"text/template"
	"time"

	"compress/gzip"

	"github.com/google/uuid"
	jsoniter "github.com/json-iterator/go"
	"github.com/pierrec/lz4"
	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
	"go.uber.org/atomic"
	"gocloud.dev/blob"
	_ "gocloud.dev/blob/azureblob"
	_ "gocloud.dev/blob/fileblob"
	_ "gocloud.dev/blob/gcsblob"
	_ "gocloud.dev/blob/s3blob"

	"github.com/curiefense/curiefense/curielogger/pkg/entities"

	pwriter "github.com/xitongsys/parquet-go/writer"
)

type Bucket struct {
	bucket, prefix string
	config         BucketConfig
	storageClient  *blob.Bucket
	w              *io.PipeWriter
	writeCancel    context.CancelFunc
	size           *atomic.Int64

	closed *atomic.Bool
	wg     *sync.WaitGroup
	lock   *sync.Mutex

	parquetWriter *pwriter.ParquetWriter
}

type BucketConfig struct {
	Enabled      bool   `mapstructure:"enabled"`
	URL          string `mapstructure:"url"`
	Prefix       string `mapstructure:"prefix"`
	Format       string `mapstructure:"format"`
	PathTemplate string `mapstructure:"path"`
	Compression  string `mapstructure:"compression"`
	FlushSeconds int    `mapstructure:"flush_seconds"`
}

type templateData struct {
	Time        time.Time
	Format      string
	Compression string
	Filename    string
}

func NewBucket(v *viper.Viper, cfg BucketConfig) *Bucket {
	if cfg.Format == "" {
		cfg.Format = "json"
	}

	if cfg.PathTemplate == "" {
		cfg.PathTemplate = `{{ .Time.Format "2006-01-02" }}/{{ .Time.Format "15" }}/{{ .Filename }}.{{ .Format }}.{{ .Compression }}`
	}

	log.Debugf("%s format in use for bucket output", cfg.Format)

	g := &Bucket{
		bucket: cfg.URL,
		prefix: cfg.Prefix,
		config: cfg,
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
		if g.parquetWriter != nil {
			err := g.parquetWriter.WriteStop()
			if err != nil {
				log.Error(err)
			}
		}

		if g.w != nil {
			g.w.Close()
		}
	}

	t := time.Now()
	ctx, cancel := context.WithCancel(context.Background())
	g.writeCancel = cancel

	var contentType string
	switch fmt := g.config.Format; fmt {
	case "json":
		contentType = "application/json"
	default:
		contentType = "application/octet-stream"
	}

	var path bytes.Buffer
	tmpl, err := template.New("path").Parse(g.config.PathTemplate)

	data := templateData{
		Filename:    uuid.New().String(),
		Format:      g.config.Format,
		Compression: g.config.Compression,
		Time:        t,
	}

	if err := tmpl.Execute(&path, data); err != nil {
		log.Error(err)
		return
	}

	log.Debug(path.String())
	w, err := g.storageClient.NewWriter(ctx, path.String(), &blob.WriterOptions{
		ContentEncoding: g.config.Compression,
		ContentType:     contentType,
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

		switch g.config.Compression {
		case "gzip":
			gzw := gzip.NewWriter(w)
			defer gzw.Close()
			io.Copy(gzw, pr)
		case "lz4":
			lz4w := lz4.NewWriter(w)
			defer lz4w.Close()
			io.Copy(lz4w, pr)
		default:
			io.Copy(w, pr)
		}
	}()
	g.w = pw

	if g.config.Format == "parquet" {
		parquetWriter, err := pwriter.NewParquetWriterFromWriter(pw, new(entities.CuriefenseLog), 1)
		if err != nil {
			log.Error(err)
			return
		}
		g.parquetWriter = parquetWriter
	}
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

func (g *Bucket) Write(lg entities.CuriefenseLog) (err error) {
	g.size.Inc()
	switch fmt := g.config.Format; fmt {
	case "parquet":
		err = g.parquetWriter.Write(lg)
	default:
		b, _ := jsoniter.Marshal(lg)
		rst := append(b, []byte("\n")...)
		_, err = g.w.Write(rst)
	}
	return err
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
