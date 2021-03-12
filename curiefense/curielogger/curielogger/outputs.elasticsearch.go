package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"html/template"
	"io"
	"log"
	"mime/multipart"
	"net/http"
	"strings"
	"time"

	elasticsearch "github.com/elastic/go-elasticsearch/v7"
)

//  ___ _      _   ___ _____ ___ ___ ___ ___   _   ___  ___ _  _
// | __| |    /_\ / __|_   _|_ _/ __/ __| __| /_\ | _ \/ __| || |
// | _|| |__ / _ \\__ \ | |  | | (__\__ \ _| / _ \|   / (__| __ |
// |___|____/_/ \_\___/ |_| |___\___|___/___/_/ \_\_|_\\___|_||_|
// ELASTICSEARCH

const (
	METRICS_ES_PREFIX   = "curiemetrics"
	ACCESSLOG_ES_PREFIX = "curieaccesslog"
)

type ElasticsearchConfig struct {
	Enabled            bool   `mapstructure:"enabled"`
	Url                string `mapstructure:"url"`
	KibanaUrl          string `mapstructure:"kibana_url"`
	Initialize         bool   `mapstructure:"initialize"`
	Overwrite          bool   `mapstructure:"overwrite"`
	AccessLogIndexName string `mapstructure:"accesslog_index_name"`
	UseDataStream      bool   `mapstructure:"use_data_stream"`
}

type ElasticsearchLogger struct {
	logger
	client *elasticsearch.Client
	config ElasticsearchConfig
}

func (l ElasticsearchLogger) getESClient() *elasticsearch.Client {
	for l.client == nil {
		cfg := elasticsearch.Config{
			Addresses: []string{l.config.Url},
		}
		conn, err := elasticsearch.NewClient(cfg)
		if err == nil {
			l.client = conn
			log.Printf("[DEBUG] Connected to elasticsearch %v\n", l.config.Url)
			break
		}
		log.Printf("[ERROR] Could not connect to elasticsearch [%v]: %v\n", l.config.Url, err)
		time.Sleep(time.Second)
	}
	return l.client
}

func (l *ElasticsearchLogger) ConfigureKibana() {

	var body, ktpl bytes.Buffer
	var fw io.Writer
	var err error
	res, _ := getResource("files/kibana/dashboard.ndjson")
	gTpl := template.Must(template.New("it").Parse(string(res)))
	gTpl.Execute(&ktpl, l.config)

	mwriter := multipart.NewWriter(&body)
	if fw, err = mwriter.CreateFormFile("file", "dashboard.ndjson"); err != nil {
		log.Printf("[ERROR] Error creating writer: %v", err)
		return
	}
	if _, err := io.Copy(fw, bytes.NewReader(ktpl.Bytes())); err != nil {
		log.Printf("[ERROR] Error with io.Copy: %v", err)
		return
	}
	mwriter.Close()

	log.Printf("[DEBUG]: configuring kibana")
	kbUrl := fmt.Sprintf("%s/api/saved_objects/_import?overwrite=true", l.config.KibanaUrl)

	client := http.Client{
		Timeout: time.Duration(5 * time.Second),
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
				log.Printf("[DEBUG]: kibana dashboard imported %s", kbUrl)
				break
			}

			if rst.StatusCode == 409 {
				log.Printf("[DEBUG]: kibana index pattern already exists %s", kbUrl)
				break
			}
		}

		log.Printf("[ERROR]: kibana index pattern creation failed (retrying in 5s) %s %v %v %v", kbUrl, err, req, rst)
		time.Sleep(5 * time.Second)
		bReader.Seek(0, 0)
	}
}

func (l *ElasticsearchLogger) Configure(channel_capacity int) error {
	l.name = "Elasticsearch"
	ch := make(chan LogEntry, channel_capacity)
	l.channel = ch
	l.do_insert = l.InsertEntry

	if l.config.AccessLogIndexName == "" {
		l.config.AccessLogIndexName = ACCESSLOG_ES_PREFIX
	}

	if !l.config.Initialize {
		return nil
	}

	log.Printf("[DEBUG] Initializing Elasticsearch configs for %s\n", l.config.Url)
	client := l.getESClient()

	// Initialize ILM Policy
	res, err := client.ILM.GetLifecycle()

	if err != nil {
		log.Printf("[ERROR] There was an error while querying the ILM Policies %v", err)
		return err
	}

	var ilm map[string]json.RawMessage
	if err := json.NewDecoder(res.Body).Decode(&ilm); err != nil {
		return nil
	}

	_, exists := ilm[l.config.AccessLogIndexName]
	if l.config.Overwrite || !exists {
		log.Printf("[DEBUG] creating / overwriting elasticsearch ilm policy %s for %s\n", l.config.AccessLogIndexName, l.config.Url)

		var iTpl bytes.Buffer
		res, _ := getResource("files/kibana/ilm_policy.json")
		gTpl := template.Must(template.New("it").Parse(string(res)))
		gTpl.Execute(&iTpl, l.config)

		body := client.ILM.PutLifecycle.WithBody(bytes.NewReader(iTpl.Bytes()))
		resp, err := client.ILM.PutLifecycle(l.config.AccessLogIndexName, body)
		if err != nil || resp.IsError() {
			log.Printf("[ERROR] index template creation failed %v %v", err, resp)
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
	tplExists, err := client.Indices.ExistsIndexTemplate(l.config.AccessLogIndexName)

	if err != nil {
		log.Printf("[ERROR] there was an error while querying the template %v", err)
		return err
	}

	if l.config.Overwrite || tplExists.IsError() {
		log.Printf("[DEBUG] creating / overwriting elasticsearch index template %s for %s\n", ACCESSLOG_ES_PREFIX, l.config.Url)
		var iTpl bytes.Buffer
		res, _ := getResource("files/kibana/es_index_template.json")
		gTpl := template.Must(template.New("it").Parse(string(res)))
		gTpl.Execute(&iTpl, l.config)

		resp, err := client.Indices.PutIndexTemplate(l.config.AccessLogIndexName, bytes.NewReader(iTpl.Bytes()))
		if err != nil || resp.IsError() {
			log.Printf("[ERROR] index template creation failed %v %v", err, resp)
		}
	}

	// Data streams take care of creating the initila index, assigning an ILM policy
	// to it, and all the internal management. The index will be `hidden` and prefixed
	// with `.ds` so, in kibana, it is necessary to flag the "show hidden indeces" option.
	//
	// For non data stream configs, we have to create the initial index to make sure the
	// alias is assigned, the policy is attached to the index, etc.
	if !l.config.UseDataStream {
		log.Printf("[DEBUG] data streams disabled: creating initial index")

		indexName := fmt.Sprintf("%s-000001", l.config.AccessLogIndexName)
		iExists, err := client.Indices.Exists([]string{indexName})

		if err != nil {
			log.Printf("[ERROR] there was an error while querying the template %v", err)
			return err
		}

		if !iExists.IsError() {
			log.Printf("[DEBUG] elasticsearch index %s exists: doing noting", indexName)
			return nil
		}

		var iTpl bytes.Buffer
		res, _ := getResource("files/kibana/index_settings.json")
		gTpl := template.Must(template.New("it").Parse(string(res)))
		gTpl.Execute(&iTpl, l.config)

		resp, err := client.Indices.Create(indexName, client.Indices.Create.WithBody(bytes.NewReader(iTpl.Bytes())))
		if err != nil || resp.IsError() {
			log.Printf("[ERROR] index template creation failed %v %v", err, resp)
			return err
		}
	}

	// Attempt to configure Kibana's index patterns
	// and dashboards in the background.
	go l.ConfigureKibana()

	return nil
}

func (l *ElasticsearchLogger) InsertEntry(e LogEntry) bool {
	log.Printf("[DEBUG] ES insertion!")
	client := l.getESClient() // needed to ensure ES cnx is not closed and reopen it if needed
	j, err := json.Marshal(e.cfLog)
	if err == nil {
		client.Index(
			"log",
			strings.NewReader(string(j)),
			client.Index.WithRefresh("true"),
			client.Index.WithPretty(),
			client.Index.WithFilterPath("result", "_id"),
		)
	} else {
		log.Printf("[ERROR] Could not convert protobuf entry into json for ES insertion.")
		return false
	}
	return true
}
