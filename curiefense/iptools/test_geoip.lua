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

function inspect(t, level)
    level = level or ""

    for k, v in pairs(t) do
        k = level .. "." .. k
        if (type(v) == "table") then
            inspect(v, k)
        else
            print(k, " ===> ", v)
        end
    end
end


a, b = mmdb:lookup_asn("89.160.20.128")
print("ASN ===>", a, b)

country = mmdb:lookup_country("89.160.20.128")

print("Country")
inspect(country, "country")

city = mmdb:lookup_city("89.160.20.128")
print("City")
inspect(city, "city")
