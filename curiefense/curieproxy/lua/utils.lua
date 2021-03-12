module(..., package.seeall)

local cjson     = require "cjson"
local json_safe = require "cjson.safe"
local rex       = require "rex_pcre2"
local resty_md5 = require "resty.md5"

local maxmind   = require "lua.maxmind"
local globals   = require "lua.globals"
local accesslog = require "lua.accesslog"
local iptools   =  require "iptools"

local find      = string.find
local gsub      = string.gsub
local char      = string.char
local byte      = string.byte
local format    = string.format
local match     = string.match
local gmatch    = string.gmatch

local concat    = table.concat
local insert    = table.insert

local iptonum = iptools.iptonum

local ipinfo    = maxmind.ipinfo

local json_decode   = json_safe.decode
local json_encode   = json_safe.encode
local log_request   = accesslog.log_request

local iptools       = require "iptools"

local urldecode     = iptools.decodeurl
local urlencode     = iptools.encodeurl

nobody = rex.new("(GET|DELETE|TRACE|OPTIONS|HEAD)")
isipv4 = rex.new("(\\d{1,3}\\.){3}\\d{1,3}")
local _match = rex.match

function re_match(subj, patt, cf )
    cf = cf or 'im'
    if type(subj) == "string" and type(patt) == "string" then
        return _match(subj, patt, 1, cf)
    end
end

function new_request_map()
    return {
        headers  = {},
        cookies  = {},
        geo      = {},
        args     = {},
        attrs    = { tags = {} },
        self     = { self = false }
    }
end

function map_headers(headers, map)
    for key, value in pairs(headers) do
        if key == "cookie" then
            map_cookies(value, map)
        else
            if startswith(key, ":") then
                -- map.handle:logDebug("mapping http2 pseudo header " .. key)
                map.attrs[key:sub(2):lower()] = value
            else
                map.headers[key:lower()] = value
            end
        end
    end
end

function map_cookies(cookiestr, map)
    local function kv_spliter (line)
        return line:split("=")
    end

    local cookies = cookiestr:split("; ")

    for k,v in pairs(cookies) do
        cookie = kv_spliter(v)
        map.cookies[cookie[1]] = cookie[2]
    end
end

function map_args(map)
    local _uri = urldecode(map.attrs.path)

    -- query
    if _uri:find("?") then
        local path, query = unpack(_uri:split("?", 1))

        map.attrs.uri = _uri
        map.attrs.path = path
        map.attrs.query = query
        local url_args = {}

        if query then
            url_args = parse_query(query)
        end

        for name, value in pairs(url_args) do
            map.args[name] = value
        end
    else
        map.attrs.uri = _uri
        map.attrs.query = ''
    end
    if not nobody:match(map.attrs.method) then
        local body = parse_body(map)
        for k,v in pairs(body) do
            map.args[k] = urldecode(v)
        end
    end
end

function map_metadata(metadata, map)
    for key, value in pairs(metadata) do
        map.attrs[key] = value
    end
end


function detectip(xff, hops)
    local len_xff = #xff
    if hops < len_xff then
        return xff[len_xff-(hops-1)]
    else
        return xff[1]
    end
end


function map_ip(headers, metadata, map)
    local client_addr = "1.1.1.1"
    local xff = headers:get("x-forwarded-for")
    local hops = metadata:get("xff_trusted_hops") or "1"

    hops = tonumber(hops)
    local addrs = map_fn(xff:split(","), trim)

    client_addr = detectip(addrs, hops) or client_addr

    -- if #addrs == 1 then
    --     client_addr = addrs[1]
    -- elseif #addrs < hops then
    --     client_addr = addrs[#addrs]
    -- else
    --     client_addr = addrs[#addrs-hops]
    -- end

    map.attrs.ip = client_addr
    map.attrs.remote_addr = client_addr
    map.attrs.ipnum = ip_to_num(client_addr)

    local city, country, iso, asn, company = unpack(ipinfo(client_addr, map.handle))

    map.geo.city = {}
    map.geo.country = {}
    map.geo.location = {}
    map.geo.continent = {}

    if city then
        map.geo.city.name = (city.city and city.city.names.en) or "-"

        -- Use lat and lon to match the key names
        -- expected by Elasticsearch's geo_ip field type
        map.geo.location.lat = city.location.latitude
        map.geo.location.lon = city.location.longitude
    end

    if country then
        -- We do this in case the City database
        -- didn't return any results for this ip
        map.geo.country.eu = country.country.is_in_european_union
        map.geo.country.name = country.country.names.en
        map.geo.country.iso = country.country.iso_code

        map.geo.continent.name = country.continent.names.en
        map.geo.continent.code = country.continent.code
    end

    if asn then
        map.geo.asn = tostring(asn)
        map.geo.company = company
    end

end

function tagify(input)
    if type(input) == "string" then
        return input:gsub("[^a-zA-Z%d:]", "-"):lower()
    end
end

function tag_request(r_map, tags)
    r_map.handle:logDebug(format('r_map.attrs %s', cjson.encode(r_map.attrs)))
    if type(tags) == "table" then
        for _, tag in ipairs(tags) do
            tag = tagify(tag)
            r_map.attrs.tags[tag] = 1
        end
    else
        tag = tagify(tags)
        r_map.attrs.tags[tag] = 1
    end
end

function map_request(handle)
    local headers = handle:headers()
    local metadata = handle:metadata()
    local map = new_request_map()

    map.handle = handle

    map_headers(headers, map)
    map_metadata(metadata, map)
    map_ip(headers, metadata, map)
    map_args(map)

    return map
end


function flatten(src_tbl, dst_tbl, prefix)
    if type(src_tbl) ~= "table" then
        return
    end

    if not prefix then
        prefix  = ''
    else
        prefix = prefix .. '_'
    end

    for k,v in pairs(src_tbl) do
        if type(v) == "table" then
            flatten(v, dst_tbl, prefix ..  k)
        else
            dst_tbl[prefix .. k] = v
        end
    end
end

function parse_body(request_map)
    local ctype = request_map.headers['content-type']
    local json_mode = ctype and ctype:find("/json")
    local buffer = request_map.handle:body()
    if buffer then
        local length = buffer:length()
        local body = buffer:getBytes(0, length)

        if json_mode then
            local flat = {}
            flatten(json_decode(body), flat)
            return flat
        else
            return parse_query(body)
        end
    else
        return {}
    end
end

-- function urldecode(str)
--     str = gsub(str, '+', ' ')
--     str = gsub(str, '%%(%x%x)', function(h) return char(tonumber(h, 16)) end)
--     str = gsub(str, '\r\n', '\n')
--     return str
-- end

-- function urlencode(str)
--     if str then
--         str = gsub(str, '\n', '\r\n')
--         str = gsub(str, '([^%w-_.~])', function(c) return format('%%%02X', byte(c)) end)
--     end
--     return str
-- end

-- parse querystring into table. urldecode tokens
function parse_query(str, sep, eq)
    if not sep then sep = '&' end
    if not eq then eq = '=' end
    local vars = {}
    for pair in gmatch(tostring(str), '[^' .. sep .. ']+') do
        if not find(pair, eq) then
            vars[urldecode(pair)] = ''
        else
            local key, value = match(pair, '([^' .. eq .. ']*)' .. eq .. '(.*)')
            if key then
                key = urldecode(key)
                value = urldecode(value)
                local _type = type(vars[key])
                if _type=='nil' then
                    vars[key] = value
                elseif _type=='table' then
                    insert(vars[key], value)
                else
                    vars[key] = {vars[key],value}
                end
            end
        end
    end
    return vars
end


--[[
Copyright 2015 The Luvit Authors. All Rights Reserved.
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at
    http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS-IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
--]]

--[[lit-meta
  name = "luvit/querystring"
  version = "2.0.1"
  license = "Apache 2"
  homepage = "https://github.com/luvit/luvit/blob/master/deps/querystring.lua"
  description = "Node-style query-string codec for luvit"
  tags = {"luvit", "url", "codec"}
]]
function select (T, fn)
    T = T or {}
    local ret = {}
    for _, v in ipairs(T) do
        if fn(v) then
            table.insert(ret, v)
        end
    end
    return ret
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

function ip_to_num(addr)
    local longip = iptonum(addr) or 0
    return tonumber (longip)
end

function nil_or_empty( input )
    if input == nil then return true end
    if type(input) == "string" and input == "" then return true end
    if type(input) == "table" and next(input) == nil then return true end

    return false
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

function string:replace(search, replace,nMax, bRegexp)
    return concat(self:split(search, nMax, bRegexp), replace)
end

function string:tohex()
    return (
        gsub(self,"(.)", function (c)return format("%02X%s",string.byte(c), "") end)
    )
end

function string:within(arg)
    return string.find(arg, self, 1, true)
end

function string:startswith(arg)
  return string.find(self, arg, 1, true) == 1
end

function string:endswith(arg)
  return string.find(self, arg, #self - #arg + 1, true) == #self - #arg + 1
end

function startswith(str, arg)
    if str and arg and type(str) == "string" and type(arg) == "string" then
        return find(str, arg, 1, true) == 1
    end
end

function endswith(str, arg)
    if str and arg then
        return find(str, arg, #str - #arg + 1, true) == #str - #arg + 1
    end
end

--our trim from luautils ...
function trim(s)
    return (string.gsub(s, "^%s*(.-)%s*$", "%1"))
end

-- takes path and return the file extention or nil
function splitext(path)
  if not path or path == "" then return nil end

  t_parts = path:split("/")
  t_ext = t_parts[#t_parts]:split(".")
  return t_ext[2]

end

-- slice table
function slice(T, first, last, step)
  if not first then first = 1 end
  if not last then last = #T end
  if not step then step = 1 end

  local sliced = {}

  for i = first, last, step do
    sliced[#sliced+1] = T[i]
  end
  return sliced
end

function dict()
  return {}
end

function defaultdict(callable)
    local T = {}
    setmetatable(T, {
        __index = function(T, key)
            local val = rawget(T, key)
            if not val then
                rawset(T, key, callable())
            end
            return rawget(T, key)
        end
    })
    return T
end

function dump_table(handle, datatable)
    for k,v in pairs(datatable) do
        if type(v) == "table" then
            dump_table(handle, v)
        end
    end
end

function table_keys(T)
    local keys = {}
    local n = 0

    for k,v in pairs(T) do
      n = n + 1
      keys[n] = k
    end

    return keys
end

function table_length(T)
    if type(T) ~= "table" then return 0 end
    local count = 0
    for _ in pairs(T) do
        count = count + 1
    end
    return count
end


function md5(input)
    if type(input) ~= "string" then return nil end

    local _md5 = resty_md5:new()
    if not _md5 then return nil end

    local ok = _md5:update(input)
    if not ok then return nil end

    local digest = _md5:final()

    return digest:tohex()
end

function custom_response(request_map, action_params)
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
    if action_params["headers"] then response["headers"] = action_params["headers"] end
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
