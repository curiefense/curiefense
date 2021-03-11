module(..., package.seeall)

local globals       = require "lua.globals"
local utils         = require "lua.utils"
-- local rangesbtree   = require "lua.rangesbtree"
local cjson         = require "cjson"
local json_safe     = require "cjson.safe"

local re_match        = utils.re_match
local tag_request     = utils.tag_request
local table_length    = utils.table_length
local json_encode     = cjson.encode

function match_singles(request_map, list_entry)
  for entry_key, list_entries in pairs(list_entry) do
    -- exact request map
    local entry_match = list_entries[request_map[entry_key]]
    if entry_match then
      -- request_map.handle:logDebug(string.format("match_singles:: Exact Match! %s %s %s", entry_match, entry_key, request_map[entry_key]))
      return entry_match
    end
    -- exact request map's attr
    entry_match = list_entries[request_map.attrs[entry_key]]
    if entry_match then
      -- request_map.handle:logDebug(string.format("match_singles:: ATTR Match! %s %s %s", entry_match, entry_key, request_map.attrs[entry_key]))
      return entry_match
    end

    -- pattern matching for country
    if entry_key == 'country' then
      local value = request_map.geo.country.iso
      if value then
        entry_match = list_entries[request_map.geo.country.iso]
        if entry_match then
          return entry_match
        end
      end
    end

    -- pattern matching for ASN
    if entry_key == 'asn' then
      local value = request_map.geo.asn
      if value then
        entry_match = list_entries[request_map.geo.asn]
        if entry_match then
          return entry_match
        end
      end
    end

    -- pattern matching for all but ip or ASN.
    if entry_key ~= 'ip' and entry_key ~= 'asn' then
      for pattern, annotation in pairs(list_entries) do
        local value = request_map.attrs[entry_key]
        if value then
          local val_patt_match = re_match(value, pattern)
          -- request_map.handle:logDebug(string.format("match_singles:: %s matched? %s >>  - regex %s %s", entry_key, val_patt_match, value, pattern))
          if val_patt_match then
            return annotation
          end
        end
      end
    end
  end
  -- no match
  return false
end

function match_pairs(request_map, list_entry)
  for pair_name, match_entries in pairs(list_entry) do
    for key, va in pairs(match_entries) do
      local value, annotation = unpack(va)
      local reqmap_value = request_map[pair_name][key]
      if value and reqmap_value then
        local val_matched = (reqmap_value == value or re_match(reqmap_value, value))
        -- request_map.handle:logDebug(string.format("matched? %s >> match_pairs %s %s", val_matched, reqmap_value, value))
        if val_matched then
          return annotation
        end
      end
    end
  end
  return false
end

function negate_match_pairs(request_map, list_entry)
  for pair_name, match_entries in pairs(list_entry) do
    for key, va in pairs(match_entries) do
      local value, annotation = unpack(va)
      local reqmap_value = request_map[pair_name][key]
      if value and reqmap_value then
        local neg_val_matched = (reqmap_value ~= value and not re_match(reqmap_value, value))
        -- request_map.handle:logDebug(string.format("negate matched? %s >> negate_match_pairs %s NOT %s", neg_val_matched, reqmap_value, value))
        if neg_val_matched then
          return annotation
        end
      end
    end
  end
  return false
end

function eval_section(request_map, section)
  -- -- request_map.handle:logDebug(string.format("SECTION %s" ,json_encode(section)))
  local section_relation_and = (section.relation == "AND")
  local section_entries = section.entries
  local m_singles, m_pairs, m_iprange, nm_singles, nm_pairs, nm_iprange  = false, false, false, false, false, false

  -- for k, v in pairs(section.entries) do
  -- --   request_map.handle:logDebug(string.format("K %s V %s", k, v))
  -- end

  if table_length(section_entries.singles) > 0 then
    -- -- request_map.handle:logDebug(string.format("match_or_list section.singles %s", json_encode(section_entries.singles)))
    local annotation, tags = match_singles(request_map, section_entries.singles)
    if not annotation then
      m_singles = false
      if section_relation_and then return false end
    else
      m_singles = true
    end
  end

  if table_length(section_entries.pairs) > 0 then
    -- -- request_map.handle:logDebug(string.format("match_or_list section_entries.pairs %s", json_encode(section_entries.pairs)))
    local annotation, tags = match_pairs(request_map, section_entries.pairs)
    if not annotation then
      m_pairs = false
      if section_relation_and then return false end
    else
      m_pairs = true
    end
  end

  if (section_entries.iprange:len() > 0 ) then
    -- -- request_map.handle:logDebug(string.format("match_or_list section_entries.iprange %s", json_encode(section_entries.iprange)))
    local within = section_entries.iprange:contains(request_map.attrs.ip)
    if not within then
      m_iprange = false
      if section_relation_and then return false end
    else
      m_iprange = true
    end
  end

  -- has entries
  if table_length(section_entries.negate_singles) > 0 then
    -- -- request_map.handle:logDebug(string.format("match_or_list section_entries.negate_singles %s", json_encode(section_entries.negate_singles)))
    local annotation, tags = match_singles(request_map, section_entries.negate_singles)
    if annotation then
      nm_singles = false
      if section_relation_and then return false end
    else
      nm_singles = true
    end
  end

  if table_length(section_entries.negate_pairs) > 0 then
    -- -- request_map.handle:logDebug(string.format("match_or_list section_entries.negate_pairs %s", json_encode(section_entries.negate_pairs)))
    local annotation, tags = negate_match_pairs(request_map, section_entries.negate_pairs)
    if not annotation then
      nm_pairs = false
      if section_relation_and then return false end
    else
      nm_pairs = true
    end
  end

  if section_entries.negate_iprange:len() > 0 then
    -- -- request_map.handle:logDebug(string.format("match_or_list section_entries.negate_iprange %s", json_encode(section_entries.negate_iprange)))
    local within = section_entries.negate_iprange:contains(request_map.attrs.ip)
    if within then
      nm_iprange = false
      if section_relation_and then return false end
    else
      nm_iprange = true
    end
  end

  return m_singles or m_pairs or m_iprange or nm_singles or nm_pairs or nm_iprange

end

function eval_list( request_map, list )

    -- request_map.handle:logDebug(string.format("TAG LIST profiling list %s (%s)", list.name, list.id))

    local list_matched = false
    local sections = list.rule.sections
    local rule_relation_and = (list.rule.relation == "AND")

    for _, section in ipairs(sections) do
      local sec_eval = eval_section(request_map, section)

      -- request_map.handle:logDebug(string.format("TAG LIST section evaluated %s", sec_eval))
      if not sec_eval then
        -- first no match, bounce.
        if rule_relation_and then
          -- request_map.handle:logDebug(string.format("TAG LIST rule_relation_and %s exiting", rule_relation_and))
          return
        end
        -- request_map.handle:logDebug("TAG LIST setting list_matched false given eval_section result and rule_relation_and is false")
        list_matched = false
      else
        -- OR mode, match and move on.
        if not rule_relation_and then
          -- request_map.handle:logDebug("TAG LIST setting tagging request given rule_relation_and is false and eval_section matched")
          return true
        end
        list_matched = true
      end
    end
    -- request_map.handle:logDebug(string.format("TAG LIST done with list -- rule_relation_and %s list_matched %s",
      -- rule_relation_and, list_matched))
    if rule_relation_and and list_matched then
      return true
    end

end

function tag_lists(request_map)
  -- request_map.handle:logDebug(string.format("TAG LIST WITH \nHEADERS:\n%s, \nARGS\n%s",
    -- json_encode(request_map.headers),json_encode(request_map.args)))

  for _, list in pairs(globals.ProfilingLists) do
    local list_eval = eval_list(request_map, list)
    if list_eval then
      tag_request(request_map, list.tags)
    end
  end
end
