function newnode(val) return { value = val, left = {}, right = {} } end

function insert(value, node)
  if not node.value then
    node.value = value
    node.left = {}
    node.right = {}
    return
  end
  if value < node.value then
    insert(value, node.left)
  else
    insert(value, node.right)
  end
end

function search(value, node)
  print (string.format("search: v %s NV %s NL %s NR %s", value, node.value, node.left, node.right))
  if node.value == nil then
    print "not found"
    return nil
  end
  if value ~= node.value then
    if value < node.value then
      print "calling with left"
      return search(value, node.left)
    else
      print "calling with right"
      return search(value, node.right)
    end
  else
    print ("found", value)
    return value
  end
end


btree = newnode(12)
insert(1234, btree)
insert(134, btree)
insert(234, btree)
insert(2344, btree)
insert(23446, btree)
insert(26, btree)
insert(64, btree)
insert(164, btree)
insert(264, btree)
insert(364, btree)
insert(464, btree)
insert(564, btree)
insert(664, btree)
insert(764, btree)
insert(864, btree)
insert(964, btree)
insert(13, btree)
insert(10, btree)
insert(3, btree)

vv = search(3, btree)
print ("vv", vv)