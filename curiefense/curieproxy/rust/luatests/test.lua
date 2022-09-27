package.path = package.path .. ";lua/?.lua"
local curiefense = require "curiefense"

local cjson = require "cjson"
local json_safe = require "cjson.safe"
local json_decode = json_safe.decode

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

local function run_inspect_request_gen(raw_request_map, mode)
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
    local res
    if human ~= nil then
      res = curiefense.test_inspect_request(meta, headers, raw_request_map.body, ip, human)
    else
      if mode ~= "lua_async" then
        res = curiefense.inspect_request("debug", meta, headers, raw_request_map.body, ip)
      else
        local r1 = curiefense.inspect_request_init("debug", meta, headers, raw_request_map.body, ip)
        if r1.error then
          error(r1.error)
        end
        if r1.decided then
          return r1
        end
        local flows = r1.flows
        local conn = redis.connect(redishost, redisport)

        -- very naive and simple implementation of flow / limit checks
        local rflows = {}
        for _, flow in pairs(flows) do
          local key = flow.key
          local len = conn:llen(key)
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
              conn:lpush(key, "foo")
              local ttl = conn:ttl(key)
              if ttl == nil or ttl < 0 then
                conn:expire(key, flow.timeframe)
              end
            end
          end
          table.insert(rflows, flow:result(flowtype))
        end

        local limits = r1.limits
        local rlimits = {}
        for _, limit in pairs(limits) do
          local key = limit.key
          local curcount = 1
          if not limit.zero_limits then
            local pw = limit.pairwith
            local expire
            if pw then
              conn:sadd(key, pw)
              curcount = conn:scard(key)
              expire = conn:ttl(key)
            else
              curcount = conn:incr(key)
              expire = conn:ttl(key)
            end
            if curcount == nil then
              curcount = 0
            end
            if expire == nil or expire < 0 then
              conn:expire(key, limit.timeframe)
            end
          end
          table.insert(rlimits, limit:result(curcount))
        end

        res = curiefense.inspect_request_process(r1, rflows, rlimits)
      end
    end
    if res.error then
      error(res.error)
    end
    return res
end

local function run_inspect_request(raw_request_map)
  return run_inspect_request_gen(raw_request_map, "lua_async")
end

local function show_logs(logs)
  for _, log in ipairs(logs) do
      print(log)
  end
end

local function equals(o1, o2)
  if o1 == o2 then return true end
  local o1Type = type(o1)
  local o2Type = type(o2)
  if o1Type ~= o2Type then return false end
  if o1Type ~= 'table' then return false end
  local keySet = {}

    for key1, value1 in pairs(o1) do
        local value2 = o2[key1]
        if value2 == nil or equals(value1, value2) == false then
            return false
        end
        keySet[key1] = true
    end

    for key2, _ in pairs(o2) do
        if not keySet[key2] then return false end
    end
    return true
  end

-- testing from envoy metadata
local function test_raw_request(request_path)
  print("Testing " .. request_path)
  local raw_request_maps = load_json_file(request_path)
  for _, raw_request_map in pairs(raw_request_maps) do
    local res = run_inspect_request(raw_request_map)

    local r = cjson.decode(res.response)
    local request_map = cjson.decode(res:request_map(nil))

    local good = compare_tag_list(raw_request_map.name, request_map.tags, raw_request_map.response.tags)
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
      if raw_request_map.response.headers then
        local hgood = true
        for h, v in pairs(raw_request_map.response.headers) do
          if r.response.headers[h] ~= v then
            print("Header " .. h .. ", expected " .. cjson.encode(v) .. " but got " ..
              cjson.encode(r.response.headers[h]))
            good = false
            hgood = false
          end
        end
        if not hgood then
          print("Returned headers are " .. cjson.encode(r.response.headers))
        end
      end
      for _, trigger_name in pairs({
         "acl_triggers",
         "rate_limit_triggers",
         "global_filter_triggers",
         "content_filter_triggers"
      }) do
        local expected = raw_request_map.response[trigger_name]
        if expected then
          local actual = request_map[trigger_name]

          if equals(actual, expected) == false then
            local jactual = cjson.encode(actual)
            local jexpected = cjson.encode(expected)
            print("Expected " .. trigger_name .. ":")
            print("  " ..  jexpected)
            print("but got:")
            print("  " .. jactual)
            good = false
          end
        end
      end
    end

    if not good then
      show_logs(request_map.logs)
      print(res.response)
      print(res.request_map)
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

    local res = run_inspect_request(raw_request_map)
    local r = cjson.decode(res.response)
    local request_map = cjson.decode(res:request_map(nil))

    local good = compare_tag_list(raw_request_map.name, request_map.tags, raw_request_map.response.tags)
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
        for _, log in ipairs(request_map.logs) do
            print(log["elapsed_micros"] .. "µs " .. log["message"])
        end
        print(res.response)
        print(res.request_map)
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
    local res = run_inspect_request(raw_request_map)
    local request_map = cjson.decode(res:request_map(nil))
    for _, section in pairs({"arguments", "headers", "cookies"}) do
      for _, value in pairs(request_map[section]) do
        local p = string.find(value["name"], secret)
        if p ~= nil then
          error("Could find secret in " .. section .. "/" .. value["name"])
        end
        p = string.find(value["value"], secret)
        if p ~= nil then
          error("Could find secret in " .. section .. "/" .. value["name"])
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
    local r = run_inspect_request(raw_request_map)
    local res = cjson.decode(r.response)
    local request_map = cjson.decode(r:request_map(nil))

    if raw_request_map.tag and not contains(request_map.tags, raw_request_map.tag) then
      show_logs(request_map.logs)
      error("curiefense.session_limit_check should have returned tag '" .. raw_request_map.tag ..
            "', but returned: " .. r.response)
    end

    if raw_request_map.pass then
      if res["action"] ~= "pass" then
        show_logs(request_map.logs)
        error("curiefense.session_limit_check should have returned pass, but returned: " .. r.response)
      end
    else
      if res["action"] == "pass" then
        show_logs(request_map.logs)
        print("response: " .. r.request_map)
        error("curiefense.session_limit_check should have blocked, but returned: " .. r.response)
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
    local r = run_inspect_request(raw_request_map)
    local request_map = cjson.decode(r:request_map(nil))
    local expected_tag = raw_request_map["tag"]

    local tag_found = false
    for _, tag in pairs(request_map["tags"]) do
      if tag == expected_tag then
        tag_found = true
        break
      end
    end

    if raw_request_map.pass then
      if tag_found then
        print("we found the tag " .. expected_tag .. " in the request info, but it should have been absent")
        good = false
      end
    else
      if not tag_found then
        print("we did not find the tag " .. expected_tag .. " in the request info. All tags:")
        for _, tag in pairs(request_map["tags"]) do
          print(" * " .. tag)
        end
        good = false
      end
    end

    if not good then
        for _, log in ipairs(request_map.logs) do
            print(log)
        end
        print(r.response)
        print(r.request_map)
        error("mismatch in flow control")
    end

    if raw_request_map.delay then
      socket.sleep(raw_request_map.delay)
    end
  end
end

local test_request = '{ "headers": { ":authority": "localhost:30081", ":method": "GET", ":path": "/dqsqsdqsdcqsd"' ..
  ', "user-agent": "dummy", "x-forwarded-for": "12.13.14.15" }, "name": "test block by ip tagging", "response": {' ..
  '"action": "custom_response", "block_mode": true, "status": 503, "tags": [ "all", "geo:united-states", "ip:12-1' ..
  '3-14-15", "sante", "securitypolicy-entry:default", "contentfiltername:default-contentfilter", "securitypolicy:' ..
  'default-entry", "aclname:default-acl", "aclid:--default--", "asn:7018", "tagbyip", "contentfilterid:--default-' ..
  '-", "bot" ] } }'

print("***  first request logs, check for configuration problems here ***")
local tres = run_inspect_request(json_decode(test_request))
show_logs(tres.logs)
print("*** done ***")
print("")

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