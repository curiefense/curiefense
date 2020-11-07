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
UA = "IEChromscapezilla/3.1415926535897932384626433832795"

gh = require("grasshopper")
print("js_app", gh.js_app())
print("js_bio", gh.js_bio())

seed = gh.gen_new_seed(UA)

print("browser receives:", seed)

hashcash= "00000000000000000000"
ctr = "29"
ts = os.time()
browser_sig = "000000000000000000000"
token = seed
workproof = b64enc(table.concat({hashcash, ctr, ts, browser_sig, token},
                      ZEBRA_DELIM))

print("browser sends workproof:", workproof)

rbzid = gh.verify_workproof(workproof, UA)
if (rbzid == nil) then
   print("Bad workproof")
else
      print("got rbzid:", rbzid)
      print("is rbzid good?", gh.parse_rbzid(rbzid, UA));
end
