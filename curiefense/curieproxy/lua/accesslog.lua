local accesslog = {}
local cjson = require "cjson"
local json_encode   = cjson.encode

-- dynamic metadata filter name
local DMFN = "com.reblaze.curiefense"
local LOG_KEY = "request.info"

local function get_log_table(request_map)
  -- handle is userData which is not serializable
  local entries = {
    ["geo"]     = "geo",
    ["headers"] = "headers",
    ["cookies"] = "cookies",
    ["args"]    = "arguments",
    ["attrs"]   = "attributes",
    ["tags"]    = "tags"
  }

  local log_table = {}

  for luaname, logname in pairs(entries) do
    log_table[logname] = request_map[luaname]
  end

  log_table.blocked = log_table.attributes.blocked
  log_table.block_reason = log_table.attributes.block_reason

  log_table.attributes.blocked = nil
  log_table.attributes.block_reason = nil
  return log_table
end

local function get_log_str_map(request_map)
  local log_table = get_log_table(request_map)
  local str_map = json_encode(log_table)
  return str_map
end

function accesslog.envoy_log_request(request_map)
  local request_handle = request_map.handle
  local str_map = get_log_str_map(request_map)
  request_handle:logDebug(str_map)
  request_handle:streamInfo():dynamicMetadata():set(DMFN, LOG_KEY, str_map)
end

return accesslog