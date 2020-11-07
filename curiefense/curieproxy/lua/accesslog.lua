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
  local entries = { "headers", "cookies", "args", "attrs"}
  local log_table = {}

  for _, name in ipairs(entries) do
    log_table[name] = request_map[name]
  end

  -- local str_map = json_encode(log_table)

  request_handle:streamInfo():dynamicMetadata():set(DMFN, LOG_KEY, json_encode(log_table))

end

