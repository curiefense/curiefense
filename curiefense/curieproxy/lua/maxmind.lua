module(..., package.seeall)

local mmdb = require "mmdb"
local rex  = require("rex_pcre2");

local isipv4    = rex.new("(\\d{1,3}\\.){3}\\d{1,3}")
local asndb     = assert(mmdb.read("/config/current/config/maxmind/GeoLite2-ASN.mmdb"))
local countrydb = assert(mmdb.read("/config/current/config/maxmind/GeoLite2-Country.mmdb"))
-- local countrydb = assert(mmdb.read("/config/maxmind/GeoLite2-City.mmdb"))

function ipinfo(ip, handle)
    -- returns { county, asn, company}
    local country_info, asn_info = nil, nil

    if isipv4:match(ip) then
        country_info = countrydb:search_ipv4(ip)
        asn_info = asndb:search_ipv4(ip)
    else
        country_info = countrydb:search_ipv6(ip)
        asn_info = asndb:search_ipv6(ip)
    end

    return {
        country_info and country_info["country"] and country_info["country"]["names"]["en"],
        asn_info and asn_info.autonomous_system_number,
        asn_info and asn_info.autonomous_system_organization
    }
end

