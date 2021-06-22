-- preplists.lua
module(..., package.seeall)

local utils       = require "lua.utils"

local curiefense     = require "curiefense"

local cjson       = require "cjson"
local json_safe   = require "cjson.safe"


local dict          = utils.dict
local defaultdict   = utils.defaultdict
local slice         = utils.slice
local table_length  = utils.table_length


function categorize_singles(key)
    local mastercategory = "singles"
    if key:startswith("!") then
        mastercategory = "negate_singles"
    end
    return mastercategory, key
end

function categorize_pairs(pairv)
    local mastercategory = "pairs"
    if pairv:startswith("!") then
        mastercategory = "negate_pairs"
        pairv = pairv:sub(2)
    end
    return mastercategory, pairv
end

function get_annotation(data)
    if table_length(data) > 1 then
        return data[2]
    else
        return nil
    end
end


function gen_section_dict(section)

    local masterdict = {
        [ "singles" ]          = defaultdict(dict),
        [ "negate_singles" ]   = defaultdict(dict),

        [ "pairs" ]            = defaultdict(dict),
        [ "negate_pairs" ]     = defaultdict(dict),

        [ "iprange" ]          = curiefense.new_ip_set(),
        [ "negate_iprange" ]   = curiefense.new_ip_set(),
    }

    for _, entry in ipairs(section["entries"]) do
        category, data = entry[1], slice(entry, 2)
        -- pairs
        if category:within("args cookies headers") then
            local pairk, pairv, pairannotation = data[1][1], data[1][2], get_annotation(data)
            local mastercategory, pairv = categorize_pairs(pairv)
            masterdict[mastercategory][category][pairk] = { pairv, pairannotation }
        -- singles
        else

            if category:within("path query uri asn country method") then
                -- negate vs standard
                local mastercategory, key = categorize_singles(data[1])
                -- store
                masterdict[mastercategory][category][key] = get_annotation(data)
            else
                if category == "ip" then
                    local address = data[1]
                    -- single address
                    if not ("/"):within(address) or address:endswith("/32") then
                        address = address:replace("/32", "")
                        mastercategory, key = categorize_singles(address)

                        -- store
                        masterdict[mastercategory][category][key] = get_annotation(data)
                    -- range
                    else
                        local cidr, mastercategory = data[1], "iprange"

                        if cidr:startswith("!") then
                            mastercategory = "negate_iprange"
                            cidr = cidr:sub(2)
                        end

                        local annotation = get_annotation(data) or mastercategory
                        masterdict[mastercategory]:add(cidr, annotation)
                    end
                end
            end
        end
    end

    return masterdict

end

function gen_list_entries(lst, handle)

    local section_gate = lst.rule.relation
    local sections = lst.rule.sections

    local rule = {
        ["relation"] = lst.rule.relation,
        ["sections"]  = {}
    }

    for _, section in ipairs(sections) do
        table.insert(rule.sections, {
            [ "relation" ] = section.relation,
            [ "entries" ] = gen_section_dict(section)
        })
    end

    return {
        ["id"] = lst["id"],
        ["name"] = lst["name"],
        ["active"] = lst["active"],
        ["tags"] = lst["tags"],
        ["rule"] = rule
    }

end
