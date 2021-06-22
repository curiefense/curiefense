module(..., package.seeall)

local resty_post = require "lua.post"
local utils      = require "lua.utils"

local new_request_map = utils.new_request_map
local get_geo_info = utils.get_geo_info



function map_headers(ngx)
    return ngx.req.get_headers()
end

function map_cookies(ngx, headers)
    local ngx_ctx = ngx.ctx
    local nreq = ngx.req

    local cookies = {}

    local kv_spliter = function (line) return string.split(line, "=") end
    local cookies_str = headers["Cookie"]
    -- if a client sends several cookie headers... join them before running split..

    if type(cookies_str) == "table" then
        cookies_str = join(cookies_str, "; ")
    end

    if cookies_str then
        local cookies_lines = string.split(cookies_str, "; ")
        for k,v in pairs(cookies_lines) do
            if v then
                cookie = kv_spliter(v)
                local key = cookie[1]
                if key then
                    cookies[key] = cookie[2]
                end
            end
        end
    end

    return cookies

end

function map_ip(ngx, map)
    -- assuming real_ip module is configured
    local client_addr = ngx.var.remote_addr

    map = get_geo_info(map, client_addr)

    return map

end

function map_args(ngx)
    local ngx_ctx = ngx.ctx

    local nreq = ngx.req
    local nvars = ngx.var
    local args_table = {}

    local uri_only = {COPY=true,DELETE=true,GET=true,HEAD=true,MKCOL=true,MOVE=true,TRACE=true,UNLOCK=true}
    local uri_and_body = {POST=true,PROPFIND=true,PROPPATCH=true,PUT=true,LOCK=true,CONNECT=true,OPTIONS=true}

    if uri_only[nvars.request_method] then
        args_table = nreq.get_uri_args()

    elseif uri_and_body[nvars.request_method] then

        local _uri_args = nreq.get_uri_args()
        -- only if nreq.get_body_file returns nil, read it from in-memory
        -- see more at https://groups.google.com/forum/#!topic/openresty/7jVJ-M65vtU
        if not nreq.get_body_file() then
            local post = resty_post:new()
            local _post_body_args = post:read()

            if type(_post_body_args) ~= "table" then
                _post_body_args = { [ "unnamed_arg" ] = tostring(_post_body_args) }
            end
            if type(_uri_args) == "table" then
                for k, v in pairs(_uri_args) do
                    _post_body_args[k] = v
                end
            end
            args_table = _post_body_args
        else
            args_table = {}
        end
    end

    return args_table
end

function map_request(ngx)

    local headers = map_headers(ngx)
    local cookies = map_cookies(ngx)
    local map_args = map_args(ngx)

    local map = new_request_map()
    local map_ip = map_ip(ngx, map)

    local headers = handle:headers()
    local metadata = handle:metadata()
    local map = new_request_map()

    map.handle = handle

    map_headers(headers, map)
    map_metadata(metadata, map)
    map_ip(headers, metadata, map)
    map_args(handle, map)

    map.attrs.session_sequence_key = string.format(
        "%s%s%s",
        map.attrs.method,
        map.headers.host or map.attrs.authority,
        map.attrs.path
    )

    return map
end