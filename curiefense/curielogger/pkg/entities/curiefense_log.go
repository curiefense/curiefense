package entities

type CuriefenseLog struct {
	Timestamp string `json:"timestamp"`

	Blocked     bool                   `json:"blocked"`
	BlockReason map[string]interface{} `json:"block_reason"`
	Tags        []string               `json:"tags"`

	RXTimers RXTimer `json:"rx_timers"`
	TXTimers TXTimer `json:"tx_timers"`

	Upstream   Upstream   `json:"upstream"`
	Downstream Downstream `json:"downstream"`

	TLS      TLS      `json:"tls"`
	Request  Request  `json:"request"`
	Response Response `json:"response"`
	Metadata Metadata `json:"metadata"`
}
