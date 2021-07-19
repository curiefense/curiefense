local session_rust_nginx = {}
local cjson       = require "cjson"
local curiefense  = require "curiefense"
local grasshopper = require "grasshopper"
local utils       = require "lua.nativeutils"
local sfmt = string.format
local custom_response = utils.nginx_custom_response

function session_rust_nginx.inspect(handle)
    local ip_str = handle.var.remote_addr

    local headers = {}

    local rheaders, err = handle.req.get_headers()
    if err == "truncated" then
        handle.log(handle.ERR, "truncated headers: " .. err)
    end

    for k, v in pairs(rheaders) do
        headers[k] = v
    end

    handle.log(handle.INFO, cjson.encode(headers))

    handle.req.read_body()
    local body_content = handle.req.get_body_data()
    if body_content ~= nil then
        handle.ctx.body_len = body_content:len()
    else
        handle.ctx.body_len = 0
    end
    local meta = { path=handle.var.request_uri, method=handle.req.get_method(), authority=nil }

    -- the meta table contains the following elements:
    --   * path : the full request uri
    --   * method : the HTTP verb
    --   * authority : optionally, the HTTP2 authority field
    local response
    response, err = curiefense.inspect_request(
        meta, headers, body_content, ip_str, grasshopper
    )

    if err then
        handle.log(handle.ERR, sfmt("curiefense.inspect_request_map error %s", err))
    end

    if response then
        local response_table = cjson.decode(response)
        handle.ctx.response = response_table
        handle.log(handle.DEBUG, "decision: " .. response)
        utils.log_nginx_messages(handle, response_table["logs"])
        local request_map = response_table["request_map"]
        request_map.handle = handle
        if response_table["action"] == "custom_response" then
            custom_response(request_map, response_table["response"])
        end
    end
end

local function parse_ip_port(ipport)
    if ipport == nil then
        return nil, nil
    end
    local s, _ = string.find(ipport, ":")
    if s == nil then
      return ipport, nil
    else
      local port_part = string.sub(ipport,s+1)
      local host_part = string.sub(ipport, 1, s-1)
      return host_part, tonumber(port_part) or port_part
    end
end

-- log block stage processing
function session_rust_nginx.log(handle)
    local response = handle.ctx.response
    handle.ctx.response = nil
    local request_map = response.request_map

    local body_len = handle.ctx.body_len
    local req_len = handle.var.request_length

    local raw_status = handle.var.status
    local status = tonumber(raw_status) or raw_status
    local req = {
        tags=request_map["tags"],
        path=handle.var.uri,
        host=handle.var.host,
        -- TODO: authority
        -- authority= "34.66.199.37:30081",
        requestid=handle.var.request_id,
        method=handle.var.request_method,
        response={
          code=status,
          headers=handle.resp.get_headers(),
          codedetails="unknown",
          nxgrequestlength=handle.var.request_length
        },
        scheme=handle.var.scheme,
        metadata={},
        port=0,
    }

    if response.response ~= cjson.null then
        req.block_reason=response.response.reason
        req.blocked=response.response.block_mode
    else
        req.block_reason=nil
        req.blocked=false
    end

    local raw_server_port = handle.var.server_port
    local raw_remote_port = handle.var.remote_port
    local server_port = tonumber(raw_server_port) or raw_server_port
    local remote_port = tonumber(raw_remote_port) or raw_remote_port

    req.upstream = {}
    req.upstream.cluster = handle.var.proxy_host

    req.downstream = {
      localaddressport=server_port,
      remoteaddress=handle.var.remote_addr,
      localaddress=handle.var.server_addr,
      remoteaddressport=remote_port,
      directlocaladdress=handle.var.server_addr,
      directremoteaddressport=remote_port,
      directremoteaddress=handle.var.remote_addr,
    }

    -- handle upstream_addr with ports
    local u_host, u_port = parse_ip_port(handle.var.upstream_addr)
    if u_port == nil then
        u_port = tonumber(handle.var.proxy_port) or handle.var.proxy_port
    end
    req.upstream.remoteaddress = u_host
    req.upstream.remoteaddressport = u_port

    -- TLS: TODO, need to see the corresponding envoy input
    req.tls = {
          version=handle.var.ssl_protocol,
          snihostname=handle.var.ssl_server_name,
          fullciphersuite=handle.var.ssl_cipher,
          peercertificate={
            dn=handle.var.ssl_client_s_dn,
            properties=handle.var.ssl_client_s_dn,
            propertiesaltnames=nil
          },
          localcertificate=nil, -- no info from nginx :(
          sessionid=handle.var.ssl_session_id,
    }

    req.request = {
        originalpath="",
        geo=request_map["geo"],
        arguments=request_map["args"],
        headers=request_map["headers"],
        cookies=request_map["cookies"],
        -- TODO: are we currently including the length of the first line of the HTTP request?
        headersbytes=req_len - body_len,
        bodybytes=body_len
    }

    -- nginx variables:
    -- downstream is the client
    -- upstream is the proxied service

    -- !!! most of these variables are unset in the default configuration !!!
    -- connection_time: connection time in seconds with a milliseconds resolution (1.19.10)
    -- request_time: request processing time in seconds with a milliseconds resolution (1.3.9, 1.2.6);
   --     time elapsed since the first bytes were read from the client
    -- session_time: session duration in seconds with a milliseconds resolution
    -- upstream_header_time: keeps time spent on receiving the response header from the upstream server
    -- upstream_queue_time: keeps time the request spent in the upstream queue
    -- upstream_response_time: keeps time spent on receiving the response from the upstream server
    -- upstream_session_time: session duration in seconds with millisecond resolution
    -- upstream_first_byte_time: time to receive the first byte of data
    -- upstream_connect_time: keeps time spent on establishing a connection with the upstream server

    req.rx_timers = {
        -- Interval between the first downstream byte received and the last downstream byte received
        -- (i.e. time it takes to receive a request).
        lastbyte=nil,
        -- Interval between the first downstream byte received and the first upstream byte received
        -- (i.e. time it takes to start receiving a response).
        firstupstreambyte=nil,
        -- Interval between the first downstream byte received and the last upstream byte received
        -- (i.e. time it takes to receive a complete response).
        lastupstreambyte=nil,
    }
    req.tx_timers = {
        -- Interval between the first downstream byte received and the first upstream byte sent.
        firstupstreambyte=handle.var.upstream_first_byte_time,
        -- Interval between the first downstream byte received and the last upstream byte sent.
        lastupstreambyte=handle.var.request_time,
        -- Interval between the first downstream byte received and the first downstream byte sent.
        firstdownstreambyte=nil, -- TODO
        -- Interval between the first downstream byte received and the last downstream byte sent.
        lastdownstreambyte=nil
    }

    -- building the formatted timestamp (should it be added by logstash?)
    local tm = handle.utctime() -- format "yyyy-mm-dd hh:mm:ss"
    local fracpart = tostring(handle.now()%1):sub(2,10)
    local timestamp = tm:gsub(' ', 'T') .. fracpart .. 'Z'
    req.timestamp = timestamp

    req.request.attributes=request_map.attrs
    handle.var.request_map = cjson.encode(req)
end

return session_rust_nginx