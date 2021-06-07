package entities

type RXTimer struct {
	FirstUpstreamByte float64 `json:"firstupstreambyte,string"`
	LastUpstreamByte  float64 `json:"lastupstreambyte,string"`
	LastByte          float64 `json:"lastbyte"`
}

type TXTimer struct {
	FirstUpstreamByte   float64 `json:"firstupstreambyte,string"`
	LastUpstreamByte    float64 `json:"lastupstreambyte,string"`
	FirstDownstreamByte float64 `json:"firstdownstreambyte,string"`
	LastDownstreamByte  float64 `json:"lastdownstreambyte,string"`
}

type Downstream struct {
	ConnectionTermination   bool   `json:"connectiontermination"`
	DirectRemoteAddress     string `json:"directremoteaddress"`
	DirectRemoteAddressPort uint32 `json:"directremoteaddressport"`
	LocalAddress            string `json:"localaddress"`
	LocalAddressPort        uint32 `json:"localaddressport"`
	ProtocolError           bool   `json:"protocolerror"`
	RemoteAddress           string `json:"remoteaddress"`
	RemoteAddressPort       uint32 `json:"remoteaddressport"`
}

type Upstream struct {
	Cluster                string `json:"cluster"`
	ConnectionFailure      bool   `json:"connectionfailure"`
	ConnectionTermination  bool   `json:"connectiontermination"`
	LocalAddress           string `json:"localaddress,omitempty"`
	LocalAddressPort       uint32 `json:"localaddressport,omitempty"`
	Overflow               bool   `json:"overflow"`
	RemoteAddress          string `json:"remoteaddress,omitempty"`
	RemoteAddressPort      uint32 `json:"remoteaddressport,omitempty"`
	RemoteReset            bool   `json:"remotereset"`
	RequestTimeout         bool   `json:"requesttimeout"`
	RetryLimitExceeded     bool   `json:"retrylimitexceeded"`
	TransportFailureReason string `json:"transportfailurereason"`
}

type CertificateData struct {
	Properties         string   `json:"properties"`
	PropertiesAltNames []string `json:"propertiesaltnames"`
}

type TLS struct {
	LocalCertificate CertificateData `json:"localcertificate"`
	PeerCertificate  CertificateData `json:"peercertificate"`
	CipherSuite      string          `json:"ciphersuite"`
	SessionId        string          `json:"sessionid"`
	SNIHostname      string          `json:"snihostname"`
	Version          string          `json:"version"`
}

type NameValue struct {
	Name  string `json:"name"`
	Value string `json:"value"`
}

type RequestAttributes struct {
	ipnum  uint32 `json:"ipnum,omitempty"`
	IP     string `json:"ip,omitempty"`
	Query  string `json:"query,omitempty"`
	URI    string `json:"uri,omitempty"`
	Path   string `json:"path,omitempty"`
	Method string `json:"method,omitempty"`
}

type Request struct {
	RequestId    string                 `json:"requestid"`
	Scheme       string                 `json:"scheme"`
	BodyBytes    uint64                 `json:"bodybytes"`
	HeadersBytes uint64                 `json:"headersbytes"`
	Headers      map[string]string      `json:"headers"`
	Cookies      map[string]string      `json:"cookies"`
	Arguments    map[string]string      `json:"arguments"`
	Geo          map[string]interface{} `json:"geo"`
	Attributes   RequestAttributes      `json:"attributes"`
}

type Response struct {
	BodyBytes    uint64            `json:"bodybytes"`
	Code         int               `json:"code"`
	CodeDetails  string            `json:"codedetails"`
	Headers      map[string]string `json:"headers"`
	HeadersBytes uint64            `json:"headersbytes"`
	Trailers     map[string]string `json:"trailers"`
}

type Metadata struct {
	DelayInjected              bool    `json:"delayinjected"`
	FailedLocalHealthCheck     bool    `json:"failedlocalhealthcheck"`
	FaultInjected              bool    `json:"faultinjected"`
	InvalidEnvoyRequestHeaders bool    `json:"invalidenvoyrequestheaders"`
	LocalReset                 bool    `json:"localreset"`
	NoHealthyUpstream          bool    `json:"nohealthyupstream"`
	NoRouteFound               bool    `json:"noroutefound"`
	RateLimited                bool    `json:"ratelimited"`
	RateLimitServiceError      bool    `json:"ratelimitserviceerror"`
	RouteName                  string  `json:"routename"`
	SampleRate                 float64 `json:"samplerate"`
	StreamIdleTimeout          bool    `json:"streamidletimeout"`
	UnauthorizedDetails        string  `json:"unauthorizeddetails"`
}
