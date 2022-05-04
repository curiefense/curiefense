package.path = package.path .. ";lua/?.lua"
local curiefense = require "curiefense"

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

-- check a table contains element
local function contains(list, x)
  for _, v in pairs(list) do
    if v == x then return true end
  end
  return false
end
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

-- test that two lists contain the same tags
local function compare_tag_list(name, actual, expected)
  -- do not check tags when they are unspecified
  if expected == nil then
    return true
  end

  local m_actual = {}
  local good = true
  for _, a in ipairs(actual) do
    if not startswith(a, "container:") then
      m_actual[a] = 1
    end
  end
  for _, e in ipairs(expected) do
    if not startswith(e, "container:") and not m_actual[e] then
      good = false
      print(name .. " - missing expected tag: " .. e)
    end
    m_actual[e] = nil
  end
  if not good then
    print("Actual tags:")
    for _, e in ipairs(actual) do
      print("  " .. e)
    end
    print("^ missing tags in " .. name)
    return false
  end
  for a, _ in pairs(m_actual) do
    print(a)
    good = false
  end
  if not good then
    print("^ extra tags in " .. name)
  end
  return good
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

    local human = nil
    if raw_request_map.human ~= nil then
      human = raw_request_map.human
      if human then
        headers["Cookie"] = "rbzid=OK;"
      end
    end
    local response, merr
    if human ~= nil then
      response, merr = curiefense.test_inspect_request(meta, headers, raw_request_map.body, ip, human)
    else
      response, merr = curiefense.inspect_request(meta, headers, raw_request_map.body, ip, grasshopper)
    end
    if merr then
      error(merr)
    end
    return response
end

local function show_logs(logs)
  for _, log in ipairs(logs) do
      print(log["elapsed_micros"] .. "µs " .. log["message"])
  end
end


-- testing from envoy metadata
local function test_raw_request(request_path)
  print("Testing " .. request_path)
  local raw_request_maps = load_json_file(request_path)
  for _, raw_request_map in pairs(raw_request_maps) do
    local response = run_inspect_request(raw_request_map)
    local r = cjson.decode(response)

    local good = compare_tag_list(raw_request_map.name, r.request_map.tags, raw_request_map.response.tags)
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
      if raw_request_map.response.reason then
        local actual = cjson.encode(r.response.reason)
        local expected = cjson.encode(raw_request_map.response.reason)
        if actual ~= expected then
          print("Expected reason " .. expected ..  ", but got " .. actual)
          good = false
        end
      end
    end

    if not good then
      show_logs(r.logs)
      print(response)
      error("mismatch in " .. raw_request_map.name)
    end
  end
end

-- with stats
local function test_raw_request_stats(request_path, pverbose)
  print("Testing " .. request_path)
  local total = 0
  local ok = 0
  local raw_request_maps = load_json_file(request_path)
  for _, raw_request_map in pairs(raw_request_maps) do

    total = total + 1

    local verbose = pverbose
    if raw_request_map["verbose"] ~= nil then
      verbose = raw_request_map["verbose"]
    end

    local response = run_inspect_request(raw_request_map)
    local r = cjson.decode(response)

    local good = compare_tag_list(raw_request_map.name, r.request_map.tags, raw_request_map.response.tags)
    if r.action ~= raw_request_map.response.action then
      if verbose then
        print("Expected action " .. cjson.encode(raw_request_map.response.action) ..
          ", but got " .. cjson.encode(r.action))
      end
      good = false
    end
    if r.response ~= cjson.null then
      if r.response.status ~= raw_request_map.response.status then
        if verbose then
          print("Expected status " .. cjson.encode(raw_request_map.response.status) ..
            ", but got " .. cjson.encode(r.response.status))
        end
        good = false
      end
      if r.response.block_mode ~= raw_request_map.response.block_mode then
        if verbose then
          print("Expected block_mode " .. cjson.encode(raw_request_map.response.block_mode) ..
            ", but got " .. cjson.encode(r.response.block_mode))
        end
        good = false
      end
    end

    if not good then
      if verbose then
        for _, log in ipairs(r.logs) do
            print(log["elapsed_micros"] .. "µs " .. log["message"])
        end
        print(response)
      end
      print("mismatch in " .. raw_request_map.name)
    else
      ok = ok + 1
    end
  end
  print("good: " .. ok .. "/" .. total .. " - " .. string.format("%.2f%%", 100.0 * ok / total))
end


local function test_masking(request_path)
  print("Testing " .. request_path)
  local raw_request_maps = load_json_file(request_path)
  for _, raw_request_map in pairs(raw_request_maps) do
    local secret = raw_request_map["secret"]
    local response = run_inspect_request(raw_request_map)
    local r = cjson.decode(response)
    for _, section in pairs({"args", "headers", "cookies", "path"}) do
      for k, value in pairs(r.request_map[section]) do
        local p = string.find(value, secret)
        if p ~= nil then
          error("Could find secret in " .. section .. "/" .. k)
        end
      end
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

    if raw_request_map.tag and not contains(res.request_map.tags, raw_request_map.tag) then
      show_logs(res.logs)
      error("curiefense.session_limit_check should have returned tag '" .. raw_request_map.tag ..
            "', but returned: " .. jres)
    end

    if raw_request_map.pass then
      if res["action"] ~= "pass" then
        show_logs(res.logs)
        error("curiefense.session_limit_check should have returned pass, but returned: " .. jres)
      end
    else
      if res["action"] == "pass" then
        show_logs(res.logs)
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
  local good = true
  local raw_request_maps = load_json_file(request_path)
  for n, raw_request_map in pairs(raw_request_maps) do
    print(" -> step " .. n)
    local jres = run_inspect_request(raw_request_map)
    local res = cjson.decode(jres)

    if raw_request_map.pass then
      if res["action"] ~= "pass" then
        print("curiefense.session_flow_check should have returned pass, but returned: " .. res["action"])
        good = false
      end
    else
      if res["action"] == "pass" then
        print("curiefense.session_flow_check should have blocked, but returned: " .. res["action"])
        good = false
      end
    end

    if not good then
        for _, log in ipairs(res.logs) do
            print(log["elapsed_micros"] .. "µs " .. log["message"])
        end
        print(jres)
        error("mismatch in flow control")
    end

    if raw_request_map.delay then
      socket.sleep(raw_request_map.delay)
    end
  end
end

-- running content filter only filter
local function run_inspect_content_filter(raw_request_map)
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

    local response, merr = curiefense.inspect_content_filter(meta, headers, raw_request_map.body, ip,
      raw_request_map.content_filter_id)
    if merr then
      error(merr)
    end
    return response
end

local test_request = '{ "headers": { ":authority": "localhost:30081", ":method": "GET", ":path": "/dqsqsdqsdcqsd"' ..
  ', "user-agent": "dummy", "x-forwarded-for": "12.13.14.15" }, "name": "test block by ip tagging", "response": {' ..
  '"action": "custom_response", "block_mode": true, "status": 503, "tags": [ "all", "geo:united-states", "ip:12-1' ..
  '3-14-15", "sante", "securitypolicy-entry:default", "contentfiltername:default-contentfilter", "securitypolicy:' ..
  'default-entry", "aclname:default-acl", "aclid:--default--", "asn:7018", "tagbyip", "contentfilterid:--default-' ..
  '-", "bot" ] } }'

print("***  first request logs, check for configuration problems here ***")
local tresponse = run_inspect_request(json_decode(test_request))
show_logs(cjson.decode(tresponse)["logs"])
print("*** done ***")
print("")

-- testing content filter only filtering
local function test_content_filter(request_path)
  print("Testing " .. request_path)
  local raw_request_maps = load_json_file(request_path)
  for _, raw_request_map in pairs(raw_request_maps) do
    local response = run_inspect_content_filter(raw_request_map)
    local r = cjson.decode(response)

    local good = true

    for _, log in ipairs(r.logs) do
        if log["message"] == "Content Filter profile not found" then
          print("content filter profile '" .. raw_request_map.content_filter_id .. "' not found")
          good = false
        end
    end

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
    end

    if not good then
      for _, log in ipairs(r.logs) do
          print(log["elapsed_micros"] .. "µs " .. log["message"])
      end
      print(response)
      error("mismatch in " .. raw_request_map.name .. " profile: " .. raw_request_map.content_filter_id)
    end
  end
end

local prefix = nil

if arg[1] == "GOWAF" then
  for file in lfs.dir[[luatests/gowaf]] do
    if ends_with(file, ".json") then
      test_raw_request_stats("luatests/gowaf/" .. file, false)
    end
  end
  os.exit()
elseif arg[1] then
  prefix = arg[1]
end

for file in lfs.dir[[luatests/raw_requests]] do
  if startswith(file, prefix) and ends_with(file, ".json") then
    test_raw_request("luatests/raw_requests/" .. file)
  end
end

for file in lfs.dir[[luatests/masking]] do
  if startswith(file, prefix) and ends_with(file, ".json") then
    test_masking("luatests/masking/" .. file)
  end
end

for file in lfs.dir[[luatests/contentfilter_only]] do
  if startswith(file, prefix) and ends_with(file, ".json") then
    test_content_filter("luatests/contentfilter_only/" .. file)
  end
end

for file in lfs.dir[[luatests/ratelimit]] do
  if startswith(file, prefix) and ends_with(file, ".json") then
    test_ratelimit("luatests/ratelimit/" .. file)
  end
end

for file in lfs.dir[[luatests/flows]] do
  if startswith(file, prefix) and ends_with(file, ".json") then
    test_flow("luatests/flows/" .. file)
  end
end
