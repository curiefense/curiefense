--[[
source: https://github.com/hamishforbes/lua-resty-iputils/blob/master/lib/resty/iputils.lua

]]
module(..., package.seeall)

_VERSION = '0.3.1'

local bit        = require("bit")

local ipairs     = ipairs
local tonumber   = tonumber
local tostring   = tostring
local type       = type

local byte       = string.byte
local str_find   = string.find
local str_sub    = string.sub

local lshift     = bit.lshift
local band       = bit.band
local bor        = bit.bor
local xor        = bit.bxor

-- Precompute binary subnet masks...
local bin_masks = {}
for i=0,32 do
    bin_masks[tostring(i)] = lshift((2^i)-1, 32-i)
end
-- ... and their inverted counterparts
local bin_inverted_masks = {}
for i=0,32 do
    local i = tostring(i)
    bin_inverted_masks[i] = xor(bin_masks[i], bin_masks["32"])
end

function split_octets(input)
    local pos = 0
    local prev = 0
    local octs = {}

    for i=1, 4 do
        pos = str_find(input, ".", prev, true)
        if pos then
            if i == 4 then
                -- Should not have a match after 4 octets
                return nil, "Invalid IP"
            end
            octs[i] = str_sub(input, prev, pos-1)
        elseif i == 4 then
            -- Last octet, get everything to the end
            octs[i] = str_sub(input, prev, -1)
            break
        else
            return nil, "Invalid IP"
        end
        prev = pos +1
    end

    return octs
end


function unsign(bin)
    if bin < 0 then
        return 4294967296 + bin
    end
    return bin
end


function ip2bin(ip)
    if lrucache then
        local get = lrucache:get(ip)
        if get then
            return get[1], get[2]
        end
    end

    if type(ip) ~= "string" then
        return nil, "IP must be a string"
    end

    local octets = split_octets(ip)
    if not octets or #octets ~= 4 then
        return nil, "Invalid IP"
    end

    -- Return the binary representation of an IP and a table of binary octets
    local bin_octets = {}
    local bin_ip = 0

    for i,octet in ipairs(octets) do
        local bin_octet = tonumber(octet)
        if not bin_octet or bin_octet < 0 or bin_octet > 255 then
            return nil, "Invalid octet: "..tostring(octet)
        end
        bin_octets[i] = bin_octet
        bin_ip = bor(lshift(bin_octet, 8*(4-i) ), bin_ip)
    end
    if not bin_ip then
        return nil, 'error'
    end
    bin_ip = unsign(bin_ip)
    if lrucache then
        lrucache:set(ip, {bin_ip, bin_octets})
    end
    return bin_ip, bin_octets
end

function split_cidr(input)
    local pos = str_find(input, "/", 0, true)
    if not pos then
        return {input}
    end
    return {str_sub(input, 1, pos-1), str_sub(input, pos+1, -1)}
end


function parse_cidr(cidr)
    local mask_split = split_cidr(cidr, '/')
    local net        = mask_split[1]
    local mask       = mask_split[2] or "32"
    local mask_num   = tonumber(mask)

    if not mask_num or (mask_num > 32 or mask_num < 0) then
        return nil, "Invalid prefix: /"..tostring(mask)
    end

    local bin_net, err = ip2bin(net) -- Convert IP to binary
    if not bin_net then
        return nil, err
    end
    local bin_mask     = bin_masks[mask] -- Get masks
    local bin_inv_mask = bin_inverted_masks[mask]

    local lower = band(bin_net, bin_mask) -- Network address
    local upper = bor(lower, bin_inv_mask) -- Broadcast address
    if not upper or not lower then
        return nil, 'error'
    end
    return unsign(lower), unsign(upper)
end

function parse_cidrs(cidrs)
    local out = {}
    local i = 1
    for _,cidr in ipairs(cidrs) do
        local lower, upper = parse_cidr(cidr)
        if not lower then
            log_err("Error parsing '", cidr, "': ", upper)
        else
            out[i] = {lower, upper}
            i = i+1
        end
    end
    return out
end

function ip_in_cidrs(ip, cidrs)
    local bin_ip, bin_octets = ip2bin(ip)
    if not bin_ip then
        return nil, bin_octets
    end

    for _,cidr in ipairs(cidrs) do
        if bin_ip >= cidr[1] and bin_ip <= cidr[2] then
            return true
        end
    end
    return false
end

function binip_in_cidrs(bin_ip_ngx, cidrs)
    if 4 ~= #bin_ip_ngx then
        return false, "invalid IP address"
    end

    local bin_ip = 0
    for i=1,4 do
        bin_ip = bor(lshift(bin_ip, 8), byte(bin_ip_ngx, i))
    end
    if not bin_ip then
        return false
    end
    bin_ip = unsign(bin_ip)

    for _,cidr in ipairs(cidrs) do
        if bin_ip >= cidr[1] and bin_ip <= cidr[2] then
            return true
        end
    end
    return false
end

