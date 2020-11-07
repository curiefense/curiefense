module(..., package.seeall)

local grasshopper = require "grasshopper"
local utils   = require "lua.utils"

local deny_request = utils.deny_request
local tag_request  = utils.tag_request

local parse_rbzid       = grasshopper.parse_rbzid
local gen_new_seed      = grasshopper.gen_new_seed
local verify_workproof  = grasshopper.verify_workproof

local sfmt = string.format

local chall_lib = grasshopper.js_app()
local biolib = grasshopper.js_bio()

function verified (handle, request_map)
    local rbzid = request_map.cookies["rbzid"]
    local ua = request_map.headers["user-agent"]

    if not rbzid or not ua then
        return false
    end

    rbzid = rbzid:replace("-", "=")
    return parse_rbzid(rbzid, ua)
end

function phase01(handle, request_map, reload_page)
    handle:logDebug("ACL DENY BOT MATCHED! << phase01 in action >>")
    local contract = 3
    local ua = request_map.headers["user-agent"] or "never-provided"

    local seed = gen_new_seed(ua)
    local code_block = chall_lib ..
        [[;;window.rbzns={bereshit: "]] .. reload_page .. [[", seed: "]]
        .. seed .. [[", storage:"]] .. contract .. [["};winsocks();]]

    local content = [[<html><head><meta charset="utf-8">]] ..
        [[<script>]] .. code_block .. [[</script></head><body></body></html>]]


    handle:logDebug(sfmt("005 ACL DENY BOT MATCHED! << pushing JS>>"))
    local headers = {
        -- [":status"] = "247",
        ["Content-Type"] = "text/html; charset=utf-8",
        ["Expires"] = "Thu, 01 Aug 1978 00:01:48 GMT",
        ["Cache-Control"] = "no-cache, private, no-transform, no-store",
        ["Pragma"] = "no-cache",
        ["P3P"] = 'CP="IDC DSP COR ADM DEVi TAIi PSA PSD IVAi IVDi CONi HIS OUR IND CNT"',
    }
    tag_request(request_map, "challenge-phase01")
    deny_request(request_map, {["reason"] = "challenge-phase01"}, true, "247", headers, content)
end

function extract_zebra(headers)
    for n, v in pairs(headers) do
        if n:startswith("x-zebra-") then
            return v:gsub("-", "=")
        end
    end
end

function phase02(handle, request_map)
    handle:logDebug("006 ACL DENY BOT MATCHED! << phase02 in action >>")
    local ua = request_map.headers["user-agent"]
    handle:logDebug(sfmt("007 phase02 UA %s", ua))
    if ua then
        local workproof = extract_zebra(request_map.headers)
        handle:logDebug(sfmt("008 phase02 workproof %s", workproof))
        if workproof then
            local rbzid = verify_workproof(workproof, ua)
            handle:logDebug(sfmt("009 phase02 rbzid %s", rbzid))
            if rbzid then
                local cookie = "rbzid=" .. rbzid:replace("=", "-") .. "; Path=/; HttpOnly"
                local headers = { ["Set-Cookie"] = cookie }
                tag_request(request_map, "challenge-phase02")
                deny_request(request_map, {["reason"] = "challenge-phase02"}, true, "248", headers, "{}")
            else
                handle:logDebug("010 phase02 NO rbzid!")
            end
        else
            handle:logDebug("011 phase02 NO workproof!")
        end
    end
end

