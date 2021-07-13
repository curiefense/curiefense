package entities

import ald "github.com/envoyproxy/go-control-plane/envoy/data/accesslog/v3"

type LogEntry struct {
	FullEntry *ald.HTTPAccessLogEntry
	CfLog     CuriefenseLog
}
