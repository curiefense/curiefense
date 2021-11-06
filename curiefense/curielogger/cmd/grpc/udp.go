package main

import (
	"context"
	"encoding/json"
	"net"

	pkg "github.com/curiefense/curiefense/curielogger/pkg"
	"github.com/curiefense/curiefense/curielogger/pkg/entities"
	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
)

type udpServer struct {
	logger *pkg.LogSender
}

func newUDPSrv(sender *pkg.LogSender) *udpServer {
	return &udpServer{logger: sender}
}

// serve is capable of answering to a single client at a time
func udpInit(srv *udpServer, v *viper.Viper) {
	go serve(srv)
}

func serve(srv *udpServer) {
	ctx := context.Background()
	address := ":9002"

	log.Infof("UDP server listening on %v", address)
	pc, err := net.ListenPacket("udp", address)
	if err != nil {
		log.Errorf("failed to UDP listen on '%s' with '%v'", address, err)
		return
	}

	defer func() {
		if err := pc.Close(); err != nil {
			log.Errorf("failed to close packet connection with '%v'", err)
		}
	}()

	errChan := make(chan error, 1)
	doneChan := make(chan error, 1)
	// maxBufferSize specifies the size of the buffers that
	// are used to temporarily hold data from the UDP packets
	// that we receive.
	buffer := make([]byte, 4096)

	go func() {
		for {
			var cfLog entities.CuriefenseLog
			n, _, err := pc.ReadFrom(buffer)
			if err != nil {
				doneChan <- err
				return
			}

			b := buffer[:n]
			err = json.Unmarshal(b, &cfLog)
			//log.Debugf("address: '%+v' bytes: '%d' request: '%s'", addr, n, string(b))

			if err != nil {
				log.Error(err)
				continue
			}

			entry := entities.LogEntry{CfLog: cfLog}
			srv.logger.Write(&entry)
		}
	}()

	select {
	case <-ctx.Done():
		log.Infof("cancelled with '%v'", ctx.Err())
	case _ = <-errChan:
	}

	return
}
