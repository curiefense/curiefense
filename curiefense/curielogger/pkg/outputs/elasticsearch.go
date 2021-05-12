package outputs

import (
	"bytes"
	_ "embed"
	"encoding/json"
	"fmt"
	"html/template"
	"io"
	"io/ioutil"
	"mime/multipart"
	"net/http"
	"strings"
	"time"

	"github.com/elastic/go-elasticsearch/v7"
	"github.com/elastic/go-elasticsearch/v7/esapi"
	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
)

const (
	ELASTICSEARCH_URL   = `ELASTICSEARCH_URL`
	ACCESSLOG_ES_PREFIX = "curieaccesslog"
)

type EmbeddedResource struct {
	Filename string
	Contents string
	Length   int
}

//go:embed es_index_template.json
var es_index_template string

//go:embed ilm_policy.json
var ilm_policy string

//go:embed index_settings.json
var index_settings string

//go:embed dashboard.ndjson
var dashboard string

type ElasticSearch struct {
	client *elasticsearch.Client
	cfg    ElasticsearchConfig
}

type ElasticsearchConfig struct {
	Enabled            bool   `mapstructure:"enabled"`
	Url                string `mapstructure:"url"`
	KibanaUrl          string `mapstructure:"kibana_url"`
	Initialize         bool   `mapstructure:"initialize"`
	Overwrite          bool   `mapstructure:"overwrite"`
	AccessLogIndexName string `mapstructure:"accesslog_index_name"`
	UseDataStream      bool   `mapstructure:"use_data_stream"`
	ILMPolicy          string `mapstructure:"ilm_policy"`
}

func NewElasticSearch(v *viper.Viper, cfg ElasticsearchConfig) *ElasticSearch {
	url := v.GetString(ELASTICSEARCH_URL)
	c, err := elasticsearch.NewClient(elasticsearch.Config{
		Addresses: strings.Split(url, `,`),
	})
	if err != nil {
		log.Error(err)
		return nil
	}
	log.Info(`initialized es`)
	es := &ElasticSearch{client: c, cfg: cfg}
	es.ConfigureEs()
	return es
}

func (es *ElasticSearch) Write(p []byte) (n int, err error) {
	r, err := es.client.Index(
		"curieaccesslog",
		strings.NewReader(string(p)),
		es.client.Index.WithRefresh("true"),
		es.client.Index.WithPretty(),
		es.client.Index.WithFilterPath("result", "_id"),
	)
	if err != nil {
		return 0, err
	}
	return len(p), r.Body.Close()
}

func (es *ElasticSearch) Close() error {
	return nil
}

func (es *ElasticSearch) ConfigureKibana() {

	var body, ktpl bytes.Buffer
	var fw io.Writer
	var err error
	gTpl := template.Must(template.New("it").Parse(dashboard))
	gTpl.Execute(&ktpl, es.cfg)

	mwriter := multipart.NewWriter(&body)
	if fw, err = mwriter.CreateFormFile("file", "dashboard.ndjson"); err != nil {
		log.Error("Error creating writer: %v", err)
		return
	}
	if _, err := io.Copy(fw, bytes.NewReader(ktpl.Bytes())); err != nil {
		log.Error("Error with io.Copy: %v", err)
		return
	}
	mwriter.Close()

	log.Debug("configuring kibana")
	kbUrl := fmt.Sprintf("%s/api/saved_objects/_import?overwrite=true", es.cfg.KibanaUrl)

	client := http.Client{
		Timeout: 5 * time.Second,
	}

	bReader := bytes.NewReader(body.Bytes())
	req, err := http.NewRequest("POST", kbUrl, bReader)
	if err != nil {
		return
	}

	req.Header.Set("Content-Type", mwriter.FormDataContentType())
	req.Header.Set("kbn-xsrf", "true")

	for i := 0; i < 60; i++ {
		rst, err := client.Do(req)
		if rst != nil {
			if rst.StatusCode == 200 {
				log.Debugf("kibana dashboard imported %s", kbUrl)
				break
			}

			if rst.StatusCode == 409 {
				log.Debugf("kibana index pattern already exists %s", kbUrl)
				break
			}
		}

		log.Errorf("kibana index pattern creation failed (retrying in 5s) %s %v %v %v", kbUrl, err, req, rst)
		time.Sleep(5 * time.Second)
		bReader.Seek(0, 0)
	}
}

func (es *ElasticSearch) ConfigureEs() {
	log.Warn("The Elasticsearch output is deprecated and will be removed in the 1.5.0 release. More on the reasoning and discussion here: https://github.com/curiefense/curiefense/issues/317")

	if !es.cfg.Initialize {
		return
	}
	var res *esapi.Response
	var err error
	for i := 0; i < 60; i++ {
		res, err = es.client.ILM.GetLifecycle()
		if err != nil {
			log.Errorf("There was an error while querying the ILM Policies %v", err)
			time.Sleep(time.Second * 5)
			continue
		}
		break
	}

	var ilm map[string]json.RawMessage
	if err := json.NewDecoder(res.Body).Decode(&ilm); err != nil {
		log.Errorf("There was an error while reading the ILM Policies %v", err)
		return
	}

	_, exists := ilm[es.cfg.AccessLogIndexName]
	if es.cfg.Overwrite || !exists {
		log.Debugf("creating / overwriting elasticsearch ilm policy %s for %s\n", es.cfg.AccessLogIndexName, es.cfg.Url)

		policy := es.cfg.ILMPolicy
		if policy == "" {
			var iTpl bytes.Buffer
			gTpl := template.Must(template.New("it").Parse(ilm_policy))
			gTpl.Execute(&iTpl, es.cfg)
			policy = string(iTpl.Bytes())
		}

		body := es.client.ILM.PutLifecycle.WithBody(strings.NewReader(policy))
		resp, err := es.client.ILM.PutLifecycle(es.cfg.AccessLogIndexName, body)
		if err != nil || resp.IsError() {
			var body []byte
			if resp != nil {
				defer resp.Body.Close()
				body, _ = ioutil.ReadAll(resp.Body)
			}
			log.Printf("[ERROR] index template creation failed %v %v %v", err, resp, string(body))
		}
	}

	// Create the Index Template
	//
	// This is how the mapping, ILM policies, and rollover aliases are assigned to
	// the indices or datastreams. There should always be an index template.
	//
	// TODO: Version the index template, as we may have to change the index mapping
	// in the future. Elastic's beats handle this in a decent way, look them up before
	// working on this task.
	tplExists, err := es.client.Indices.ExistsIndexTemplate(es.cfg.AccessLogIndexName)

	if err != nil {
		log.Error("there was an error while querying the template %v", err)
		return
	}

	if es.cfg.Overwrite || tplExists.IsError() {
		log.Printf("[DEBUG] creating / overwriting elasticsearch index template %s for %s\n", ACCESSLOG_ES_PREFIX, es.cfg.Url)
		var iTpl bytes.Buffer
		gTpl := template.Must(template.New("it").Parse(es_index_template))
		gTpl.Execute(&iTpl, es.cfg)

		resp, err := es.client.Indices.PutIndexTemplate(es.cfg.AccessLogIndexName, bytes.NewReader(iTpl.Bytes()))
		if err != nil || resp.IsError() {
			log.Errorf("[ERROR] index template creation failed %v %v", err, resp)
		}
	}

	// Data streams take care of creating the initila index, assigning an ILM policy
	// to it, and all the internal management. The index will be `hidden` and prefixed
	// with `.ds` so, in kibana, it is necessary to flag the "show hidden indeces" option.
	//
	// For non data stream configs, we have to create the initial index to make sure the
	// alias is assigned, the policy is attached to the index, etc.
	if !es.cfg.UseDataStream {
		log.Debugf("[DEBUG] data streams disabled: creating initial index")

		indexName := fmt.Sprintf("%s-000001", es.cfg.AccessLogIndexName)
		iExists, err := es.client.Indices.Exists([]string{indexName})

		if err != nil {
			log.Error("[ERROR] there was an error while querying the template %v", err)
			return
		}

		if !iExists.IsError() {
			log.Debugf("[DEBUG] elasticsearch index %s exists: doing noting", indexName)
			return
		}

		var iTpl bytes.Buffer
		gTpl := template.Must(template.New("it").Parse(index_settings))
		gTpl.Execute(&iTpl, es.cfg)

		resp, err := es.client.Indices.Create(indexName, es.client.Indices.Create.WithBody(bytes.NewReader(iTpl.Bytes())))
		if err != nil || resp.IsError() {
			log.Errorf("[ERROR] index template creation failed %v %v", err, resp)
			return
		}
	}

	// Attempt to configure Kibana's index patterns
	// and dashboards in the background.
	go es.ConfigureKibana()

}
