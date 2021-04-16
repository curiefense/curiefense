package outputs

import (
	"fmt"
	"net/http"
	neturl "net/url"

	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
)

const (
	CURIELOGGER_FLUENTD_URL = `CURIELOGGER_FLUENTD_URL`
)

type FluentD struct {
	url string
}

func NewFluentD(v *viper.Viper) *FluentD {
	log.Info(`initialized fluentd`)
	log.Warn("The FluentD output is deprecated and will be removed in the 1.5.0 release. More on the reasoning and discussion here: https://github.com/curiefense/curiefense/issues/317")
	return &FluentD{url: fmt.Sprintf("%scuriefense.log", v.GetString(CURIELOGGER_FLUENTD_URL))}
}

func (b *FluentD) Write(p []byte) (n int, err error) {
	r, err := http.PostForm(b.url, neturl.Values{"json": {string(p)}})
	if err != nil {
		return 0, err
	}
	return len(p), r.Body.Close()
}

func (b *FluentD) Close() error {
	return nil
}
