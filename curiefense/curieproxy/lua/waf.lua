module(..., package.seeall)

local globals   = require "lua.globals"
local utils     = require "lua.utils"

local WAFPass   = globals.WAFPass
local WAFBlock  = globals.WAFBlock

local re_match  = utils.re_match

function store_section(master_dict, key, subkey,  value)
    if master_dict[key] then
        master_dict[key][subkey] = value
    else
        master_dict[key] = { [subkey] = value}
    end
end

function build_section(section_name, profile)
    local name_rules, regex_rules, max_len, max_count

    if section_name == "headers" then
        name_rules = profile.headers.names
        regex_rules = profile.headers.regex
        max_count = profile.max_headers_count
        max_len = profile.max_header_length

    elseif section_name == "cookies" then
        name_rules = profile.cookies.names
        regex_rules = profile.cookies.regex
        max_count = profile.max_cookies_count
        max_len = profile.max_cookie_length

    elseif section_name == "args" then
        name_rules = profile.args.names
        regex_rules = profile.args.regex
        max_count = profile.max_args_count
        max_len = profile.max_arg_length
    end

    return {name_rules, regex_rules, max_len, max_count}

end

function name_check(section, name, name_rule, value, omit_entries, sig_excludes)
    local matched = re_match(value, name_rule.reg)

    if matched then
        store_section(omit_entries, section, name, true)
    else
        if name_rule.restrict then
            return  WAFBlock, string.format("(%s)/%s mismatch with %s", section, name_rule.reg, value)
        elseif #name_rule.exclusions  > 0 then
            store_section(sig_excludes, section, name, name_rule.exclusions)
        end
    end
end

function regex_check(section, name, regex_rules, omit_entries, sig_excludes)

    for name_patt, patt_rule in pairs(regex_rules) do
        if re_match(name, name_patt) then
            local matched = re_match(value, patt_rule.reg)
            if matched then
                store_section(omit_entries, section, name, true)
            else
                if patt_rule.restrict then
                    return WAFBlock, string.format("(%s)/name-patt:%s value-patt:%s mismatch with name: %s value:%s", section, name_patt, patt_rule.reg, name, value)
                elseif #patt_rule.exclusions > 0 then
                    store_section(sig_excludes, section, name, patt_rule.exclusions)
                end
            end
        end
    end

    return nil, nil

end

function waf_regulate(section, profile, request, omit_entries, exclude_sigs)
    request.handle:logDebug("WAF regulation - positive security for section: " .. section)
    local name_rules, regex_rules, max_len, max_count = unpack(build_section(section, profile))

    request.handle:logDebug(string.format("name_rules %s, regex_rules %s, max_len %s, max_count %s", name_rules, regex_rules, max_len, max_count))

    local entries = request[section]
    local check_regex = (#regex_rules > 0)
    local ignore_alphanum = profile.ignore_alphanum

    if #entries > max_count then
        return WAFBlock, string.format("Maximum entries for section %s exceeded. Limit: %s, Got: %s",
            section, max_count, #entries)
    end

    for name, value in pairs(entries) do
        if value then
            -- headers/ cookies/args length
            local value_len = value:len()
            if value_len > max_len then
                return WAFBlock, string.format("Length of %s/%s exceeded. Limit: %s, Got: %s",
                    section, name, max_len, value_len)
            end

            if ignore_alphanum and re_match(value, "^\\w$") then
                store_section(omit_entries, section, name, true)
            else
                name_rule = name_rules[name]
                if name_rule then
                    local respone, msg = name_check(section, name, name_rule, value, omit_entries, sig_excludes)
                    if WAFBlock == response then
                        return response, msg
                    end
                end
                if check_regex then
                    local response, msg = regex_check(section, name, regex_rules, omit_entries, sig_excludes)
                    if WAFBlock == response then
                        return response, msg
                    end
                end
            end
        end
    end

    return WAFPass, ""
end

function check(waf_profile, request)
    request.handle:logDebug("WAF inspection starts - with profile %s", waf_profile.name)
    local omit_entries = {}
    local exclude_sigs = {}
    local sections = {"headers", "cookies", "args"}

    for _, section in ipairs(sections) do
        request.handle:logDebug("WAF inspecting section: " .. section)
        -- positive security
        local response, msg = waf_regulate(section, waf_profile, request, omit_entries, exclude_sigs)
        if response == WAFBlock then
            return response, msg
        end
        -- negative security
        for name, value in pairs(request[section]) do
            if omit_entries[section] == nil or (not omit_entries[section][name]) then
                for _, sig in ipairs(globals.WAFSignatures) do
                    if exclude_sigs[sections] == nil or (not exclude_sigs[sections][name][sig.id]) then

                        if re_match(value, sig.operand) then

                            return WAFBlock, {
                                ["initiator"] = "waf",
                                ["sig_id"] = sig.id,
                                ["sig_category"] = sig.category,
                                ["sig_subcategory"] = sig.subcategory,
                                ["sig_severity"] = sig.severity,
                                ["sig_certainity"] = sig.certainity,
                                ["sig_operand"] = sig.operand,
                                ["sig_msg"] = sig.msg,
                                ["section"] = section,
                                ["name"] = name,
                                ["value"] = value
                            }

                        end
                    end
                end
            end
        end
    end

    return WAFPass, "waf-passed"
end
