local session_rust_nginx = {}
local cjson       = require "cjson"
local curiefense  = require "curiefense"
local sfmt = string.format
local redis = require "resty.redis"

local HOPS = os.getenv("XFF_TRUSTED_HOPS") or 1

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

    handle.log(handle.ERR, cjson.encode(action_params))

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
    local redishost = os.getenv("REDIS_HOST") or "redis"
    local redisport = os.getenv("REDIS_PORT") or 6379
    local red = redis:new()
    red:set_timeout(100) -- 100ms
    local _, err = red:connect(redishost, redisport)
    if err then
        handle.log(handle.ERR, err)
    end
    return red
end

function session_rust_nginx.inspect(handle)
    local ip_str = handle.var.remote_addr

    local rheaders, err = handle.req.get_headers()
    if err == "truncated" then
        handle.log(handle.ERR, "truncated headers: " .. err)
    end

    local headers = make_safe_headers(rheaders)

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
    local meta = { path=handle.var.request_uri, method=handle.req.get_method(), authority=nil }

    local res = curiefense.inspect_request_init_hops(
        meta, headers, body_content, ip_str, HOPS
    )

    if res.error then
        handle.log(handle.ERR, sfmt("curiefense.inspect_request_init error %s", res.error))
    end

    if not res.decided then
        -- handle flow / limit
        local flows = res.flows
        local limits = res.limits
        local rflows = {}
        local rlimits = {}

        -- TODO: avoid connecting to redis when there are no flows and all limits are zero limits
        if not rawequal(next(flows), nil) or not rawequal(next(limits), nil) then
            -- Redis required
            -- TODO: write a pipelined implementation that will run through all the flow and limits at once!
            local red = redis_connect(handle)
            red:init_pipeline()

            for _, flow in pairs(flows) do
                red:llen(flow.key)
            end
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
            if not results then
                handle.log(handle.ERR, "failed to run redis calls " .. redis_err)
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
        end

        res = curiefense.inspect_request_process(res, rflows, rlimits)
        if res.error then
            handle.log(handle.ERR, sfmt("curiefense.inspect_request_process error %s", res.error))
        end
    end

    handle.ctx.request_map = res.request_map

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
    end
end

-- log block stage processing
function session_rust_nginx.log(handle)
    local request_map = handle.ctx.request_map
    handle.ctx.request_map = nil
    handle.var.request_map = request_map
end

return session_rust_nginx