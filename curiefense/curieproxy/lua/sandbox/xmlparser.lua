-- from https://github.com/jonathanpoelen/xmlparser
module(..., package.seeall)

local logger        = require "logger"

local io = io
local string = string
local pairs = pairs

-- http://lua-users.org/wiki/StringTrim
local trim = function(s)
  local from = s:match"^%s*()"
  return from > #s and "" or s:match(".*%S", from)
end

-- our trim from luautils ...
-- function trim(s)
--     return (string.gsub(s, "^%s*(.-)%s*$", "%1"))
-- end

local gtchar = string.byte('>', 1)
local slashchar = string.byte('/', 1)
local D = string.byte('D', 1)
local E = string.byte('E', 1)

function parse(s, evalEntities)
  -- remove comments
  s = s:gsub('<!%-%-(.-)%-%->', '')

  local entities, tentities = {}
  
  if evalEntities then
    local pos = s:find('<[_%w]')
    if pos then
      s:sub(1, pos):gsub('<!ENTITY%s+([_%w]+)%s+(.)(.-)%2', function(name, q, entity)
        entities[#entities+1] = {name=name, value=entity}
      end)
      tentities = createEntityTable(entities)
      s = replaceEntities(s:sub(pos), tentities)
    end
  end

  local t, l = {}, {}

  local addtext = function(txt)
    txt = txt:match'^%s*(.*%S)' or ''
    if #txt ~= 0 then
      t[#t+1] = {text=txt}
    end    
  end
  
  s:gsub('<([?!/]?)([-:_%w]+)%s*(/?>?)([^<]*)', function(type, name, closed, txt)
    -- open
    if #type == 0 then
      local a = {}
      if #closed == 0 then
        local len = 0
        for all,aname,_,value,starttxt in string.gmatch(txt, "(.-([-_%w]+)%s*=%s*(.)(.-)%3%s*(/?>?))") do
          len = len + #all
          a[aname] = value
          if #starttxt ~= 0 then
            txt = txt:sub(len+1)
            closed = starttxt
            break
          end
        end
      end
      t[#t+1] = {tag=name, attrs=a, children={}}

      if closed:byte(1) ~= slashchar then
        l[#l+1] = t
        t = t[#t].children
      end

      addtext(txt)
    -- close
    elseif '/' == type then
      t = l[#l]
      l[#l] = nil

      addtext(txt)
    -- ENTITY
    elseif '!' == type then
      if E == name:byte(1) then
        txt:gsub('([_%w]+)%s+(.)(.-)%2', function(name, q, entity)
          entities[#entities+1] = {name=name, value=entity}
        end, 1)
      end
    end
  end)

  return {children=t, entities=entities, tentities=tentities}
end

function defaultEntityTable()
  return { quot='"', apos='\'', lt='<', gt='>', amp='&', tab='\t', nbsp=' ', }
end

function replaceEntities(s, entities)
  return s:gsub('&([^;]+);', entities)
end

function createEntityTable(docEntities, resultEntities)
  entities = resultEntities or defaultEntityTable()
  for _,e in pairs(docEntities) do
    e.value = replaceEntities(e.value, entities)
    entities[e.name] = e.value
  end
  return entities
end



function flatten_xml (XML)
  if not XML then return end
  local p_xml = xml_parse(XML)
  if p_xml then
    local ret_tbl = {}
    _do_flatten_xml(p_xml, ret_tbl)

    return ret_tbl
  end

end

function _do_flatten_xml(src_tbl,dst_tbl, prefix)
    if not src_tbl then
        return
    end

    if not prefix then
        prefix = ""
    end

    local tag = src_tbl.tag
    local attrs = src_tbl.attrs

    if tag then
        prefix = prefix .. src_tbl.tag
    end

    prefix = prefix:replace(":", "_")

    if attrs then
        for attr_name, attr_value in pairs(attrs) do
            dst_tbl[prefix .. attr_name ] = attr_value
        end
    end

    if src_tbl.children then
        for idx, child in ipairs (src_tbl.children) do
          local idx_prefix = prefix
          if idx_prefix ~= "" then
            idx_prefix = prefix .. tostring(idx)
          end
          _do_flatten_xml(child, dst_tbl, idx_prefix)
        end
    end

    for idx, entry_value in pairs(src_tbl) do
        -- if idx ~= "tag" and idx ~= "children" and type(entry_value) ~= "table" then
        if idx == "text" then
            -- print (prefix .. tostring(idx) .. ":" .. tostring(entry_value))
            dst_tbl[prefix] = entry_value
        end
    end
end