package.path = package.path .. ";lua/?.lua"
local curiefense = require "curiefense"

local sfmt = string.format
local cjson = require "cjson"
local json_safe = require "cjson.safe"
local json_decode = json_safe.decode

local grasshopper = require "grasshopper"
local nativeutils = require "nativeutils"
local startswith = nativeutils.startswith

local ffi = require "ffi"
ffi.load("crypto", true)

local redis = require "lua.redis"
local socket = require "socket"
local redishost = os.getenv("REDIS_HOST") or "redis"
local redisport = os.getenv("REDIS_PORT") or 6379

local lfs = require 'lfs'

local function ends_with(str, ending)
  return ending == "" or str:sub(-#ending) == ending
end
local function read_file(path)
    local fh = io.open(path, "r")
    if fh ~= nil then
        local data = fh:read("*all")
        fh:close()
        if data then
            return data
        end
    end
end
local function load_json_file(path)
    local data = read_file(path)
    if data then
        return json_decode(data)
    end
end

local _, err = curiefense.init_config()
if err then
    local failure = false
    for _, r in ipairs(err) do
        if not ends_with(r, "CFGLOAD") then
          print(sfmt("curiefense.init_config failed: '%s'", r))
          failure = true
        end
    end
    if failure then
      error("Configuration loading failed")
    end
end

-- test that two lists contain the same tags
local function compare_tag_list(name, actual, expected)
  local m_actual = {}
  for _, a in ipairs(actual) do
    if not startswith(a, "container:") then
      m_actual[a] = 1
    end
  end
  for _, e in ipairs(expected) do
    if not startswith(e, "container:") and not m_actual[e] then
      error(name .. " - missing expected tag: " .. e)
    end
    m_actual[e] = nil
  end
  local good = true
  for a, _ in pairs(m_actual) do
    print(a)
    good = false
  end
  if not good then
    error("^ extra tags in " .. name)
  end
end

local function run_inspect_request(raw_request_map)
    local meta = {}
    local headers = {}
    for k, v in pairs(raw_request_map.headers) do
      if startswith(k, ":") then
          meta[k:sub(2):lower()] = v
      else
          headers[k] = v
      end
    end
    local ip = "1.2.3.4"
    if raw_request_map.ip then
      ip = raw_request_map.ip
    elseif headers["x-forwarded-for"] then
      ip = headers["x-forwarded-for"]
    end

    local response, merr = curiefense.inspect_request(meta, headers, raw_request_map.body, ip, grasshopper)
    if merr then
      error(merr)
    end
    return response
end

-- testing from envoy metadata
local function test_raw_request(request_path)
  print("Testing " .. request_path)
  local raw_request_maps = load_json_file(request_path)
  for _, raw_request_map in pairs(raw_request_maps) do
    local response = run_inspect_request(raw_request_map)
    local r = cjson.decode(response)

    compare_tag_list(raw_request_map.name, r.request_map.tags, raw_request_map.response.tags)
    local good = true
    if r.action ~= raw_request_map.response.action then
      print("Expected action " .. cjson.encode(raw_request_map.response.action) ..
        ", but got " .. cjson.encode(r.action))
      good = false
    end
    if r.response ~= cjson.null then
      if r.response.status ~= raw_request_map.response.status then
        print("Expected status " .. cjson.encode(raw_request_map.response.status) ..
          ", but got " .. cjson.encode(r.response.status))
        good = false
      end
      if r.response.block_mode ~= raw_request_map.response.block_mode then
        print("Expected block_mode " .. cjson.encode(raw_request_map.response.block_mode) ..
          ", but got " .. cjson.encode(r.response.block_mode))
        good = false
      end
    end

    if not good then
      for _, log in ipairs(r.logs) do
          print(log["elapsed_micros"] .. "µs " .. log["message"])
      end
      error("mismatch in " .. raw_request_map.name)
    end
  end
end

-- remove all keys from redis
local function clean_redis()
    local conn = redis.connect(redishost, redisport)
    local keys = conn:keys("*")
    for _, key in pairs(keys) do
      conn:del(key)
    end
end

-- testing for rate limiting
local function test_ratelimit(request_path)
  print("Rate limit " .. request_path)
  clean_redis()
  local raw_request_maps = load_json_file(request_path)
  for n, raw_request_map in pairs(raw_request_maps) do
    print(" -> step " .. n)
    local jres = run_inspect_request(raw_request_map)
    local res = cjson.decode(jres)

    if raw_request_map.pass then
      if res["action"] ~= "pass" then
        error("curiefense.session_limit_check should have returned pass, but returned: " .. jres)
      end
    else
      if res["action"] == "pass" then
        error("curiefense.session_limit_check should have blocked, but returned: " .. jres)
      end
    end

    if raw_request_map.delay then
      socket.sleep(raw_request_map.delay)
    end
  end
end

-- testing for control flow
local function test_flow(request_path)
  print("Flow control " .. request_path)
  clean_redis()
  local raw_request_maps = load_json_file(request_path)
  for n, raw_request_map in pairs(raw_request_maps) do
    print(" -> step " .. n)
    local jres = run_inspect_request(raw_request_map)
    local res = cjson.decode(jres)

    if raw_request_map.pass then
      if res["action"] ~= "pass" then
        error("curiefense.session_flow_check should have returned pass, but returned: " .. jres)
      end
    else
      if res["action"] == "pass" then
        error("curiefense.session_flow_check should have blocked, but returned: " .. jres)
      end
    end

    if raw_request_map.delay then
      socket.sleep(raw_request_map.delay)
    end
  end
end

for file in lfs.dir[[luatests/raw_requests]] do
  if ends_with(file, ".json") then
    test_raw_request("luatests/raw_requests/" .. file)
  end
end

for file in lfs.dir[[luatests/flows]] do
  if ends_with(file, ".json") then
    test_flow("luatests/flows/" .. file)
  end
end

for file in lfs.dir[[luatests/ratelimit]] do
  if ends_with(file, ".json") then
    test_ratelimit("luatests/ratelimit/" .. file)
  end
end
