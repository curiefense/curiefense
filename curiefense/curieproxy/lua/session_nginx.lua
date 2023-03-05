local session_rust_nginx = {}

-- local cjson       = require "cjson"
-- see more at https://github.com/openresty/lua-cjson/blob/91c3363e7dc86263a379719e5508fc0979df84d3/manual.txt#module-instantiation

local cjson         = require "cjson.safe"
local curiefense    = require "curiefense"
local utils         = require "lua.nativeutils"
local redis         = require "resty.redis"

local sfmt          = string.format

local HOPS          = os.getenv("XFF_TRUSTED_HOPS") or 1
local redishost     = os.getenv("REDIS_HOST") or "redis"
local redisport     = os.getenv("REDIS_PORT") or 6379

local function custom_response(handle, action_params)
    if not action_params then action_params = {} end
    local block_mode = action_params.block_mode
    -- if not block_mode then block_mode = true end

    if not block_mode then
        handle.log(handle.DEBUG, "altering: " .. cjson.encode(action_params))
        if action_params["headers"] and action_params["headers"] ~= cjson.null then
            for k, v in pairs(action_params.headers) do
                handle.req.set_header(k, v)
            end
        end
        return
    end

    if action_params["headers"] and action_params["headers"] ~= cjson.null then
        for k, v in pairs(action_params["headers"]) do
            handle.header[k] = v
        end
    end

    if action_params["status"] then
        local raw_status = action_params["status"]
        local status = tonumber(raw_status) or raw_status
        handle.status = status
    end

    handle.log(handle.DEBUG, cjson.encode(action_params))

    if block_mode then
        if action_params["content"] then handle.say(action_params["content"]) end
        handle.exit(handle.HTTP_OK)
    end

end

local function make_safe_headers(rheaders)
    local headers = {}

    for k, v in pairs(rheaders) do
        if type(v) == "table" then
            local new_v = v[1]
            for i = 2, #v do
                new_v = new_v .. "; " .. v[i]
            end
            headers[k] = new_v
        else
            headers[k] = v
        end
    end
    return headers
end

local function redis_connect(handle)
    local red = redis:new()
    local ok, err = red:connect(redishost, redisport)
    if not ok then
        handle.log(handle.ERR, "connect error: " .. err)
    end
    return red
end

function get_request_headers(handle)

    local rheaders, err = handle.req.get_headers()
    if err == "truncated" then
        handle.log(handle.ERR, "truncated headers: " .. err)
    end

    return make_safe_headers(rheaders)
end


function session_rust_nginx.inspect(handle, loglevel, secpolid)
    local ip_str = handle.var.remote_addr

    local headers = get_request_headers(handle)

    handle.req.read_body()
    local body_content = handle.req.get_body_data()
    if body_content ~= nil then
        handle.ctx.body_len = body_content:len()
    else
        handle.ctx.body_len = 0
    end
    -- the meta table contains the following elements:
    --   * path : the full request uri
    --   * method : the HTTP verb
    --   * authority : optionally, the HTTP2 authority field
    --   * scheme: http / https
    local meta = { path=handle.var.request_uri, method=handle.req.get_method(), authority=nil,
            scheme=handle.var.scheme }
    local params = {loglevel=loglevel, meta=meta, headers=headers, body=body_content,
            ip=handle.var.remote_addr, hops=HOPS, plugins=plugins}

    if secpolid then
        params['secpolid'] = secpolid
    end

    local res = curiefense.inspect_request_init(params)

    if res.error then
        handle.log(handle.ERR, sfmt("curiefense.inspect_request_init error %s", res.error))
    end

    if not res.decided then
        -- handle flow / limit
        local flows = res.flows
        local rflows = {}
        local rlimits = {}
        local red = nil

        -- TODO: avoid connecting to redis when there are no flows and all limits are zero limits
        if not rawequal(next(flows), nil) then
            -- Redis required
            red = redis_connect(handle)
            red:init_pipeline()
            for _, flow in pairs(flows) do
                red:llen(flow.key)
            end
            local results, redis_err = red:commit_pipeline()
            if redis_err or not results then
                handle.log(handle.ERR, "failed to run redis calls: " .. redis_err)
                flows = {}
            end

            local result_idx = 1

            for _, flow in pairs(flows) do
                local len = results[result_idx]
                result_idx = result_idx + 1
                local step = flow.step
                local flowtype = "nonlast"
                if flow.is_last then
                    if step == len then
                        flowtype = "lastok"
                    else
                        flowtype = "lastblock"
                    end
                else
                    if step == len then
                        local key = flow.key
                        red:lpush(key, "foo")
                        local ttl = red:ttl(key)
                        if ttl == nil or ttl < 0 then
                            red:expire(key, flow.timeframe)
                        end
                    end
                end
                table.insert(rflows, flow:result(flowtype))
            end
        end

        local r2 = curiefense.inspect_request_flows(res, rflows)
        local limits = r2.limits

        if not rawequal(next(limits), nil) then
            if red == nil then
                red = redis_connect(handle)
            end
            red:init_pipeline()

            for _, limit in pairs(limits) do
                local key = limit.key
                if not limit.zero_limits then
                    local pw = limit.pairwith
                    if pw then
                        red:sadd(key, pw)
                        red:scard(key)
                        red:ttl(key)
                    else
                        red:incr(key)
                        red:ttl(key)
                    end
                end
            end
            local results, redis_err = red:commit_pipeline()
            if redis_err or not results then
                handle.log(handle.ERR, "failed to run redis calls: " .. redis_err)
                limits = {}
            end

            local result_idx = 1

            red:init_pipeline()
            for _, limit in pairs(limits) do
                local key = limit.key
                local curcount = 1
                if not limit.zero_limits then
                    local pw = limit.pairwith
                    if pw then
                        result_idx = result_idx + 1
                    end
                    curcount = results[result_idx]
                    result_idx = result_idx + 1
                    local expire = results[result_idx]
                    result_idx = result_idx + 1
                    if curcount == nil then
                        curcount = 0
                    end
                    if expire == nil or expire < 0 then
                        red:expire(key, limit.timeframe)
                    end
                end
                table.insert(rlimits, limit:result(curcount))
            end
            red:commit_pipeline()

            red:set_keepalive(300000, 360)
        end

        res = curiefense.inspect_request_process(r2, rlimits)
        if res.error then
            handle.log(handle.ERR, sfmt("curiefense.inspect_request_process error %s", res.error))
        end
    end

    handle.ctx.res = res

    if res.error then
        handle.log(handle.ERR, sfmt("curiefense.inspect_request_map error %s", res.error))
    end

    local response = res.response
    if response then
        local response_table = cjson.decode(response)
        handle.log(handle.DEBUG, "decision: " .. response)
        for _, log in ipairs(res.logs) do
            handle.log(handle.DEBUG, log)
        end
        if response_table["action"] == "custom_response" then
            custom_response(handle, response_table["response"])
        end
        if response_table["action"] == "pass" then
            local analyser_response = response_table["response"]
            if analyser_response ~= cjson.null and analyser_response["headers"] and
                analyser_response["headers"] ~= cjson.null then
                for k, v in pairs(analyser_response.headers) do
                    handle.req.set_header(k, v)
                end
            end
        end
    end
end

-- log block stage processing
function session_rust_nginx.log(handle, extra)
    local res = handle.ctx.res
    handle.ctx.res = nil
    handle.var.request_map = res:request_map(extra)
end

--[[
{ "plugins":
  { "jwt":
    { "headers": "auth-token-id"}
  }
}
]]--
-- implementation of extracting jwt header
function _jwt_parse (handle, namespace_info)
    local header_name = namespace_info["headers"]
    local headers = get_request_headers(handle)

    if headers then
        local jwt_token = headers[header_name]

        if jwt_token then
            local parts = utils.split(jwt_token, ".")
            if parts and type(parts) == "table" then
                local header, payload, sig = utils.unpack(parts)
                -- verify a valid structure of 3 elements
                if header and payload and sig then
                    local json_payload = ngx.decode_base64(payload)
                    local data_table = cjson.decode(json_payload)
                    return data_table
                end
            end
        end
    end
end

-- {"jwt": {"headers": "auth-token-id"} }
local supported_plugins = {
    [ "jwt" ] = _jwt_parse
}

function session_rust_nginx.load_plugins_data(handle, secpol_plugins)
    local plugins_config = {}
    for name, func in pairs(supported_plugins) do
        local namespace_info = secpol_plugins[name]
        if namespace_info then
            local pi_data = func(handle, namespace_info)
            if pi_data then
                plugins_config [name] = pi_data
            end
        end
    end

    -- check if table is not empty
    if next(plugins_config) ~= nil then
        return plugins_config
    else
        return nil
    end
end


return session_rust_nginx
