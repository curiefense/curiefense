local nativeutils = {}
-- helpers for native rust libraries

function nativeutils.trim(s)
    return (string.gsub(s, "^%s*(.-)%s*$", "%1"))
end

function nativeutils.startswith(str, arg)
    if str and arg and type(str) == "string" and type(arg) == "string" then
        return string.find(str, arg, 1, true) == 1
    end
end

function nativeutils.endswith(str, arg)
    if str and arg then
        return string.find(str, arg, #str - #arg + 1, true) == #str - #arg + 1
    end
end

-- source http://lua-users.org/wiki/SplitJoin
function nativeutils.split(input, sSeparator, nMax, bRegexp)
    local aRecord = {}

    if sSeparator ~= '' then
      if (nMax == nil or nMax >= 1)then
        if input ~= nil then
          if input:len() > 0 then
            local bPlain = not bRegexp
            nMax = nMax or -1

            local nField=1
            local nStart=1
            local nFirst,nLast = input:find(sSeparator, nStart, bPlain)
            while nFirst and nMax ~= 0 do
                aRecord[nField] = input:sub(nStart, nFirst-1)
                nField = nField+1
                nStart = nLast+1
                nFirst,nLast = input:find(sSeparator, nStart, bPlain)
                nMax = nMax-1
            end
            aRecord[nField] = input:sub(nStart)
          end
        end
      end
    end

    return aRecord
end

function nativeutils.map_fn (T, fn)
    T = T or {}
    local ret = {}
    for _, v in ipairs(T) do
        local new_value = fn(v)
        table.insert(ret, new_value)
    end
    return ret
end

return nativeutils
