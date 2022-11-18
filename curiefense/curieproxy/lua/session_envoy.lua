local session_rust_envoy = {}
local cjson       = require "cjson"
local curiefense  = require "curiefense"
local utils       = require "lua.nativeutils"
local sfmt = string.format

local function detectip(xff, hops)
    local len_xff = #xff
    if hops < len_xff then
        return xff[len_xff-(hops-1)]
    else
        return xff[1]
    end
end

local function extract_ip(headers, metadata)
    local client_addr = "1.1.1.1"
    local xff = headers:get("x-forwarded-for")
    local hops = metadata:get("xff_trusted_hops") or "0"

    hops = tonumber(hops)
    local addrs = utils.map_fn(utils.split(xff, ","), utils.trim)

    client_addr = detectip(addrs, hops) or client_addr

    return client_addr
end

-- dynamic metadata filter name
local DMFN = "com.reblaze.curiefense"
local LOG_KEY = "request.info"

local function log_request(handle, inspection_result)
  handle:streamInfo():dynamicMetadata():set(DMFN, LOG_KEY, inspection_result:request_map(nil))
end

local function custom_response(handle, action_params)
    if not action_params then action_params = {} end
    local block_mode = action_params.block_mode
    -- if not block_mode then block_mode = true end

    if not block_mode then
        handle:logDebug("altering the request")
        local headers = handle:headers()
        if type(action_params.headers) == "table" then
            for k, v in pairs(action_params.headers) do
                headers:replace(k, v)
            end
        end
        return
    end

    local response = {
        [ "status" ] = "503",
        [ "headers"] = { [":status"] = "503" },
        [ "reason" ] = { initiator = "undefined", reason = "undefined"},
        [ "content"] = "request denied"
    }

    -- override defaults
    if action_params["status"] then response["status"] = action_params["status"] end
    if action_params["headers"] and action_params["headers"] ~= cjson.null then
        response["headers"] = action_params["headers"]
    end
    if action_params["reason" ] then response["reason" ] = action_params["reason" ] end
    if action_params["content"] then response["content"] = action_params["content"] end

    response["headers"][":status"] = response["status"]

    if block_mode then
        handle:logDebug(cjson.encode(response))
        handle:respond( response["headers"], response["content"])
    end
end

function session_rust_envoy.on_response(handle)
    handle:logDebug("todo, capture return code")
end

function session_rust_envoy.inspect(handle)
    local ip_str = extract_ip(handle:headers(), handle:metadata())

    local headers = {}
    local meta = {}
    for k, v in pairs(handle:headers()) do
        if utils.startswith(k, ":") then
            meta[k:sub(2):lower()] = v
        else
            if headers[k] then
                headers[k] = headers[k] .. " " .. v
            else
                headers[k] = v
            end
        end
    end

    local hbody = handle:body()
    local body_content = nil
    if hbody then
        body_content = hbody:getBytes(0, hbody:length())
    end

    -- the meta table contains the following elements:
    --   * path : the full request uri
    --   * method : the HTTP verb
    --   * authority : optionally, the HTTP2 authority field
    local res = curiefense.inspect_request(
        {loglevel="info", meta=meta, headers=headers, body=body_content, ip=ip_str}
    )

    log_request(handle, res)

    if res.error then
        handle:logErr(sfmt("curiefense.inspect_request_map error %s", res.error))
    end

    local response = res.response
    if response then
        local response_table = cjson.decode(response)
        handle:logDebug("decision " .. response)
        for _, log in ipairs(res.logs) do
            handle:logDebug(log)
        end
        if response_table["action"] == "custom_response" then
            custom_response(handle, response_table["response"])
        end
    end
end

return session_rust_envoy