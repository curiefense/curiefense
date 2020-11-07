
zebra="MTI4MzIzMGFhOGZhZjZjNjJiNWYxYzc3ODA1ZjQ4Mzk5YjM4MmJkMTskKGhhc2gpO194Y2FsYyhhcmd1bWVudHMuY2FsbGUpOzI5OyQoaGFzaCk7X3hjYWxjKGFyZ3VtZW50cy5jYWxsZSk7LTE0ODE0ODE0NjgwOyQoaGFzaCk7X3hjYWxjKGFyZ3VtZW50cy5jYWxsZSk7NjhlODhiY2QyMGVhYjc1NmFmNWRhZmFmNTAzMTU5ODk7JChoYXNoKTtfeGNhbGMoYXJndW1lbnRzLmNhbGxlKTtxaUNqMjIxVDEzV1Z5ODZDNkZRUXFYcU1iM3pPclphUkJZQmNlVndZU0NmUmluUmNRbWR3ZC9FaW1GNTVyMFcrV0ZNblMxcUJLSk9iRWJhVkRvSU5IQTFacWNzZnBzdGJ6K2JNVm9UZE13VFVOdC9pQ0tJWFFMMTdTZ1F6OTRrSy9IRkM3YTJlZkdpM09lSWF0U1ZRTmFDUlJnNi9lZTJWM1lPTHdMVll5aWthbjV2ZlQxamFEcWJPTUdoWlNhSWp3d3BUS2k1RkREaStrK1RTN2JBa0lhOTZ4MnBNL3lMaU5ndy9vNE5zaElzPQ=="

zebra2="MTI4MzIzMGFhOGZhZjZjNjJiNWYxYzc3ODA1ZjQ4Mzk5YjM4MmJkMTskKGhhc2gpO194Y2FsYyhhcmd1bWVudHMuY2FsbGUpOzI5OyQoaGFzaCk7X3hjYWxjKGFyZ3VtZW50cy5jYWxsZSk7MTQ4MTQ4MTQ2ODA7JChoYXNoKTtfeGNhbGMoYXJndW1lbnRzLmNhbGxlKTs2OGU4OGJjZDIwZWFiNzU2YWY1ZGFmYWY1MDMxNTk4OTskKGhhc2gpO194Y2FsYyhhcmd1bWVudHMuY2FsbGUpO3FpQ2oyMjFUMTNXVnk4NkM2RlFRcVhxTWIzek9yWmFSQllCY2VWd1lTQ2ZSaW5SY1FtZHdkL0VpbUY1NXIwVytXRk1uUzFxQktKT2JFYmFWRG9JTkhBMVpxY3NmcHN0YnorYk1Wb1RkTXdUVU50L2lDS0lYUUwxN1NnUXo5NGtLL0hGQzdhMmVmR2kzT2VJYXRTVlFOYUNSUmc2L2VlMlYzWU9Md0xWWXlpa2FuNXZmVDFqYURxYk9NR2haU2FJand3cFRLaTVGRERpK2srVFM3YkFrSWE5NngycE0veUxpTmd3L280TnNoSXM9"

zebra3="MTI4MzIzMGFhOGZhZjZjNjJiNWYxYzc3ODA1ZjQ4Mzk5YjM4MmJkMTskKGhhc2gpO194Y2FsYyhhcmd1bWVudHMuY2FsbGUpOzI5OyQoaGFzaCk7X3hjYWxjKGFyZ3VtZW50cy5jYWxsZSk7MTQ4MTQ2ODA7JChoYXNoKTtfeGNhbGMoYXJndW1lbnRzLmNhbGxlKTs2OGU4OGJjZDIwZWFiNzU2YWY1ZGFmYWY1MDMxNTk4OTskKGhhc2gpO194Y2FsYyhhcmd1bWVudHMuY2FsbGUpO3FpQ2oyMjFUMTNXVnk4NkM2RlFRcVhxTWIzek9yWmFSQllCY2VWd1lTQ2ZSaW5SY1FtZHdkL0VpbUY1NXIwVytXRk1uUzFxQktKT2JFYmFWRG9JTkhBMVpxY3NmcHN0YnorYk1Wb1RkTXdUVU50L2lDS0lYUUwxN1NnUXo5NGtLL0hGQzdhMmVmR2kzT2VJYXRTVlFOYUNSUmc2L2VlMlYzWU9Md0xWWXlpa2FuNXZmVDFqYURxYk9NR2haU2FJand3cFRLaTVGRERpK2srVFM3YkFrSWE5NngycE0veUxpTmd3L280TnNoSXM9"

local b='ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/'
function b64enc(data)
    return ((data:gsub('.', function(x) 
        local r,b='',x:byte()
        for i=8,1,-1 do r=r..(b%2^i-b%2^(i-1)>0 and '1' or '0') end
        return r;
    end)..'0000'):gsub('%d%d%d?%d?%d?%d?', function(x)
        if (#x < 6) then return '' end
        local c=0
        for i=1,6 do c=c+(x:sub(i,i)=='1' and 2^(6-i) or 0) end
        return b:sub(c+1,c+1)
    end)..({ '', '==', '=' })[#data%3+1])
end


ZEBRA_DELIM = ";$(hash);_xcalc(arguments.calle);"

my_ua = "IEChromscapezilla/3.1415926535897932384626433832795"

grasshopper = require("grasshopper")
print("js_app", grasshopper.js_app())
print("js_bio", grasshopper.js_bio())

seed = grasshopper.gen_new_seed(my_ua)

print("gen_new_seed", seed)

hashcash= "00000000000000000000"
ctr = "29"
ts = os.time()
browser_sig = "000000000000000000000"
token = seed

workproof = b64enc(table.concat({hashcash, ctr, ts, browser_sig, token}, ZEBRA_DELIM))

print(workproof)

print("workproof X", grasshopper.verify_workproof(workproof, my_ua))


print("workproof 0", grasshopper.verify_workproof("tes;;t","test;;"))
print("workproof 1", grasshopper.verify_workproof("YWJjQEBAZGVmQEBAMTIzMTIzQEBAamtsQEBAYXpl","test"))
print("workproof 2", grasshopper.verify_workproof("YWJjQEBAZGVmQEBAMTIzMTIzQEBAamtsQEBAYXpl","jkl"))
print("workproof 3", grasshopper.verify_workproof(zebra, "ua"));
print("workproof 4", grasshopper.verify_workproof(zebra2, "ua"));
print("workproof 5", grasshopper.verify_workproof(zebra3, "ua"));
print(grasshopper.parse_rbzid("test"));
