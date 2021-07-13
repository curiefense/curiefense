package main

import (
	"context"
	"fmt"
	"net"
	"os"

	pkg "github.com/curiefense/curiefense/curielogger/pkg"
	als "github.com/envoyproxy/go-control-plane/envoy/service/accesslog/v3"
	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
	"go.uber.org/fx"
	"google.golang.org/grpc"
)

const (
	GRPC_LISTENER = `CURIELOGGER_GRPC_LISTEN`
)

func main() {
	log.SetOutput(os.Stderr)
	log.SetLevel(log.WarnLevel)

	app := fx.New(
		fx.NopLogger,
		fx.Provide(
			pkg.NewConfig,
			pkg.InitOutputs,
			pkg.NewMetrics,
			pkg.NewLogSender,
			newSyslogSrv,
			newGrpcSrv,
		),
		fx.Invoke(syslogInit),
		fx.Invoke(grpcInit),
	)
	if err := app.Start(context.Background()); err != nil {
		panic(err)
	}
}

func grpcInit(srv *grpcServer, v *viper.Viper) {
	port := v.GetString(GRPC_LISTENER)
	if port == `` {
		port = `9001`
	}
	sock, err := net.Listen("tcp", fmt.Sprintf(":%s", port))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	s := grpc.NewServer()

	log.Infof("GRPC server listening on %v", port)
	als.RegisterAccessLogServiceServer(s, srv)
	if err := s.Serve(sock); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
