local iptools = require ("iptools")
local inspect = require('inspect')

local mmdb = iptools.new_geoipdb()

mmdb:load_asn_db("../curieproxy/config/maxmind/GeoLite2-ASN.mmdb")
mmdb:load_city_db("../curieproxy/config/maxmind/GeoLite2-City.mmdb")
mmdb:load_country_db("../curieproxy/config/maxmind/GeoLite2-Country.mmdb")

function ipinfo(ip, handle)
    local country, iso = mmdb:lookup_country(ip)
    local asn, org = mmdb:lookup_asn(ip)
    return {
        country, asn, org, iso
    }
end



-- print("loading asn:",g:load_asn_db("/home/pbi/reb/curiefense/curiefense/curieproxy/config/maxmind/GeoLite2-ASN.mmdb"))
-- print("loading country:", g:load_country_db("/home/pbi/reb/curiefense/curiefense/curieproxy/config/maxmind/GeoLite2-Country.mmdb"))
-- print("laoded")
a, b = mmdb:lookup_asn("89.160.20.128")
print("ASN ===>", a, b)

a, b = mmdb:lookup_country("89.160.20.128")
print("country ===>", a, b)

city = mmdb:lookup_city("89.160.20.128")
print("city ===>", inspect(city))
