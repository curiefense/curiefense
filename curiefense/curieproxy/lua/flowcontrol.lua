module(..., package.seeall)

local globals       = require "lua.globals"
local utils         = require "lua.utils"
local redisutils    = require "lua.redisutils"
local sessionutils  = require "lua.sessionutils"
local cjson         = require "cjson"

local json_encode   = cjson.encode
local buildkey      = sessionutils.buildkey
local match_tags    = sessionutils.match_tags

local list_length   = redisutils.list_length
local list_push_ttl = redisutils.list_push_ttl

local build_key     = redisutils.build_key
local re_match      = utils.re_match

function validate_flow(session_sequence_key, flow, redis_key, request_map)
    local sequence = flow.sequence
    local seq_len = #sequence
    local handle = request_map.handle
    local last_entry = sequence[seq_len]
    local listlen = list_length(redis_key)

    if session_sequence_key == last_entry.key then
        if listlen == seq_len or (listlen + 1 == seq_len) then
            handle:logDebug(string.format('flowcontrol listlen == seq_len or (listlen + 1 == seq_len) RETURNING TRUE'))
            return true
        else
            handle:logDebug(string.format('flowcontrol listlen == seq_len or (listlen + 1 == seq_len) RETURNING FALSE'))
            return false
        end
    end

    for idx=seq_len-1, 1, -1 do
        local seq_entry = sequence[idx]
        if seq_entry.key == session_sequence_key then
            handle:logDebug(string.format("flowcontrol seq_entry.key %s idx %s", seq_entry.key, idx))
            if idx-1 == listlen then
                handle:logDebug(string.format("flowcontrol pushing to redis %s %s", redis_key, session_sequence_key))
                list_push_ttl(redis_key, session_sequence_key, flow.ttl)
            end
        end
    end

    return true
end

function request_match_sequence_entry(flow, session_sequence_key, request_map)
    local handle = request_map.handle
    local sections = { "headers", "cookies", "args" }
    local sequence_entry = nil

    local sequence = flow.sequence
    for _, entry in ipairs(sequence) do
        handle:logDebug(string.format("flowcontrol entry -- %s", json_encode(entry)))
        handle:logDebug(string.format("flowcontrol entry.key %s, session_sequence_key %s", entry.key, session_sequence_key))
        if entry.key == session_sequence_key then
            sequence_entry = entry
            handle:logDebug("flowcontrol request_match_sequence_entry FOUND ENTRY")
            break
        end
    end

    if sequence_entry then
        for _, section in ipairs(sections) do
            local sequence_entry_section = sequence_entry[section]
            for name, value in pairs(sequence_entry_section) do
                handle:logDebug(string.format("flowcontrol request_match_sequence_entry section %s name %s value %s",
                    section, name, value))
                if section == 'headers' and name == 'host' then
                    handle:logDebug("flowcontrol request_match_sequence_entry SKIP HOST HEADER")
                else
                    handle:logDebug(string.format("flowcontrol request_match_sequence_entry COMPARE %s with %s", request_map_value, value))
                    if not request_map[section][name] then
                        return false
                    end
                    local request_map_value = request_map[section][name]
                    -- handle:logDebug(string.format("flowcontrol request_match_sequence_entry COMPARE %s with %s", request_map_value, value))
                    if not re_match(request_map_value, value) then
                        return false
                    end
                end
            end
        end
    end
    return true
end

function check(request_map)
    local handle = request_map.handle

    local flow_control_db = globals.FlowControl
    local session_sequence_key = request_map.attrs.session_sequence_key

    if flow_control_db then
        for _, flow in ipairs(flow_control_db) do
            -- this request within a given element of the sequence
            if flow.sequence_keys[session_sequence_key] then
                if request_match_sequence_entry(flow, session_sequence_key, request_map) then
                    local should_exclude = match_tags(flow.exclude, request_map)
                    handle:logDebug(string.format("flowcontrol should_exclude? %s", should_exclude))
                    if not should_exclude then
                        local should_include = (#flow.include == 0) or match_tags(flow.include, request_map)
                        handle:logDebug(string.format("flowcontrol should_include? %s", should_include))
                        if should_include then
                            local redis_key = build_key(request_map, flow.key, flow.id, flow.name)
                            local valid = validate_flow(session_sequence_key, flow, redis_key, request_map)
                            if not valid then
                                return flow.action
                            end
                        end
                    end
                else
                    handle:logDebug('flowcontrol not matching HCA -- skip')
                    return false
                end
            end
        end
    end
    return false
end

--[===[

[
    {
        "exclude": [],
        "include": [
            "all"
        ],
        "name": "Flow Control Example",
        "key": [
            {
                "attrs": "ip"
            }
        ],
        "sequence": [
            {
                "method": "GET",
                "uri": "/login",
                "cookies": {},
                "headers": {
                    "host": "www.example.com"
                },
                "args": {}
            },
            {
                "method": "POST",
                "uri": "/login",
                "cookies": {},
                "headers": {
                    "host": "www.example.com"
                },
                "args": {}
            }
        ],
        "active": true,
        "notes": "New Flow Control Notes and Remarks",
        "action": {
            "type": "default"
        },
        "ttl": 60,
        "id": "c03dabe4b9ca"
    }
]


Flow controls are globals
hence can cause huge performance hurdle.
to minimize the risk, given Host, Method Path are mandatory
    map for each request those for quick relevant check

    only if match, start the actual checks with optionally other entries matching (headers, attrs)
    and redis state etc.

In other words, given the above example
[
    {
        "exclude": [],
        "include": ["all"],
        "name": "Flow Control Example",
        "key": [ { "attrs": "ip" } ],
        "sequence": [
            {
                "key": "GETwww.example.com/login",
                "method": "GET",
                "uri": "/login",
                "cookies": {},
                "headers": { "host": "www.example.com" },
                "args": {}
            },
            {
                "key": "POSTwww.example.com/login",
                "method": "POST",
                "uri": "/login",
                "cookies": {},
                "headers": { "host": "www.example.com" },
                "args": {}
            }
        ],
        "active": true,
        "notes": "New Flow Control Notes and Remarks",
        "action": { "type": "default" },
        "ttl": 60,
        "id": "c03dabe4b9ca",
        "sequence_keys": {
            "GETwww.example.com/login": 1,
            "POSTwww.example.com/login": 1
        }
    }
]

chronological order matter to some extent
sequence matter

    if not should exclude and should include

        scan sequence from end to start

        if match the last entry of the sequence
            redis LIST should be at length of (sequence -1 or sequence).

        for each item in the seq
            if match item
                if redis LIST length  equals to index - 1
                    add to redis

]===]--
