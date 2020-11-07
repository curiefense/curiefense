module(..., package.seeall)

local cjson       = require "cjson"
local json_safe   = require "cjson.safe"
local socket      = require "socket"
local lfs         = require "lfs"

local preplists   = require "lua.preplists"

local gen_list_entries  = preplists.gen_list_entries
local json_decode       = json_safe.decode



-- global datasets

URLMap          = nil
ACLProfiles     = nil
WAFProfiles     = nil
WAFSignatures   = nil
ProfilingLists  = nil
LimitRules      = nil

MaxMindCountry  = nil
MaxMindASN      = nil

-- global enums

ACLNoMatch   = -1
ACLForceDeny = 0
ACLBypass    = 1
ACLAllowBot  = 2
ACLDenyBot   = 3
ACLAllow     = 4
ACLDeny      = 5

WAFPass = 1
WAFBlock = 0

ContainerID = ''

-- reload timer
last_reload_check_time = 0
last_reload_time = 0
--

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

function read_container_id()
    local id = read_file("/etc/hostname")
    if id then
        ContainerID = "container:" .. id:sub(1,-2) -- trim \n
    else
        ContainerID = 'container:unknown'
    end
end

function load_json_file(path)
    local data = read_file(path)
    if data then
        return json_decode(data)
    end
end

function direct_load(handle, path)
    handle:logDebug(string.format("loading %s", path))
    return load_json_file(path)
end

function load_and_reconstruct(handle, path)
    handle:logInfo(string.format("loading %s", path))
    local store = {}
    local json_map = direct_load(handle, path)

    for _, entry in ipairs(json_map) do
        handle:logDebug(string.format("loading %s from %s", entry.id, path))
        store[entry.id] = entry
    end

    return store
end

function load_and_reconstruct_acl(handle, path)
    handle:logInfo(string.format("loading %s", path))
    local store = {}
    local json_map = direct_load(handle, path)
    local acl_actions = {"allow", "allow_bot", "deny_bot", "bypass", "deny", "force_deny" }
    for _, json_acl in ipairs(json_map) do
        acl_entry = {
            ["id"] = json_acl.id,
            ["name"] = json_acl.name
        }

        for _, action in ipairs(acl_actions) do
            acl_entry[action] = {}
            for _, tag in ipairs(json_acl[action]) do
                acl_entry[action][tag] = 1
            end
        end

        store[json_acl.id] = acl_entry
    end

    return store
end

function new_waf_store( json_map )
    return {

        ["id"]                  = json_map["id"],
        ["name"]                = json_map["name"],
        ["ignore_alphanum"]     = json_map["ignore_alphanum"],

        ["max_header_length"]   = json_map["max_header_length"],
        ["max_cookie_length"]   = json_map["max_cookie_length"],
        ["max_arg_length"]      = json_map["max_arg_length"],

        ["max_headers_count"]   = json_map["max_headers_count"],
        ["max_cookies_count"]   = json_map["max_cookies_count"],
        ["max_args_count"]      = json_map["max_args_count"],

        ["args"]                = { ["names"] = {}, ["regex"] = {}},
        ["headers"]             = { ["names"] = {}, ["regex"] = {}},
        ["cookies"]             = { ["names"] = {}, ["regex"] = {}}

    }
end

function load_and_reconstruct_waf(handle, path)
    local json_map = direct_load(handle, path)
    local store = {}

    -- json_map_str = cjson.encode(json_map)
    -- handle:logDebug(string.format("WAF RECONSTRUCT - JSON MAP \n%s\n",json_map_str))

    local sections = { "args", "headers", "cookies" }
    local subsections = { "names", "regex" }
    for _, waf_profile in ipairs(json_map) do
        local waf_store = new_waf_store(waf_profile)
        for _, section in ipairs(sections) do
            handle:logDebug(string.format("WAF RECONSTRUCT, section %s ", waf_profile[section]))
            for _, subsection in ipairs(subsections) do
                handle:logDebug(string.format("WAF RECONSTRUCT, section %s subsection %s", section, subsection))
                for _, entry in ipairs(waf_profile[section][subsection]) do
                    waf_store[section][subsection][entry.key] = {
                      ["reg"] = entry.reg,
                      ["restrict"]  = entry.restrict,
                      ["exclusions"] = entry.exclusions
                    }
                end
            end
        end
        store[waf_store.id] = waf_store
    end

    return store
end

function load_and_reconstruct_taglist(handle, path)
    local store = {}

    local json_map = direct_load(handle, path)

    for _, list in ipairs(json_map) do
        handle:logDebug(string.format("LART -- ID:Name active? %s:%s", list.id, list.name, list.active))
        if list.active then
            local tag_list = gen_list_entries(list, handle)
            handle:logDebug(
                string.format(
                    "LART -- ID:Name %s:%s tags %s singles %s negate_singles %s pairs %s negate_pairs %s iprange %s negate_iprange %s",
                    tag_list.id,
                    tag_list.name,
                    tag_list.tags,
                    #(tag_list.singles or {}),
                    #(tag_list.negate_singles or {}),
                    #(tag_list.pairs or {}),
                    #(tag_list.negate_pairs or {}),
                    #(tag_list.iprange or {}),
                    #(tag_list.negate_iprange or {})
                )
            )
            store[tag_list.id] = tag_list
        end
    end

    return store
end

local dl  = direct_load
local lr  = load_and_reconstruct
local lra = load_and_reconstruct_acl
local lrw = load_and_reconstruct_waf
local lrt = load_and_reconstruct_taglist

function reload(handle)
    handle:logDebug("RELOAD CONFIG ENTERED")

    ACLProfiles     = lr(handle, "/config/current/config/json/acl-profiles.json")
    WAFProfiles     = lrw(handle,"/config/current/config/json/waf-profiles.json")
    WAFSignatures   = dl(handle, "/config/current/config/json/waf-signatures.json")
    URLMap          = dl(handle, "/config/current/config/json/urlmap.json")
    LimitRules      = lr(handle, "/config/current/config/json/limits.json")
    ProfilingLists  = lrt(handle,"/config/current/config/json/profiling-lists.json")

    for id, entry in pairs(LimitRules) do
        handle:logDebug(string.format("globals lua -- LimitRules %s %s", id, entry.name))
    end

    handle:logDebug("RELOAD CONFIG DONE")
end

function maybe_reload(handle)
    handle:logDebug("MAYBE_RELOAD CONFIG ENTERED")
    local fname
    local curtime = os.time()

    if lfs.attributes("/config/current").change > last_reload_time then
        last_reload_time = curtime
        ACLProfiles     = lra(handle, "/config/current/config/json/acl-profiles.json")
        WAFProfiles     = lrw(handle, "/config/current/config/json/waf-profiles.json")
        WAFSignatures   = dl(handle,  "/config/current/config/json/waf-signatures.json")
        URLMap          = dl(handle,  "/config/current/config/json/urlmap.json")
        LimitRules      = lr(handle,  "/config/current/config/json/limits.json")
        for id, entry in pairs(LimitRules) do
            handle:logDebug(string.format("globals lua -- LimitRules %s %s", id, entry.name))
        end
        ProfilingLists  = lrt(handle, "/config/current/config/json/profiling-lists.json")
    end

    handle:logDebug("MAYBE_RELOAD CONFIG DONE")
end

function init(handle)
    handle:logDebug("INIT ENTERED")

    if os.time() - last_reload_check_time > 5 then
        read_container_id()
        maybe_reload(handle)
        last_reload_check_time = os.time()
     end
    handle:logDebug("INIT LOADING ENDS")
end

function print_collection(handle, collection)
    for k, v in pairs(collection) do
        handle:logDebug(string.format("%s - %s", k, v))
    end
end

-- function x()
--     local objURLMap          = { mtime = 0, val = nil, path="/config/json/urlmap.json",               load=dl }
--     local objACLProfiles     = { mtime = 0, val = nil, path="/config/json/acl-profiles.json",         load=lr }
--     local objWAFProfiles     = { mtime = 0, val = nil, path="/config/json/waf-profiles.json",         load=lr }
--     local objWAFSignatures   = { mtime = 0, val = nil, path="/config/json/waf-signatures.json",       load=dl }

--     function load_if_needed(obj, handle)
--         while (true)
--         do
--             d = lfs.attributes(obj.path)
--             if d ~= nil then
--                 if d.modification > obj.mtime then
--                     obj.val = obj.load(handle, obj.path)
--                 end
--             end
--             if obj.val == nil then
--                 handle:logWarning(string.format("Config file [%s] not found. Must wait for it.", path))
--                 socket.sleep(1)
--             else
--                 return obj.val
--             end
--         end
--     end
-- end
