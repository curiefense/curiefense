module(..., package.seeall)

local cjson = require "cjson"

local json_encode   = cjson.encode
local json_decode   = cjson.decode

-- dynamic metadata filter name
DMFN = "com.reblaze.curiefense"
LOG_KEY = "request.info"

function log_request(request_map)
  -- handle is userData which is not serilizable
  local request_handle = request_map.handle
  local entries = {
  	["geo"] 	= "geo",
  	["headers"] = "headers",
  	["cookies"] = "cookies",
  	["args"] 	= "arguments",
  	["attrs"] 	= "attributes"
  }

  local log_table = {}

  for luaname, logname in pairs(entries) do
    log_table[logname] = request_map[luaname]
  end

  local tags = log_table.attributes.tags

  if tags then
    local tagnames = {}
    local n = 0

    for name,v in pairs(tags) do
      n = n + 1
      tagnames[n] = name
    end

  	log_table.tags = tagnames
    log_table.attributes.tags = nil
  end

  log_table.blocked = log_table.attributes.blocked
  log_table.block_reason = log_table.attributes.block_reason

  log_table.attributes.blocked = nil
  log_table.attributes.block_reason = nil


  local str_map = json_encode(log_table)
  request_handle:logDebug(str_map)

  request_handle:streamInfo():dynamicMetadata():set(DMFN, LOG_KEY, str_map)

end

