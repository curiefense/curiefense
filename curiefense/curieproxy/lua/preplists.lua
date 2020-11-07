-- preplists.lua
module(..., package.seeall)

local utils       = require "lua.utils"
local iputils     = require "lua.iputils"
local rangesbtree = require "lua.rangesbtree"

local cjson       = require "cjson"
local json_safe   = require "cjson.safe"


local dict          = utils.dict
local defaultdict   = utils.defaultdict
local slice         = utils.slice

local parse_cidr    = iputils.parse_cidr
local build_ranges_lists = rangesbtree.build_ranges_lists

function gen_masterdict(lst)

    return {
        -- copy essentials as is.
        [ "id" ]               = lst["id"],
        [ "name" ]             = lst["name"],
        [ "entries_relation" ] = lst["entries_relation"],
        [ "tags" ]             = lst["tags"],

        [ "singles" ]          = defaultdict(dict),
        [ "negate_singles" ]   = defaultdict(dict),

        [ "pairs" ]            = defaultdict(dict),
        [ "negate_pairs" ]     = defaultdict(dict),

        [ "iprange" ]          = {},
        [ "negate_iprange" ]   = {}
    }

end

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
    if #data > 1 then
        return data[2]
    else
        return nil
    end
end

function gen_list_entries(lst, handle)
    local masterdict = gen_masterdict(lst)

    for _, entry in ipairs(lst["entries"]) do
        category, data = entry[1], slice(entry, 2)
        handle:logDebug(string.format("category: %s", category))
        -- pairs
        if category:within("args cookies headers") then
            local pairk, pairv, pairannotation = data[1][1], data[1][2], get_annotation(data)
            local mastercategory, pairv = categorize_pairs(pairv)
            handle:logDebug(string.format("K %s V %s A %s", pairk, pairv, pairannotation))
            masterdict[mastercategory][category][pairk] = { pairv, pairannotation }
        -- singles
        else

            if category:within("path query uri asn country method") then
                -- negate vs standard
                local mastercategory, key = categorize_singles(data[1])
                handle:logDebug(string.format("MC %s C %s K %s D %s",
                    mastercategory, category, key, data))
                -- store
                masterdict[mastercategory][category][key] = get_annotation(data)
            else
                if category == "ip" then
                    local address = data[1]
                    handle:logDebug(string.format("ADDRESS %s", address))
                    -- single address
                    if not ("/"):within(address) or address:endswith("/32") then
                        address = address:replace("/32", "")
                        mastercategory, key = categorize_singles(address)

                        handle:logDebug(string.format("MC %s C %s K %s D %s",
                            mastercategory, category, key, data))

                        -- store
                        masterdict[mastercategory][category][key] = get_annotation(data)
                    -- range
                    else
                        local cidr, mastercategory = data[1], "iprange"

                        if cidr:startswith("!") then
                            mastercategory = "negate_iprange"
                            cidr = cidr:sub(2)
                        end

                        local start_addr, end_addr = parse_cidr(cidr)
                        -- [[start,end], annotation]
                        elem =  { {start_addr, end_addr}, get_annotation(data) }
                        handle:logDebug(string.format("MC %s CIDR %s S %s E %s EL %s",
                            mastercategory, cidr, start_addr, end_addr, elem))

                        table.insert(masterdict[mastercategory], elem)
                    end
                end
            end
        end
    end
    if #masterdict['iprange'] > 0 then
        masterdict['iprange'] = build_ranges_lists(masterdict['iprange'])
    end
    handle:logDebug(string.format("MASTER DICT -- \n%s\n",json_safe.encode(masterdict)))
    return masterdict
end
