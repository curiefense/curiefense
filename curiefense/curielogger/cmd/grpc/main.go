package main

import (
	"context"
	"fmt"
	"github.com/curiefense/curiefense/curielogger/pkg"
	"github.com/curiefense/curiefense/curielogger/pkg/outputs"
	als "github.com/envoyproxy/go-control-plane/envoy/service/accesslog/v3"
	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
	"go.uber.org/fx"
	"google.golang.org/grpc"
	"net"
	"os"
)

const (
	GRPC_LISTENER = `CURIELOGGER_GRPC_LISTEN`
)

func main() {
	log.SetOutput(os.Stderr)
	lvl, err := log.ParseLevel(os.Getenv(`LOG_LEVEL`))
	if err != nil {
		lvl = log.WarnLevel
	}
	log.SetLevel(lvl)

	app := fx.New(
		fx.NopLogger,
		fx.Provide(
			pkg.NewConfig,
			outputs.InitOutputs,
			pkg.NewMetrics,
			pkg.NewLogSender,
			newGrpcSrv,
		),
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

	log.Info("GRPC server listening on %v", port)
	als.RegisterAccessLogServiceServer(s, srv)
	if err := s.Serve(sock); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
