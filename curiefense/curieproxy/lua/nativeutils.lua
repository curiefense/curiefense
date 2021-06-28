module(..., package.seeall)
-- helpers for native rust libraries
local accesslog   = require "lua.accesslog"
local log_request = accesslog.envoy_log_request
local cjson       = require "cjson"

function trim(s)
    return (string.gsub(s, "^%s*(.-)%s*$", "%1"))
end

function string:startswith(arg)
  return string.sub(self, 1, #arg) == arg
end

function string:endswith(arg)
  return string.sub(self, 1, #arg) == arg
end

function startswith(str, arg)
    if str and arg and type(str) == "string" and type(arg) == "string" then
        return string.find(str, arg, 1, true) == 1
    end
end

function endswith(str, arg)
    if str and arg then
        return string.find(str, arg, #str - #arg + 1, true) == #str - #arg + 1
    end
end

-- source http://lua-users.org/wiki/SplitJoin
function string:split(sSeparator, nMax, bRegexp)

    local aRecord = {}

    if sSeparator ~= '' then
      if (nMax == nil or nMax >= 1)then
        if self ~= nil then
          if self:len() > 0 then
            local bPlain = not bRegexp
            nMax = nMax or -1

            local nField=1 nStart=1
            local nFirst,nLast = self:find(sSeparator, nStart, bPlain)
            while nFirst and nMax ~= 0 do
                aRecord[nField] = self:sub(nStart, nFirst-1)
                nField = nField+1
                nStart = nLast+1
                nFirst,nLast = self:find(sSeparator, nStart, bPlain)
                nMax = nMax-1
            end
            aRecord[nField] = self:sub(nStart)
          end
        end
      end
    end

    return aRecord
end

function map_fn (T, fn)
    T = T or {}
    local ret = {}
    for _, v in ipairs(T) do
        local new_value = fn(v)
        table.insert(ret, new_value)
    end
    return ret
end

function nginx_custom_response(request_map, action_params)
    if not action_params then action_params = {} end
    local block_mode = action_params.block_mode
    -- if not block_mode then block_mode = true end

    local handle = request_map.handle

    if action_params["headers"] and action_params["headers"] ~= cjson.null then
        for k, v in pairs(response["headers"]) do
            handle.log[k] = v
        end
    end

    if action_params["status"] then
        ngx.status = action_params["status"]
    end

    handle.log(handle.ERR, cjson.encode(action_params))

    if block_mode then
        if action_params["content"] then handle.say(action_params["content"]) end
        handle.exit(ngx.HTTP_OK)
    end

end

function log_nginx_messages(handle, logs)
    for _, log in ipairs(logs) do
        level = log["level"]
        msg = log["elapsed_micros"] .. "µs " .. log["message"]
        if level == "debug" then
            handle.log(handle.DEBUG, msg)
        elseif level == "info" then
            handle.log(handle.INFO, msg)
        elseif level == "warning" then
            handle.log(handle.WARN, msg)
        elseif level == "error" then
            handle.log(handle.ERR, msg)
        else
            handle.log(handle.ERR, "Can't log this message: " .. cjson.encode(logs))
        end
    end
end

function envoy_custom_response(request_map, action_params)
    if not action_params then action_params = {} end
    local block_mode = action_params.block_mode
    -- if not block_mode then block_mode = true end

    local handle = request_map.handle
    -- handle:logDebug(string.format("custom_response - action_params %s, block_mode %s", json_encode(action_params), block_mode))

    local response = {
        [ "status" ] = "403",
        [ "headers"] = { ["x-curiefense"] = "response" },
        [ "reason" ] = { initiator = "undefined", reason = "undefined"},
        [ "content"] = "curiefense - request denied"
    }

    -- override defaults
    if action_params["status" ] then response["status" ] = action_params["status" ] end
    if action_params["headers"] and action_params["headers"] ~= cjson.null then response["headers"] = action_params["headers"] end
    if action_params["reason" ] then response["reason" ] = action_params["reason" ] end
    if action_params["content"] then response["content"] = action_params["content"] end

    response["headers"][":status"] = response["status"]

    request_map.attrs.blocked = true
    request_map.attrs.block_reason = response["reason"]


    if block_mode then
        log_request(request_map)
        request_map.handle:respond( response["headers"], response["content"])
    end

end


function log_envoy_messages(handle, logs)
    for _, log in ipairs(logs) do
        level = log["level"]
        msg = log["elapsed_micros"] .. "µs " .. log["message"]
        if level == "debug" then
            handle:logDebug(msg)
        elseif level == "info" then
            handle:logInfo(msg)
        elseif level == "warning" then
            handle:logWarn(msg)
        elseif level == "error" then
            handle:logErr(msg)
        else
            handle:logErr("Can't log this message: " .. cjson.encode(logs))
        end
    end
end