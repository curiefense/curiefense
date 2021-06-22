module(..., package.seeall)

-- parse request body args
-- XML, JSON, form-url-encoded or multipart.

-- Copyright (C) Anton heryanto.
-- module(..., package.seeall)

local cjson         = require "cjson"
local json_safe     = require "cjson.safe"
local utils         = require "lua.utils"
local curiefense    = require "curiefense"

local find          = string.find
local gsub          = string.gsub
local char          = string.char
local byte          = string.byte
local format        = string.format
local match         = string.match
local gmatch        = string.gmatch

local concat        = table.concat
local insert        = table.insert

local json_decode   = json_safe.decode

local flatten       = utils.flatten

local urldecode     = curiefense.decodeurl
local urlencode     = curiefense.encodeurl

function parse_body(request_map)
    local ctype = request_map.headers['content-type']
    local json_mode = ctype and ctype:find("/json")
    local buffer = request_map.handle:body()
    local length = buffer:length()
    local body = buffer:getBytes(0, length)

    -- request_map.handle:logDebug(string.format("Request body: %s", body))
    if not body then
        return {}
    else
        if json_mode then
            -- request_map.handle:logDebug("JSON parsing")
            return flatten(json_decode(body))
        else
            -- request_map.handle:logDebug("URL parsing")
            return parse_query(body)
        end
    end
end

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
