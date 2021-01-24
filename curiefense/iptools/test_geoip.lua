local iptools = require ("iptools")
-- local inspect = require('inspect')

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



a, b = mmdb:lookup_asn("89.160.20.128")
print("ASN ===>", a, b)

a, b = mmdb:lookup_country("89.160.20.128")
print("country ===>", a, b)

city = mmdb:lookup_city("89.160.20.128")
-- print("city ===>", inspect(city))
for k,v in pairs(city) do
    for k2, v2 in pairs(v) do
        print("city ",k,k2," ===> ", v2)
    end
end
