module(..., package.seeall)

local utils     = require "lua.utils"

local re_match  = utils.re_match


function hashkey(key)
    local hashed = md5(key)
    return hashed or key
end


function should_exclude(request_map, exclude_set)
    local exclude = false
    for section, entries in pairs(exclude_set) do
        for name, value in pairs(entries) do
            if request_map[section][name] then
                if re_match(request_map[section][name], value) then
                    exclude = true
                    break
                end
            end
        end
    end
    return exclude
end

function should_include(request_map, include_set)
    local include = true
    for section, entries in pairs(include_set) do
        for name, value in pairs(entries) do
            if request_map[section][name] then
                if not re_match(request_map[section][name], value) then
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


--[[

this:

a = {}
for i=1,100000 do table.insert(a, tostring(i)) end
table.concat(a, '')

is faster than:

a = ''
for i=1,100000 do a = a .. tostring(i) end

]]--

function build_key(request_map, key_set, entry_id, entry_name)
    local key = {}

    for _, entry in ipairs(key_set) do
        local section, name = next(entry)
        if section and name then
            local entry = request_map[section][name]
            if entry then
                table.insert(key, tostring(entry))
            else
                return false
            end
        else
            return false
        end
    end

    key = string.format("%s%s%s", entry_name, entry_id, table.concat(key, ''))

    return hashkey(key)

end


function match_tags (tags, request_map)
    local request_map_tags = request_map.attrs.tags
    for _, tag in ipairs(tags) do
        if request_map_tags[tag] then
            return true
        end
    end
end
