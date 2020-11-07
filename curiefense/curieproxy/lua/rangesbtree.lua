module(..., package.seeall)

-- binary tree for ip ranges
-- [[1, 1234], [2345, 214235], [3245466,24556677]]
-- nodevalue evaluated by lower-end of the range
-- TODO:
-- if during insertion, a broader range is evaluated, that is, a range that contains an existing one
-- it will replace the existing

function newnode(range, annotation) return { range = range, left = {}, right = {}, annotation=annotation } end

function btree_insert(range, annotation, node)
  if not node.range then
    node.range = range
    node.annotation = annotation
    node.left = {}
    node.right = {}
    return
  end
  if range[1] < node.range[1] then
    btree_insert(range, annotation, node.left)
  else
    btree_insert(range, annotation, node.right)
  end
end

function btree_search(value, node, handle)
  if node.range == nil then
    return nil
  end

  local lend = node.range[1]
  local uend = node.range[2]

  if value >= lend and value <= uend then
    return value, node.annotation
  else
    if value < lend then
      return btree_search(value, node.left, handle)
    else
      return btree_search(value, node.right, handle)
    end
  end
end

function build_ranges_lists(ranges)
  -- first
  local range, annotation = unpack(ranges[1])
  local btree = newnode(range, annotation)

  -- rest
  for idx=2, #ranges, 1 do
    range, annotation = unpack(ranges[idx])
    btree_insert(range, annotation, btree)
  end

  return btree
end
--[[
  btree = newnode({2851059200, 2851059455})
  insert({1666600960, 1666603007}, btree)
  vv = search(1666603009, btree)
]]--

--[[
  elem =  { {start_addr, end_addr}, get_annotation(data) }
  table.insert(masterdict[mastercategory], elem)
]]