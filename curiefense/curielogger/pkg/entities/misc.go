package entities

type RXTimer struct {
	FirstUpstreamByte float64 `parquet:"name=firstupstreambyte, type=FLOAT" json:"firstupstreambyte,string"`
	LastUpstreamByte  float64 `parquet:"name=lastupstreambyte, type=FLOAT" json:"lastupstreambyte,string"`
	LastByte          float64 `parquet:"name=lastbyte, type=FLOAT" json:"lastbyte"`
}

type TXTimer struct {
	FirstUpstreamByte   float64 `parquet:"name=firstupstreambyte, type=FLOAT" json:"firstupstreambyte,string"`
	LastUpstreamByte    float64 `parquet:"name=lastupstreambyte, type=FLOAT" json:"lastupstreambyte,string"`
	FirstDownstreamByte float64 `parquet:"name=firstdownstreambyte, type=FLOAT" json:"firstdownstreambyte,string"`
	LastDownstreamByte  float64 `parquet:"name=lastdownstreambyte, type=FLOAT" json:"lastdownstreambyte,string"`
}

type Downstream struct {
	ConnectionTermination   bool   `parquet:"name=connectiontermination, type=BOOLEAN" json:"connectiontermination"`
	DirectRemoteAddress     string `parquet:"name=directremoteaddress, type=BYTE_ARRAY, convertedtype=UTF8" json:"directremoteaddress"`
	DirectRemoteAddressPort int32  `parquet:"name=directremoteaddressport, type=INT32, convertedtype=INT_32" json:"directremoteaddressport"`
	LocalAddress            string `parquet:"name=localaddress, type=BYTE_ARRAY, convertedtype=UTF8" json:"localaddress"`
	LocalAddressPort        int32  `parquet:"name=localaddressport, type=INT32, convertedtype=INT_32" json:"localaddressport"`
	ProtocolError           bool   `parquet:"name=protocolerror, type=BOOLEAN" json:"protocolerror"`
	RemoteAddress           string `parquet:"name=remoteaddress, type=BYTE_ARRAY, convertedtype=UTF8" json:"remoteaddress"`
	RemoteAddressPort       int32  `parquet:"name=remoteaddressport, type=INT32, convertedtype=INT_32" json:"remoteaddressport"`
}

type Upstream struct {
	ConnectionFailure      bool   `parquet:"name=connectionfailure, type=BOOLEAN" json:"connectionfailure"`
	ConnectionTermination  bool   `parquet:"name=connectiontermination, type=BOOLEAN" json:"connectiontermination"`
	Overflow               bool   `parquet:"name=overflow, type=BOOLEAN" json:"overflow"`
	RemoteReset            bool   `parquet:"name=remotereset, type=BOOLEAN" json:"remotereset"`
	RequestTimeout         bool   `parquet:"name=requesttimeout, type=BOOLEAN" json:"requesttimeout"`
	RetryLimitExceeded     bool   `parquet:"name=retrylimitexceeded, type=BOOLEAN" json:"retrylimitexceeded"`
	Cluster                string `parquet:"name=cluster, type=BYTE_ARRAY, convertedtype=UTF8" json:"cluster"`
	RemoteAddress          string `parquet:"name=remoteaddress, type=BYTE_ARRAY, convertedtype=UTF8" json:"remoteaddress,omitempty"`
	LocalAddress           string `parquet:"name=localaddress, type=BYTE_ARRAY, convertedtype=UTF8" json:"localaddress,omitempty"`
	TransportFailureReason string `parquet:"name=transportfailurereason, type=BYTE_ARRAY, convertedtype=UTF8" json:"transportfailurereason"`
	LocalAddressPort       int32  `parquet:"name=localaddressport, type=INT32, convertedtype=INT_32" json:"localaddressport,omitempty"`
	RemoteAddressPort      int32  `parquet:"name=remoteaddressport, type=INT32, convertedtype=INT_32" json:"remoteaddressport,omitempty"`
}

type CertificateData struct {
	Properties         string   `parquet:"name=properties, type=BYTE_ARRAY, convertedtype=UTF8" json:"properties"`
	PropertiesAltNames []string `parquet:"name=propertiesaltnames, type=MAP, convertedtype=LIST, valuetype=BYTE_ARRAY, valueconvertedtype=UTF8" json:"propertiesaltnames"`
}

type TLS struct {
	LocalCertificate CertificateData `parquet:"name=localcertificate, type=MAP" json:"localcertificate"`
	PeerCertificate  CertificateData `parquet:"name=peercertificate, type=MAP" json:"peercertificate"`
	CipherSuite      string          `parquet:"name=ciphersuite, type=BYTE_ARRAY, convertedtype=UTF8" json:"ciphersuite"`
	SessionId        string          `parquet:"name=sessionid, type=BYTE_ARRAY, convertedtype=UTF8" json:"sessionid"`
	SNIHostname      string          `parquet:"name=snihostname, type=BYTE_ARRAY, convertedtype=UTF8" json:"snihostname"`
	Version          string          `parquet:"name=version, type=BYTE_ARRAY, convertedtype=UTF8" json:"version"`
}

type NameValue struct {
	Name  string `parquet:"name=name, type=BYTE_ARRAY, convertedtype=UTF8" json:"name"`
	Value string `parquet:"name=value, type=BYTE_ARRAY, convertedtype=UTF8" json:"value"`
}

type RequestAttributes struct {
	ipnum  uint32 `json:"ipnum,omitempty"`
	IP     string `parquet:"name=ip, type=BYTE_ARRAY, convertedtype=UTF8" json:"ip,omitempty"`
	Query  string `parquet:"name=query, type=BYTE_ARRAY, convertedtype=UTF8" json:"query,omitempty"`
	URI    string `parquet:"name=uri, type=BYTE_ARRAY, convertedtype=UTF8" json:"uri,omitempty"`
	Path   string `parquet:"name=path, type=BYTE_ARRAY, convertedtype=UTF8" json:"path,omitempty"`
	Method string `parquet:"name=method, type=BYTE_ARRAY, convertedtype=UTF8" json:"method,omitempty"`
	Authority string `parquet:"name=authority, type=BYTE_ARRAY, convertedtype=UTF8" json:"authority,omitempty"`
}

type Request struct {
	RequestId    string            `parquet:"name=requestid, type=BYTE_ARRAY, convertedtype=UTF8" json:"requestid"`
	Scheme       string            `parquet:"name=scheme, type=BYTE_ARRAY, convertedtype=UTF8" json:"scheme"`
	BodyBytes    int64             `parquet:"name=bodybytes, type=INT64, convertedtype=INT_64" json:"bodybytes"`
	HeadersBytes int64             `parquet:"name=headersbytes, type=INT64, convertedtype=INT_64" json:"headersbytes"`
	Headers      map[string]string `parquet:"name=headers, type=MAP, convertedtype=MAP, keytype=BYTE_ARRAY, keyconvertedtype=UTF8, valuetype=BYTE_ARRAY, valueconvertedtype=UTF8" json:"headers"`
	Cookies      map[string]string `parquet:"name=cookies, type=MAP, convertedtype=MAP, keytype=BYTE_ARRAY, keyconvertedtype=UTF8, valuetype=BYTE_ARRAY, convertedtype=UTF8" json:"cookies"`
	Arguments    map[string]string `parquet:"name=arguments, type=MAP, convertedtype=MAP, keytype=BYTE_ARRAY, keyconvertedtype=UTF8, valuetype=BYTE_ARRAY, convertedtype=UTF8" json:"arguments"`
	Geo          Geo               `parquet:"name=geo, type=MAP, convertedtype=MAP, keytype=BYTE_ARRAY, keyconvertedtype=UTF8, valuetype=BYTE_ARRAY, convertedtype=UTF8" json:"geo"`
	Attributes   RequestAttributes `parquet:"name=attributes, type=MAP" json:"attributes"`
}

type Geo struct {
	Company   string             `parquet:"name=company, type=BYTE_ARRAY, convertedtype=UTF8" json:"company"`
	City      map[string]string  `parquet:"name=city, type=MAP, convertedtype=MAP, keytype=BYTE_ARRAY, keyconvertedtype=UTF8, valuetype=BYTE_ARRAY, convertedtype=UTF8" json:"city"`
	Country   map[string]string  `parquet:"name=country, type=MAP, convertedtype=MAP, keytype=BYTE_ARRAY, keyconvertedtype=UTF8, valuetype=BYTE_ARRAY, convertedtype=UTF8" json:"country"`
	Continent map[string]string  `parquet:"name=continent, type=MAP, convertedtype=MAP, keytype=BYTE_ARRAY, keyconvertedtype=UTF8, valuetype=BYTE_ARRAY, convertedtype=UTF8" json:"continent"`
	Location  map[string]float32 `parquet:"name=location, type=MAP, convertedtype=MAP, keytype=BYTE_ARRAY, keyconvertedtype=UTF8, valuetype=FLOAT, convertedtype=FLOAT_32" json:"location"`
}

type Response struct {
	HeadersBytes int64             `parquet:"name=headersbytes, type=INT64, convertedtype=INT_64" json:"headersbytes"`
	BodyBytes    int64             `parquet:"name=bodybytes, type=INT64, convertedtype=INT_64" json:"bodybytes"`
	Code         int               `parquet:"name=code, type=INT32" json:"code"`
	CodeDetails  string            `parquet:"name=codedetails, type=BYTE_ARRAY, convertedtype=UTF8" json:"codedetails"`
	Headers      map[string]string `parquet:"name=headers, type=MAP, convertedtype=MAP, keytype=BYTE_ARRAY, keyconvertedtype=UTF8, valuetype=BYTE_ARRAY, valueconvertedtype=UTF8" json:"headers"`
	Trailers     map[string]string `parquet:"name=trailers, type=MAP, convertedtype=MAP, keytype=BYTE_ARRAY, keyconvertedtype=UTF8, valuetype=BYTE_ARRAY, valueconvertedtype=UTF8" json:"trailers"`
}

type Metadata struct {
	DelayInjected              bool    `parquet:"name=delayinjected, type=BOOLEAN" json:"delayinjected"`
	FailedLocalHealthCheck     bool    `parquet:"name=failedlocalhealthcheck, type=BOOLEAN" json:"failedlocalhealthcheck"`
	FaultInjected              bool    `parquet:"name=faultinjected, type=BOOLEAN" json:"faultinjected"`
	InvalidEnvoyRequestHeaders bool    `parquet:"name=invalidenvoyrequestheaders, type=BOOLEAN" json:"invalidenvoyrequestheaders"`
	LocalReset                 bool    `parquet:"name=localreset, type=BOOLEAN" json:"localreset"`
	NoHealthyUpstream          bool    `parquet:"name=nohealthyupstream, type=BOOLEAN" json:"nohealthyupstream"`
	NoRouteFound               bool    `parquet:"name=noroutefound, type=BOOLEAN" json:"noroutefound"`
	RateLimited                bool    `parquet:"name=ratelimited, type=BOOLEAN" json:"ratelimited"`
	RateLimitServiceError      bool    `parquet:"name=ratelimitserviceerror, type=BOOLEAN" json:"ratelimitserviceerror"`
	StreamIdleTimeout          bool    `parquet:"name=streamidletimeout, type=BOOLEAN" json:"streamidletimeout"`
	SampleRate                 float64 `parquet:"name=samplerate, type=FLOAT" json:"samplerate"`
	RouteName                  string  `parquet:"name=routename, type=BYTE_ARRAY, convertedtype=UTF8" json:"routename"`
	UnauthorizedDetails        string  `parquet:"name=unauthorizeddetails, type=BYTE_ARRAY, convertedtype=UTF8" json:"unauthorizeddetails"`
}
