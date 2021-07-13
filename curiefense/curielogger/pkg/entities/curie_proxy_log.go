package entities

type CurieProxyLog struct {
	Headers     map[string]string      `json:"headers"`
	Cookies     map[string]string      `json:"cookies"`
	Geo         map[string]interface{} `json:"geo"`
	Arguments   map[string]string      `json:"arguments"`
	Attributes  RequestAttributes      `json:"attributes"`
	Blocked     bool                   `json:"blocked"`
	BlockReason map[string]interface{} `json:"block_reason"`
	Tags        []string               `json:"tags"`
}
