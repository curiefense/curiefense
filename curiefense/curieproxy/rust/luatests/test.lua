package.path = package.path .. ";lua/?.lua"
local acl = require "lua.acl"
local tagprofiler = require "lua.tagprofiler"
local redisutils = require "lua.redisutils"
local curiefense = require "curiefense"
local session = require "session_hybrid"
if not session then
  error("Could not load session")
end
local flowcontrol   = require "lua.flowcontrol"

local sfmt = string.format
local cjson = require "cjson"
local json_safe = require "cjson.safe"
local json_decode = json_safe.decode
local waf = require "lua.waf"
local utils = require "lua.utils"
local socket = require "socket"

local nativeutils = require "nativeutils"
local startswith = nativeutils.startswith

local ffi = require "ffi"
ffi.load("crypto", true)

require 'lfs'

ACLNoMatch   = -1
ACLForceDeny = 0
ACLBypass    = 1
ACLAllowBot  = 2
ACLDenyBot   = 3
ACLAllow     = 4
ACLDeny      = 5

WAFPass  = 1
WAFBlock = 0

local function ends_with(str, ending)
  return ending == "" or str:sub(-#ending) == ending
end
function read_file(path)
    local fh = io.open(path, "r")
    if fh ~= nil then
        local data = fh:read("*all")
        fh:close()
        if data then
            return data
        end
    end
end
function load_json_file(path)
    local data = read_file(path)
    if data then
        return json_decode(data)
    end
end

session.global_init(nil)
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

local FakeHandle = {}
function FakeHandle:logDebug(content)
  -- ignore debug
end
function FakeHandle:logInfo(content)
  print(content)
end

function identical_tags_resolved(stage, expected, actual)
  -- checks that all expected entries are in
  local identical = true

  for ek, _ in pairs(expected) do
    if expected[ek] ~= actual[ek] then
      print(sfmt("%s - missing tag %s", stage, ek, expected[ek], actual[ek]))
      identical = false
    end
  end

  for ek, _ in pairs(actual) do
    if expected[ek] ~= actual[ek] then
      print(sfmt("%s - extra tag %s", stage, ek, expected[ek], actual[ek]))
      identical = false
    end
  end

  return identical
end

function identical_tags(stage, request_map, session_uuid)
  local expected = request_map.attrs.tags
  local srm, err = curiefense.session_serialize_request_map(session_uuid)
  if err then
    error("could not serialize request map: " .. err)
  end
  local actual = cjson.decode(srm)["attrs"]["tags"]

  return identical_tags_resolved(stage, expected, actual)
end

function test_request_map(name, request_map)

  local url = request_map.attrs.path
  local host = request_map.headers.host or request_map.attrs.authority

  local encoded = session.encode_request_map(request_map)
  local session_uuid, err = curiefense.session_init(encoded)
  if err then
    error("session_init failed: " .. err)
  end

  local urlmap_entry, url_map = session.match_urlmap(host, url, request_map)

  local acl_active        = urlmap_entry["acl_active"]
  local waf_active        = urlmap_entry["waf_active"]
  local acl_profile_id    = urlmap_entry["acl_profile"]
  local waf_profile_id    = urlmap_entry["waf_profile"]
  local acl_profile       = session.get_acl_profile(acl_profile_id)
  local waf_profile       = session.get_waf_profile(waf_profile_id)

  local json_urlmap, err = curiefense.session_match_urlmap(session_uuid)
  if err then
    error("session_match_urlmap failed: " .. err)
  end
  local rust_urlmap = cjson.decode(json_urlmap)

  for _, k in ipairs({"acl_profile", "waf_profile", "acl_active", "waf_active"}) do
    if rust_urlmap[k] ~= urlmap_entry[k] then
      error(sfmt("urlmap field %s error:\nlua= %s\nrust=%s", k, cjson.encode(urlmap_entry), json_urlmap))
    end
  end

  session.map_tags(request_map,
      sfmt('urlmap:%s', url_map.name),
      sfmt('urlmap-entry:%s', urlmap_entry.name),
      sfmt("aclid:%s", acl_profile_id),
      sfmt("aclname:%s", acl_profile.name),
      sfmt("wafid:%s", waf_profile_id),
      sfmt("wafname:%s", waf_profile.name)
  )

  local _, err = curiefense.session_tag_request(session_uuid)
  if err then
      error("curiefense.session_tag_request failed " .. err)
  end

  tagprofiler.tag_lists(request_map)

  if not identical_tags("match_urlmap", request_map, session_uuid) then
    error("failed at stage match_urlmap")
  end

  -- TODO: check redis related stuff, flow control and rate limit

  local acl_code, acl_result = acl.check(acl_profile, request_map, acl_active)
  local acl_bot_code, acl_bot_result = acl.check_bot(acl_profile, request_map, acl_active)

  local r_acl_code = nil
  local r_acl_bot_code = nil

  local jrust_acl, err = curiefense.session_acl_check(session_uuid)
  if err then
      error("curiefense.session_acl_check failed " .. err)
  end
  local rust_acl = cjson.decode(jrust_acl)
  local txt_acl_result = "?"
  if rust_acl["Match"] then
    bot = rust_acl["Match"]["bot"]
    human = rust_acl["Match"]["human"]
    if bot ~= cjson.null then
      if bot["allowed"] then
        r_acl_bot_code = ACLAllowBot
        txt_acl_result = "AB/"
      else
        r_acl_bot_code = ACLDenyBot
        txt_acl_result = "DB/"
      end
    else
      txt_acl_result = "NB/"
    end
    if human ~= cjson.null then
      if human["allowed"] then
        r_acl_code = ACLAllow
        txt_acl_result = txt_acl_result .. "AH"
      else
        r_acl_code = ACLDeny
        txt_acl_result = txt_acl_result .. "DH"
      end
    else
      txt_acl_result = txt_acl_result .. "NH"
    end
  else
    if rust_acl["Bypass"]["allowed"] then
      r_acl_code = ACLBypass
      txt_acl_result = "BP"
    else
      r_acl_code = ACLForceDeny
      txt_acl_result = "FD"
    end
  end

  if r_acl_code ~= acl_code then
    error(sfmt("for %s, acl_code differs, expected %s, actual %s (result=%s)", name, acl_code, r_acl_code, jrust_acl))
  end
  if acl_code ~= ACLForceDeny and acl_code ~= ACLBypass and r_acl_bot_code ~= acl_bot_code then
    error(sfmt("for %s, acl_bot_code differs, expected %s, actual %s (result=%s, acl_code=%d)", name, acl_bot_code, r_acl_bot_code, jrust_acl, acl_code))
  end


  local waf_code, waf_result = waf.check(waf_profile, request_map)
  local jrwaf_result, err = curiefense.session_waf_check(session_uuid)
  if err then
    error("curiefense.waf_check failed: " .. err)
  end
  local rwaf_result = cjson.decode(jrwaf_result)
  if waf_code == WAFPass and rwaf_result["action"] == "pass" then
    -- ok, both mark it pass passed
  elseif waf_code == WAFBlock and rwaf_result["action"] == "custom_response" then
    -- ok, both mark it as blocked
  else
    print("waf_check mismatch for '" .. name .. "'")
    print("native code returned: " .. jrwaf_result)
    print("lua code " .. cjson.encode(waf_code) .. ", result: " .. cjson.encode(waf_result))
    error(":(")
  end

  if not identical_tags("waf_check", request_map, session_uuid) then
    error("failed at stage waf_check")
  end

  curiefense.session_clean(session_uuid)

  local txt_waf_result = "?"
  if waf_code == WAFPass then
    txt_waf_result = "WAFPass"
  elseif waf_code == WAFBlock then
    txt_waf_result = "WAFBlock"
  end

  return txt_acl_result .. "/" .. txt_waf_result
end

-- testing from a request_map
function test_request(request_path)
  print("Testing " .. request_path)
  local raw_request_map = load_json_file(request_path)
  local request_map = raw_request_map
  request_map.handle = FakeHandle
  test_request_map("(no name)", raw_request_map)
end

-- cheating with the fake handler
local M = {}
function M.__pairs(tbl)
  return pairs(tbl.content)
end

local Machin = {}
function Machin:new(content)
  local t = {}
  t.content = content
  function t:get(key)
    return content[key]
  end
  return setmetatable(t, M)
end

-- test that two lists contain the same tags
function compare_tag_list(actual, expected)
  local m_actual = {}
  for _, a in ipairs(actual) do
    if not startswith(a, "container:") then
      m_actual[a] = 1
    end
  end
  for _, e in ipairs(expected) do
    if not startswith(e, "container:") and not m_actual[e] then
      error("Missing expected tag: " .. e)
    end
    m_actual[e] = nil
  end
  local good = true
  for a, _ in pairs(m_actual) do
    print(a)
    good = false
  end
  if not good then
    error("^ extra tags")
  end
end

-- testing from envoy metadata
function test_raw_request(request_path)
  print("Testing " .. request_path)
  local raw_request_maps = load_json_file(request_path)
  for _, raw_request_map in pairs(raw_request_maps) do
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

    local response, merr = curiefense.inspect_request(meta, headers, raw_request_map.body, ip, nil)
    if merr then
      error(merr)
    end
    local r = cjson.decode(response)

    compare_tag_list(r.request_map.tags, raw_request_map.response.tags)
    local good = true
    if r.action ~= raw_request_map.response.action then
      good = false
    end
    if r.response ~= cjson.null then
      if r.response.status ~= raw_request_map.response.status then
        good = false
      end
      if r.response.block_mode ~= raw_request_map.response.block_mode then
        good = false
      end
    end

    if not good then
      print("Tags")
      print("Got response: " .. response)
      print("Expected parts: " .. cjson.encode(r.response))
      error("mismatch")
    end
   
  end
end

-- remove all keys from redis
function clean_redis()
    local conn = redisutils.redis_connection()
    local keys = conn:keys("*")
    for _, key in pairs(keys) do
      conn:del(key)
    end
end

function redis_debug()
    local conn = redisutils.redis_connection()
    local keys = conn:keys("*")
    for _, key in pairs(keys) do
      tp = conn:type(key)
      if tp == "list" then
        print("* " .. key)
        while true do
          content = conn:lpop(key)
          if content then
            print(" - " .. content)
          else
            break
          end
        end
      else
        error("unhandled key type " .. tp)
      end
    end
end

-- testing for rate limiting
function test_ratelimit(request_path)
  print("Rate limit " .. request_path)
  clean_redis()
  local raw_request_maps = load_json_file(request_path)
  for n, raw_request_map in pairs(raw_request_maps) do
    print(" -> step " .. n)
    local handle = FakeHandle
    function handle.headers()
      return Machin:new(raw_request_map.headers)
    end
    function handle.metadata()
      return Machin:new({xff_trusted_hops=1})
    end
    local request_map = utils.map_request(handle)

    local encoded = session.encode_request_map(request_map)
    session_uuid, err = curiefense.session_init(encoded)
    if err then
      error("session_init failed: " .. err)
    end
    local json_urlmap, err = curiefense.session_match_urlmap(session_uuid)
    if err then
      error("session_match_urlmap failed: " .. err)
    end
    local _, err = curiefense.session_tag_request(session_uuid)
    if err then
        error("curiefense.session_tag_request failed " .. err)
    end
    local jres, err = curiefense.session_limit_check(session_uuid)
    if err then
        error("curiefense.session_limit_check failed " .. err)
    end
    curiefense.session_clean(session_uuid)
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
function test_flow(request_path)
  print("Flow control " .. request_path)
  clean_redis()
  local raw_request_maps = load_json_file(request_path)
  for n, raw_request_map in pairs(raw_request_maps) do
    print(" -> step " .. n)
    local handle = FakeHandle
    function handle.headers()
      return Machin:new(raw_request_map.headers)
    end
    function handle.metadata()
      return Machin:new({xff_trusted_hops=1})
    end
    local request_map = utils.map_request(handle)

    local encoded = session.encode_request_map(request_map)
    session_uuid, err = curiefense.session_init(encoded)
    if err then
      error("session_init failed: " .. err)
    end
    local json_urlmap, err = curiefense.session_match_urlmap(session_uuid)
    if err then
      error("session_match_urlmap failed: " .. err)
    end
    local _, err = curiefense.session_tag_request(session_uuid)
    if err then
        error("curiefense.session_tag_request failed " .. err)
    end

    local jres, err = curiefense.session_flow_check(session_uuid)
    if err then
        error("curiefense.session_flow_check failed " .. err)
    end
    curiefense.session_clean(session_uuid)
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

for file in lfs.dir[[luatests/requests]] do
  if ends_with(file, ".json") then
    test_request("luatests/requests/" .. file)
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
