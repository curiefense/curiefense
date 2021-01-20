-- commonlua/limit.lua
module(..., package.seeall)

local os        = require "os"
local redis     = require "lua.redis"
local utils     = require "lua.utils"
local globals   = require "lua.globals"
local cjson     = require "cjson"

local json_encode   = cjson.encode
local limit_ban_hash = 'limit-ban-hash'

local md5             = utils.md5
local re_match        = utils.re_match
local tag_request     = utils.tag_request
local deny_request    = utils.deny_request
local custom_response = utils.custom_response

--- all functions that access redis, starts with redis_

function hashkey(key)
    local hashed = md5(key)
    return hashed or key
end

function gen_ban_key( key )
    return hashkey(limit_ban_hash .. key)
end

function redis_connection()
    hostname = os.getenv("REDIS_HOST")
    if hostname == nil then
        hostname = "redis"
    end
    return redis.connect(hostname, 6379)
end

function should_exclude(request_map, limit_set)
    local exclude = false
    for section, entries in pairs(limit_set["exclude"]) do
        for name, value in pairs(entries) do
            if request_map[section][name] then
                if re_match(request_map[section][name], value) then
                    -- request_map.handle:logDebug(string.format("limit excluding (%s)[%s]=='%s'", section, name, value))
                    exclude = true
                    break
                end
            end
        end
    end
    return exclude
end

function should_include(request_map, limit_set)
    local include = true
    for section, entries in pairs(limit_set["include"]) do
        for name, value in pairs(entries) do
            if request_map[section][name] then
                if not re_match(request_map[section][name], value) then
                    -- request_map.handle:logDebug(string.format("limit NOT including (%s)[%s]=='%s'", section, name, value))
                    include = false
                    break
                end
            else
                include = false
                break
            end
        end
    end
    return include
end


-- this function will extract all rules with limit 0 first, then
-- sort remaining rules by $.limit in a descending order
function sorted_limit_rules( limit_rules, limit_ids )
    local zeros = {}
    local others = {}

    -- split
    for _, limit_id in ipairs(limit_ids) do
        local rule = limit_rules[limit_id]
        if rule then
            local limit = tonumber(rule.limit)
            if limit == 0 then
                table.insert(zeros, rule)
            else
                table.insert(others, rule)
            end
        end
    end
    -- sort
    local sorter = function( a, b) return tonumber(a.limit) > tonumber(b.limit) end
    table.sort(others, sorter)

    -- combine
    local combined = {}
    for _, entry in ipairs(zeros) do
        table.insert(combined, entry)
    end
    for _, entry in ipairs(others) do
        table.insert(combined, entry)
    end

    return combined
end

function check( request_map, limit_ids, url_map_name)
    local limit_rules = globals.LimitRules
    -- request_map.handle:logDebug(string.format("redis-limit unsorted %s", json_encode(limit_rules)))
    local sorted_rules = sorted_limit_rules(limit_rules, limit_ids)

    -- request_map.handle:logDebug(string.format("redis-limit sorted %s", json_encode(sorted_rules)))
    for _, rule in ipairs(sorted_rules) do
        check_request(request_map, rule, url_map_name)
    end

end

function unsorted_check(request_map, limit_ids, url_map_name)
    local limit_rules = globals.LimitRules

    -- for id, entry in pairs(limit_rules) do
    -- --     request_map.handle:logDebug(string.format("-- limit lua LimitRules %s %s", id, entry.name))
    -- end

    for _, limit_id in ipairs(limit_ids) do
        local limit_set = limit_rules[limit_id]
        -- -- request_map.handle:logDebug(string.format("limit check first loop currently on %s -- got %s within %s", limit_id, limit_set.name, limit_set))
        check_request(request_map, limit_set, url_map_name)
    end
end

function build_key(request_map, limit_set, url_map_name)
    local handle = request_map.handle
    local key = ''
    for _, entry in ipairs(limit_set["key"]) do
        -- handle:logDebug(string.format("limit build_key -- iterate key entries %s", _))
        local section, name = next(entry)
        -- handle:logDebug(string.format("limit build_key -- iterate key entrie's section %s, name %s", section, name))
        if section and name then
            local entry = request_map[section][name]
            -- handle:logDebug(string.format("limit build_key -- iterate key request_map[section][name] %s", request_map[section][name]))
            if entry then
                key = key .. entry
            else
                -- handle:logDebug(string.format("limit build_key -- falsifying at section %s, name %s", section, name))
                return false
            end
        else
            -- handle:logDebug(string.format("limit build_key -- falsifying at %s", _))
            return false
        end
    end
    key = string.format("%s%s%s", url_map_name, limit_set.id, key)
    -- handle:logDebug(string.format("limit build_key -- key %s", key))
    request_map.handle:logDebug(string.format("limit REQUEST KEY (%s)[%s]=='%s'", url_map_name, limit_set.id, key))
    local hashed_key = hashkey(key)
    request_map.handle:logDebug(string.format("limit REQUEST KEY hashed (%s)", hashed_key))
    return hashed_key
end

function check_request(request_map, limit_set, url_map_name)
    if limit_set then
        -- request_map.handle:logDebug("redis-limit check_request starting.")

        -- request_map.handle:logDebug("redis-limit check_request should exclude?")
        if      should_exclude(request_map, limit_set) then return false end
        -- request_map.handle:logDebug("redis-limit check_request should include?")
        if not  should_include(request_map, limit_set) then return false end

        -- request_map.handle:logDebug("check_request got here, meanning, shoud include. hence, tagging matching rule")
        -- every matching ratelimit rule is tagged by name
        tag_request(request_map, limit_set['name'])

        local key = build_key(request_map, limit_set, url_map_name)
        -- request_map.handle:logDebug(string.format("check_request key built -- %s", key))
        if not key then return false end

        local pairing_value = false
        local pair_name, pair_value = next(limit_set['pairwith'])
        if pair_name then
            pairing_value = request_map[pair_name][pair_value]
            -- request_map.handle:logDebug(string.format("redis-limit pair builder %s %s %s", pair_name, pair_value, pairing_value))
        end

        local limit = tonumber(limit_set.limit)
        local ttl = tonumber(limit_set.ttl)

        if limit == 0 then
            -- request_map.handle:logDebug(string.format("limit zero - reacting"))
            limit_react(request_map, limit_set.name, limit_set.action, key)
        end

        local reason = limit_set.name

        -- request_map.handle:logDebug(string.format("check_request start counting -- limit %s ttl %s reason %s", limit, ttl, reason))
        -- banned ?
        local ban_key = gen_ban_key(key)

        if redis_is_banned(ban_key) then
            request_map.handle:logDebug(string.format("redis-limit KEY is BANNED", pair_name, pair_value, pairing_value))

            if limit_set.action and limit_set.action.params then
                limit_react(request_map, limit_set.name, limit_set.action.params.action, ban_key, ttl)
            else
                limit_react(request_map, limit_set.name)
            end
        end

        request_map.handle:logDebug(string.format("not banned key - going with standard limit check"))
        local xert = redis_check_limit(request_map, key, limit, ttl, pairing_value)
        request_map.handle:logDebug(string.format("redis_check_limit xert  came back with %s", xert))
        if xert == 503 then
            limit_react(request_map, limit_set.name, limit_set.action, key, ttl)
        end
    end
end

function redis_check_limit(request_map, key, threshold, ttl, set_value)
    local retval = 200
    local redis_conn = redis_connection()

    if not redis_conn then
        return retval
    end

    if not set_value then
        retval = redis_check_simple(request_map, redis_conn, key, threshold, ttl)
    else
        retval = redis_check_set(request_map, redis_conn, key, threshold, set_value, ttl)
    end

    -- local ok, err = redis_conn:set_keepalive(max_idle_timeout, redis_pool_size)
    return retval
end

function redis_check_simple(request_map, redis_conn, key, threshold, ttl)
    local handle = request_map.handle
    local current = 0
    local force_expire = false

    local result = redis_conn:pipeline(
        function(pipe)
            -- pipe:multi()
            pipe:incr(key)
            pipe:ttl(key)
        end
    )

    -- handle:logDebug(string.format("limit redis_check_simple -- type(%s), [%s]", type(result), result))
    if type(result) == "table" then
        current = result[1]
        expire = result[2]

        -- handle:logDebug(string.format("limit redis_check_simple -- current (%s), expire[%s]", current, expire))

        if "userdata: NULL" == tostring(current) then
            current = 0
        else
            current = tonumber(current)
        end

        if "userdata: NULL" == tostring(expire) then
            expire = -1
        else
            expire = tonumber(expire)
        end

        if expire < 0 then
            value, err = redis_conn:expire(key, ttl)
        end

        if current ~= nil and current > threshold then
            return 503
        else
            -- handle:logDebug(string.format("limit --- %s < %s", current, threshold))
            return 200
        end
    else
        -- handle:logDebug(string.format("limit --- not a table, 200 is the answer"))
        return 200
    end
end

function redis_check_set(request_map, redis_conn, key, threshold, set_value, ttl)
    local current = 0
    set_value = md5(set_value)

    local result = redis_conn:pipeline(
        function(pipe)
            -- pipe:multi()
            pipe:sadd(key, set_value)
            pipe:scard(key)
            pipe:ttl(key)
        end
    )

    request_map.handle:logDebug("redis_check_set result: %s", json_encode(result))

    if type(result) == "table" then
        current = result[2]
        expire = result[3]

        request_map.handle:logDebug("redis_check_set current: %s, expire %s", current, expire)

        if "userdata: NULL" == tostring(current) then
            current = 0
        else
            current = tonumber(current)
        end

        if "userdata: NULL" == tostring(expire) then
            expire = -1
        else
            expire = tonumber(expire)
        end

        if expire < 0 then
            value, err = redis_conn:expire(key, ttl)
        end

        if current ~= nil and current > threshold then
            return 503
        else
            local value, err = redis_conn:sadd(key, set_value)
            if not value then
                error("failed expanding set: " .. tostring(err))
                return -1
            end
            return 200
        end
    else
        return 200
    end
end

function limit_react(request_map, rulename, action, key, ttl)

    local handle = request_map.handle
    if not action or action.type == "default" then
        action = {
            ["type"] = "default",
            ["params"] = {
                ["status"] = "503",
                ["block_mode"] = true
            }
        }
    end

    if not action.params then action.params = {} end

    action.params.reason = { initiator = "rate limit", reason = rulename}
    -- handle:logDebug(string.format("limit react --- action %s", action.type))
    if action.type == "monitor" then
        return

    -- elseif action.type == "default" then
    --     deny_request(request_map, reason, true, "503")

    elseif action.type == "ban" then
        ttl = tonumber(action.params.ttl)
        redis_ban_key(gen_ban_key(key), ttl)
        -- recursive call
        limit_react(request_map, rulename, action.params.action)

    else
        action.block_mode = (action.type ~= "request_header")
        custom_response(
            request_map,
            action.params
        )

    end

    -- if action.type == "response" then
    --     if action.params.headers then
    --         for name, value in ipairs(action.params.headers) do
    --             ngx.header[name] = value
    --         end
    --     end
    --     deny_request(request_map, reason, true, action.params.status, action.params.headers, action.params.content)
    -- end

    -- if action.type == "challenge" then
    --     -- request_map.handle:logDebug("action.type == 'challenge'")
    -- end

    -- if action.type == "redirect" then
    --     deny_request(request_map, reason, true, action.params.status, { location = action.params.location }, '')
    -- end

    -- if action.type == "request_header" then
    --     deny_request(request_map, reason, false, nil, action.params.headers, nil)
    -- end

    --[[
        for action.type == "monitor" do nothing
        tags already taken care of at check_request right after
        should_exclude and should_include
    ]]--

end

function redis_is_banned(key)
    local redis_conn = redis_connection()
    local is_banned = redis_conn:get(key)
    if "userdata: NULL" == tostring(is_banned) then
        is_banned = false
    end
    return is_banned
end

function redis_ban_key(key, ttl)
    local redis_conn = redis_connection()

    redis_conn:set(key, "1")
    redis_conn:expire(key, ttl)
end
