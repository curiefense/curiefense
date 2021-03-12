local iptools = require ("iptools")
-- local inspect = require('inspect')

local mmdb = iptools.new_geoipdb()

mmdb:load_asn_db("../curieproxy/config/maxmind/GeoLite2-ASN.mmdb")
mmdb:load_city_db("../curieproxy/config/maxmind/GeoLite2-City.mmdb")
mmdb:load_country_db("../curieproxy/config/maxmind/GeoLite2-Country.mmdb")

function ipinfo(ip, handle)
    local city = mmdb:lookup_city(ip)
    local country, iso = mmdb:lookup_country(ip)
    local asn, org = mmdb:lookup_asn(ip)
    return {
        city, country, iso, asn, org
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


local city, country, iso, asn, company = unpack(ipinfo("199.0.0.1"))
print("ASN")
print(asn, company)
assert(asn)
assert(company)

print("Country")
inspect(country, "country")
assert(country)

print("City")
inspect(city, "city")
assert(city)
