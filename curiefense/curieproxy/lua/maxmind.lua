module(..., package.seeall)

-- local mmdb = require "mmdb"
-- local rex  = require("rex_pcre2");

-- local isipv4    = rex.new("(\\d{1,3}\\.){3}\\d{1,3}")
-- local asndb     = assert(mmdb.read("/config/current/config/maxmind/GeoLite2-ASN.mmdb"))
-- local countrydb = assert(mmdb.read("/config/current/config/maxmind/GeoLite2-Country.mmdb"))
-- -- local countrydb = assert(mmdb.read("/config/maxmind/GeoLite2-City.mmdb"))

-- function ipinfo(ip, handle)
--     -- returns { county, asn, company}
--     local country_info, asn_info = nil, nil

--     if isipv4:match(ip) then
--         country_info = countrydb:search_ipv4(ip)
--         asn_info = asndb:search_ipv4(ip)
--     else
--         country_info = countrydb:search_ipv6(ip)
--         asn_info = asndb:search_ipv6(ip)
--     end

--     return {
--         country_info and country_info["country"] and country_info["country"]["names"]["en"],
--         asn_info and asn_info.autonomous_system_number,
--         asn_info and asn_info.autonomous_system_organization
--     }
-- end


local iptools = require ("iptools")

local mmdb = iptools.new_geoipdb()

mmdb:load_asn_db("/config/current/config/maxmind/GeoLite2-ASN.mmdb")
mmdb:load_city_db("/config/current/config/maxmind/GeoLite2-City.mmdb")
mmdb:load_country_db("/config/current/config/maxmind/GeoLite2-Country.mmdb")

function ipinfo(ip, handle)
    local city = mmdb:lookup_city(ip)
    local country, iso = mmdb:lookup_country(ip)
    local asn, org = mmdb:lookup_asn(ip)
    return {
        city, country, iso, asn, org
    }
end



-- print("loading asn:",g:load_asn_db("/home/pbi/reb/curiefense/curiefense/curieproxy/config/maxmind/GeoLite2-ASN.mmdb"))
-- print("loading country:", g:load_country_db("/home/pbi/reb/curiefense/curiefense/curieproxy/config/maxmind/GeoLite2-Country.mmdb"))
-- print("laoded")
-- a,b = g:lookup_asn("89.160.20.128")
-- print("ASN ===>",a,b)
-- a,b = g:lookup_country("89.160.20.128")
-- print("country ===>",a,b)
