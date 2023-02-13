package.path = package.path .. ";lua/?.lua"
local curiefense = require "curiefense"

local cjson = require "cjson"
local json_safe = require "cjson.safe"
local json_decode = json_safe.decode

local nativeutils = require "nativeutils"
local startswith = nativeutils.startswith
local endswith = nativeutils.endswith

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

local function should_skip_tag(tag)
  local prefixes = {"container:", "geo-", "network:"}
  for _, prefix in ipairs(prefixes) do
    if startswith(tag, prefix) then
      return true
    end
  end
  return false
end
-- test that two lists contain the same tags
local function compare_tag_list(name, actual, expected)
  -- do not check tags when they are unspecified
  if expected == nil then
    return true
  end

  local m_actual = {}
  local good = true

  for _, atag in ipairs(actual) do
    if (not should_skip_tag(atag)) then
      m_actual[atag] = 1
    end
  end

  for _, exptag in ipairs(expected) do
    if (not should_skip_tag(exptag)) then
      if not m_actual[exptag] then
        good = false
        print(name .. " - missing expected tag: " .. exptag)
      end
    end
    m_actual[exptag] = nil
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
      res = curiefense.test_inspect_request({loglevel="debug", meta=meta,
              headers=headers, body=raw_request_map.body, ip=ip, human=human,
              plugins=raw_request_map.plugins})
    else
      if mode ~= "lua_async" then
        res = curiefense.inspect_request({loglevel="debug", meta=meta, headers=headers,
                body=raw_request_map.body, ip=ip, plugins=raw_request_map.plugins})
      else
        -- APhase1
        local r1 = curiefense.inspect_request_init({loglevel="debug", meta=meta,
                    headers=headers, body=raw_request_map.body, ip=ip,
                    plugins=raw_request_map.plugins})
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

        -- APhase2I
        local r2 = curiefense.inspect_request_flows(r1, rflows)

        local limits = r2.limits
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

        res = curiefense.inspect_request_process(r2, rlimits)
      end
    end
    if res.error then
      error(res.error)
    end
    return res
end

local function run_inspect_request(raw_request_map, mode)
  local real_mode = "lua_async"
  if mode then
    real_mode = mode
  end
  return run_inspect_request_gen(raw_request_map, real_mode)
end

local function show_logs(logs)
  local config_passed = false
  for _, log in ipairs(logs) do
    if not config_passed then
      if not (startswith(log, "D ") or endswith(log, "error: no rules were selected, empty profile")) then
        print(log)
      end
      if endswith(log, "CFGLOAD logs end") then
        config_passed = true
      end
    else
      print(log)
    end
  end
end

local function equals(o1, o2)
  if o1 == o2 then return true end
  local o1Type = type(o1)
  local o2Type = type(o2)
  if o1Type ~= o2Type then return false end

  if o1Type ~= 'table' then return false end

  local is_array = o1[1] ~= nil -- uncorrect yet simple way to check if the table is an array

  if is_array then
    -- check if arrays are equal by first compairing their lengths, then by veryfing if each element
    -- in o1 matches one element in o2.
    if table.maxn(o1) ~= table.maxn(o2) then
      return false
    end
    for _, value1 in ipairs(o1) do
      local match = false
      for _, value2 in ipairs(o2) do
        if equals(value1, value2) == true then
          match = true
          break
        end
      end
      if match == false then
        return false
      end
    end
    return true
  end

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

local function test_status(expected_response, actual_response)
  local expected_status = expected_response.response.status

  if expected_status == nil then
    -- nothing to check
    return true
  end

  if actual_response.response == cjson.null then
    print("Expected response status " .. cjson.encode(expected_status) .. ", but got no response" )
    return false
  end

  local actual_status = actual_response.response.status

  if actual_status ~= expected_status then
    print("Expected status " .. cjson.encode(expected_status) .. ", but got " .. cjson.encode(actual_status))
    return false
  end

  return true
end

local function test_block_mode(expected_response, actual_response)
  local expected_block_mode = expected_response.response.block

  if expected_block_mode == nil then
    -- nothing to check
    return true
  end

  if actual_response.response == cjson.null then
    print("Expected block_mode " .. cjson.encode(expected_block_mode) .. ", but got no response" )
    return false
  end

  local actual_block_mode = actual_response.response.block_mode

  if actual_block_mode ~= expected_block_mode then
    print("Expected block_mode " ..
      cjson.encode(expected_block_mode) .. ", but got " .. cjson.encode(actual_block_mode))
    return false
  end

  return true
end

local function test_headers(expected_response, actual_response)
  local expected_headers = expected_response.response.headers

  if expected_headers == nil then
    -- nothing to check
    return true
  end

  if actual_response.response == cjson.null then
    print("Expected headers " ..
      cjson.encode(expected_headers) .. ", but got no response" )
    return false
  end

  local actual_headers = actual_response.response.headers

  local good = true
  for h, v in pairs(expected_headers) do
    if actual_headers[h] ~= v then
      print("Header " .. h .. ", expected " .. cjson.encode(v) .. " but got " ..
        cjson.encode(actual_headers[h]))
      good = false
    end
  end

  if not good then
    print("Returned headers are " .. cjson.encode(actual_headers))
  end

  return good
end

local function test_trigger(expected_response, parsed_responses, trigger_name)
  local expected_trigger = expected_response.response[trigger_name]

  if expected_trigger == nil then
    -- nothing to check
    return true
  end

  local actual_trigger = parsed_responses[trigger_name]
  if actual_trigger == cjson.null then
    print("Expected " .. trigger_name .. ":" .. cjson.encode(expected_response) .. ", but got no trigger" )
    return false
  end


  if equals(actual_trigger, expected_trigger) == false then
    print("Expected " .. trigger_name .. ":")
    print("  " ..  cjson.encode(expected_trigger))
    print("but got:")
    print("  " .. cjson.encode(actual_trigger))
    return false
  end

  return true
end

-- testing from envoy metadata
local function test_raw_request(request_path, mode)
  print("Testing " .. request_path .. " mode=" .. mode)
  local raw_request_maps = load_json_file(request_path)
  for _, expected in pairs(raw_request_maps) do
    local res = run_inspect_request(expected, mode)

    local actual = cjson.decode(res.response)
    local request_map = cjson.decode(res:request_map(nil))

    local good = compare_tag_list(expected.name, request_map.tags, expected.response.tags)
    if actual.action ~= expected.response.action then
      print("Expected action " .. cjson.encode(expected.response.action) ..
        ", but got " .. cjson.encode(actual.action))
      good = false
    end
    good = test_status(expected, actual) or good
    good = test_block_mode(expected, actual) or good
    good = test_headers(expected, actual) or good

    local triggers = {
      "acl_triggers",
      "rate_limit_triggers",
      "global_filter_triggers",
      "content_filter_triggers",
      "restriction_triggers"
    }
    for _, trigger_name in pairs(triggers) do
      good = test_trigger(expected, request_map, trigger_name) and good
    end

    if not good then
      show_logs(request_map.logs)
      print(res.response)
      print(res:request_map(nil))
      error("mismatch in " .. expected.name)
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
    local request_map_json = res:request_map(nil)
    local request_map = cjson.decode(request_map_json)

    local good = compare_tag_list(raw_request_map.name, request_map.tags, raw_request_map.response.tags)
    if r.action ~= raw_request_map.response.action then
      if verbose then
        print("Expected action " .. cjson.encode(raw_request_map.response.action) ..
          ", but got " .. cjson.encode(r.action))
      end
      good = false
    end
    if r.response ~= cjson.null then
      if raw_request_map.response.status then
        local response_class = math.floor(r.response.status / 100)
        local rawrm_class = math.floor(raw_request_map.response.status / 100)
        if response_class ~= rawrm_class then
          if verbose then
            print("Expected status class " .. rawrm_class .. "xx (" ..
              cjson.encode(raw_request_map.response.status) ..
              "), but got " .. response_class .. "xx (" ..
              cjson.encode(r.response.status) .. ")")
          end
          good = false
        end
      elseif not r.response.status then
        print("response status mismatch")
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
        show_logs(request_map.logs)
        print(res.response)
        print(request_map_json)
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
local function test_ratelimit(request_path, mode)
  print("Rate limit " .. request_path .. " mode=" .. mode)
  clean_redis()
  local raw_request_maps = load_json_file(request_path)
  for n, raw_request_map in pairs(raw_request_maps) do
    print(" -> step " .. n)
    local r = run_inspect_request(raw_request_map, mode)
    local res = cjson.decode(r.response)
    local request_map = cjson.decode(r:request_map(nil))

    if raw_request_map.tag and not contains(request_map.tags, raw_request_map.tag) then
      show_logs(request_map.logs)
      print("curiefense.session_limit_check should have returned tag '" .. raw_request_map.tag ..
            "', but returned:")
      for _, t in pairs(request_map.tags) do
        print(" * " .. t)
      end
      error("...")
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
local function test_flow(request_path, mode)
  print("Flow control " .. request_path .. " mode=" .. mode)
  clean_redis()
  local good = true
  local raw_request_maps = load_json_file(request_path)
  for n, raw_request_map in pairs(raw_request_maps) do
    print(" -> step " .. n)
    local r = run_inspect_request(raw_request_map, mode)
    local request_map = cjson.decode(r:request_map(nil))
    local expected_tag = raw_request_map["tag"]

    local tag_found = false
    for _, tag in pairs(request_map["tags"]) do
      if tag == expected_tag then
        tag_found = true
        break
      end
    end

    if raw_request_map.last_step then
      if raw_request_map.pass then
        if not tag_found then
          print("we did not find the tag " .. expected_tag .. " in the request info. All tags:")
          for _, tag in pairs(request_map["tags"]) do
            print(" * " .. tag)
          end
          good = false
        end
      else
        if tag_found then
          print("we found the tag " .. expected_tag .. " in the request info, but it should have been absent")
          good = false
        end
      end
    else
      if tag_found then
        print("we found the tag " .. expected_tag .. " in the request info, " ..
              "but it should have been absent (not the last step)")
        good = false
      end
    end

    local response = r.response
    local res = cjson.decode(response)
    if raw_request_map.pass then
      if res["action"] ~= "pass" then
        print("curiefense.session_limit_check should have returned pass")
        good = false
      end
    else
      if res["action"] ~= "custom_response" then
        print("curiefense.session_limit_check should have returned custom_response")
        good = false
      end
    end

    if not good then
        show_logs(request_map.logs)
        print("response: " .. response)
        local tags = request_map["tags"]
        table.sort(tags)
        print("tags: " .. cjson.encode(tags))
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
    test_raw_request("luatests/raw_requests/" .. file, "lua_async")
    test_raw_request("luatests/raw_requests/" .. file, "standard")
  end
end

for file in lfs.dir[[luatests/masking]] do
  if startswith(file, prefix) and ends_with(file, ".json") then
    test_masking("luatests/masking/" .. file)
  end
end

for file in lfs.dir[[luatests/ratelimit]] do
  if startswith(file, prefix) and ends_with(file, ".json") then
    test_ratelimit("luatests/ratelimit/" .. file, "lua_async")
    test_ratelimit("luatests/ratelimit/" .. file, "standard")
  end
end

for file in lfs.dir[[luatests/flows]] do
  if startswith(file, prefix) and ends_with(file, ".json") then
    test_flow("luatests/flows/" .. file, "lua_async")
    test_flow("luatests/flows/" .. file, "standard")
  end
end
