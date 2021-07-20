package main

import (
	"encoding/json"
	"strings"

	pkg "github.com/curiefense/curiefense/curielogger/pkg"
	"github.com/curiefense/curiefense/curielogger/pkg/entities"
	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
	syslog "gopkg.in/mcuadros/go-syslog.v2"
)

type syslogServer struct {
	logger  *pkg.LogSender
	channel syslog.LogPartsChannel
}

func newSyslogSrv(sender *pkg.LogSender) *syslogServer {
	channel := make(syslog.LogPartsChannel)
	return &syslogServer{logger: sender, channel: channel}
}

func syslogInit(srv *syslogServer, v *viper.Viper) {
	handler := syslog.NewChannelHandler(srv.channel)

	server := syslog.NewServer()
	//server.SetFormat(syslog.Automatic)
	server.SetFormat(syslog.RFC3164)
	server.SetHandler(handler)
	log.Infof("syslog server listening on 9514")
	server.ListenTCP("0.0.0.0:9514")

	go func(channel syslog.LogPartsChannel) {
		for logParts := range srv.channel {
			log.Debugf("PARTS %v", logParts)
			var cfLog entities.CuriefenseLog
			if !strings.HasPrefix(logParts["content"].(string), "nginx: ") {
				continue
			}
			content := strings.TrimPrefix(logParts["content"].(string), "nginx: ")
			log.Debugf("CONTENT %v", content)
			err := json.Unmarshal([]byte(content), &cfLog)
			if err != nil {
				log.Errorf("Error occurred during unmarshalling. Error: %s", err.Error())
			}
			log.Debugf("%v", cfLog)
			entry := entities.LogEntry{CfLog: cfLog}
			srv.logger.Write(&entry)
		}
	}(srv.channel)

	server.Boot()
	go server.Wait()
}
