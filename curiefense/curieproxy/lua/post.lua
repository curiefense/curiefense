-- commonlua/post.lua
-- Copyright (C) Anton heryanto.
-- module(..., package.seeall)

local cjson         = require "cjson"
local json_safe     = require "cjson.safe"
local luautils      = require "luautils"

local json_safe_loads   = json_safe.decode
local flatten           = luautils.flatten
local flatten_xml       = luautils.flatten_xml

local upload = require "resty.upload"
local table_new_ok, new_tab = pcall(require, "table.new")
local logger        = require "logger"


local open = io.open
local sub  = string.sub
local find = string.find
local byte = string.byte
local type = type
local tonumber = tonumber
local setmetatable = setmetatable
local random = math.random
local re_find = ngx.re.find
local read_body = ngx.req.read_body
local get_post_args = ngx.req.get_post_args
local var = ngx.var
local log = ngx.log
local WARN = ngx.WARN
local ERROR = ngx.ERROR
local prefix = ngx.config.prefix()..'logs/'
local now = ngx.now

local first = luautils.first

if not table_new_ok then
    new_tab = function(narr, nrec) return {} end
end


local _M = new_tab(0, 3)
local mt = { __index = _M }
_M.VERSION = '0.2.2'


local function tmp()
    return now() + random()
end


local function original(name)
    return name
end


function _M.new(self, opts)
    local ot = type(opts)
    opts = ot == 'string' and {path = opts} or ot == 'table' and opts or {}
    opts.path = type(opts.path) == 'string' and opts.path or prefix
    opts.chunk_size = tonumber(opts.chunk_size, 10) or 8192
    opts.name = type(opts.name) == 'function' and opts.name or opts.no_tmp
        and original or tmp
    return setmetatable(opts, mt)
end


local function decode_disposition(self, data)
    local needle = 'filename="'
    local needle_len = 10 -- #needle
    local name_pos = 18 -- #'form-data; name="'
    local last_quote_pos = #data - 1
    local filename_pos = find(data, needle)

    if not filename_pos then
        return sub(data,name_pos,last_quote_pos)
    end

    local field = sub(data,name_pos,filename_pos - 4)
    local name = sub(data,filename_pos + needle_len, last_quote_pos)
    if not name or name == '' then
        return
    end

    local fn = self.name
    local path = self.path
    local tmp_name = fn(name, field)
    local filename = path .. tmp_name
    local handler = open(filename, 'w+b')

    if not handler then
        log(WARN, 'failed to open file ', filename)
    end

    return field, name, handler, tmp_name
end


local function multipart(self)
    local chunk_size = self.chunk_size
    local form, e = upload:new(chunk_size)
    if not form then
        log(ERROR, 'failed to new upload: ', e)
        return
    end

    local m = { files = {} }
    local files = {}
    local handler, key, value
    while true do
        local ctype, res, er = form:read()

        if not ctype then
            log(ERROR, 'failed to read: ', er)
            return
        end

        if ctype == 'header' then
            local header, data = res[1], res[2]

            if header == 'Content-Disposition' then
                local tmp_name
                key, value, handler, tmp_name = decode_disposition(self, data)

                if handler then
                    files[key] = { name = value, tmp_name = tmp_name }
                end
            end

            if handler and header == 'Content-Type' then
                files[key].type = data
            end
        end

        if ctype == 'body' then
            if handler then
                handler:write(res)
            elseif res ~= '' then
                value = value and value .. res or res
            end
        end

        if ctype == 'part_end' then
            if handler then
                files[key].size = handler:seek('end')
                handler:close()
                if m.files[key] then
                    local nf = #m.files[key]
                    if nf > 0 then
                        m.files[key][nf + 1] = files[key]
                    else
                        m.files[key] = { m.files[key], files[key] }
                    end
                else
                    m.files[key] = files[key]
                end

            elseif key then
                -- handle array input, checkboxes
                -- handle one dimension array input
                -- name[0]
                -- user.name and user[name]
                -- user[0].name and user[0][name]
                -- TODO [0].name ?
                -- FIXME track mk
                local from, to = re_find(key, '(\\[\\w+\\])|(\\.)','jo')
                if from then
                    -- check 46(.)
                    local index = byte(key, from) == 46 and '' or
                        sub(key, from + 1, to - 1)
                    local name = sub(key, 0, from - 1)
                    local field
                    if #key == to then -- parse input[name]
                        local ix = tonumber(index, 10)
                        field = ix and ix + 1 or index
                        index = ''
                    else
                        -- parse input[index].field or input[index][field]
                        local ns = index == '' and 1 or 2
                        local ne = #key
                        if index ~= '' and byte(key, to + 1) ~= 46 then
                            ne = ne - 1
                        end
                        field = sub(key, to + ns, ne)
                        index = index == '' and index or (index + 1)
                    end

                    if type(m[name]) ~= 'table' then
                        m[name] = {}
                    end

                    if index ~= '' and type(m[name][index]) ~= 'table' then
                        m[name][index] = {}
                    end

                    if index ~= '' and m[name][index] then
                        m[name][index][field] = value -- input[0].name
                    else
                        m[name][field] = value
                    end

                elseif m[key] then
                    local mk = m[key]
                    if type(mk) == 'table' then
                        m[key][#mk + 1] = value
                    else
                        m[key] = { mk, value }
                    end
                else
                    m[key] = value
                end
                key = nil
                value = nil
            end
        end

        if ctype == 'eof' then break end

    end
    return m
end



function extract_multipart(input, boundary)
    local ret = {}
    boundary = '--' .. boundary

    -- last line will be "$boundary--"
    local end_at = string.find(input, boundary .. "--", 0, true)

    -- get out
    if not end_at then return ret end

    input = input:sub(0, end_at-1)
    local arg_sections = input:split(boundary)
    for _, section in ipairs(arg_sections) do

        for k,v in string.gmatch(section, [[Content%-Disposition: form%-data; name="([^\n]+)"(.*)]]) do
            local x = section:find([[filename="]], 0, true)
            if (not x) then
                ret[k] = v
            end
        end
    end

    return ret

end

function get_boundary()
    local get_headers = ngx.req.get_headers
    local match = string.match
    local header = get_headers()["content-type"]
    if not header then
        return nil
    end

    if type(header) == "table" then
        header = header[1]
    end

    local m = match(header, ";%s*boundary=\"([^\"]+)\"")
    if m then
        return m
    end

    return match(header, ";%s*boundary=([^\",;]+)")
end



-- proses post based on content type
function _M.read(self)
    local ctype = var.content_type

    if ctype and find(ctype, 'multipart') then
        -- return multipart(self)
        local boundary = get_boundary()
        local req_body = var.request_body
        if req_body and boundary then
            return extract_multipart(var.request_body, boundary)
        end

    end

    -- comment this out since we might call it on phases where it is not available
    -- log phase for instance.
    -- given we call read_body() is called upon starting of a session, it is safe to avoid
    -- it in here.

    -- read_body()

    if ctype and find(ctype, 'json') then
        local body = var.request_body
        if body then
            local as_json = json_safe_loads(body)

            -- if as_json then
            local body_type = type(as_json)

            if body_type == "userdata" then
                as_json = getmetatable(as_json)
            elseif body_type == "string" then
               as_json = { ["JSONSTR"] = as_json }
            elseif body_type == "number" then
               as_json = { ["JSONNUMBER"] = tostring(as_json) }
            elseif body_type == "boolean" then
               as_json = { ["JSONBOOL"] = tostring(as_json) }
            elseif body_type == "table" then
               -- array case
               if as_json[1] then
                   for k, v in ipairs(as_json) do
                       as_json[tostring(k)]=v
                       rawset(as_json,k, nil)
                   end
               end
               -- return as_json
            end
            if not as_json then
                as_json = { ["MalformedJSON"] = "(body)" }
            end

            local  new_table = {}
            flatten(as_json, new_table)
            return new_table
        end
        -- return body and json_safe_loads(body) or {}
    end

    if ctype and find(ctype, '/xml') then
        local body = var.request_body
        if body then
            local as_xml = flatten_xml(body)
            if type(as_xml) == "table" then
               return as_xml
            end
        end
    end

    local args = get_post_args()

    if type(args) == "table" then
        -->>>>> new JSON attempt <<<<<--
        -- try JSON once more, for implicit JSON posts
        local first_key, first_value = first(args)
        local as_json = json_safe_loads(first_key)

        if as_json and (type(first_value) == "boolean" or type(first_value) == "number" or type(first_value) == "string" )  then
            return as_json
        end
        -->>>>> / new JSON attempt <<<<<--
        return args
    else
        local ret = {}
        ret["unparsed"] = args
        return ret
    end
end


return _M
