package entities

type CuriefenseLog struct {
	Timestamp string `parquet:"name=timestamp, type=BYTE_ARRAY, convertedtype=UTF8" json:"timestamp"`

	Blocked     bool                   `parquet:"name=blocked, type=BOOLEAN" json:"blocked"`
	BlockReason map[string]interface{} `parquet:"name=blocked_reason, type=MAP, convertedtype=MAP, keytype=BYTE_ARRAY, keyconvertedtype=UTF8, valuetype=BYTE_ARRAY, valueconvertedtype=UTF8" json:"block_reason"`
	Tags        []string               `parquet:"name=tags, type=MAP, convertedtype=LIST, valuetype=BYTE_ARRAY, valueconvertedtype=UTF8" json:"tags"`

	RXTimers RXTimer `parquet:"name=rx_timers, type=MAP" json:"rx_timers"`
	TXTimers TXTimer `parquet:"name=tx_timers, type=MAP" json:"tx_timers"`

	Upstream   Upstream   `parquet:"name=upstream, type=MAP" json:"upstream"`
	Downstream Downstream `parquet:"name=downstream, type=MAP" json:"downstream"`

	TLS      TLS      `parquet:"name=tls, type=MAP" json:"tls"`
	Request  Request  `parquet:"name=request, type=MAP" json:"request"`
	Response Response `parquet:"name=response, type=MAP" json:"response"`
	Metadata Metadata `parquet:"name=metadata, type=MAP" json:"metadata"`
}
