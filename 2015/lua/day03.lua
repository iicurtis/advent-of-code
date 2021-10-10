local f = io.open("../inputs/day03.txt", "r")

input = f:read("a")
local count = 1
local x = 1
local y = 1
local visited = {[1] = {[1] = true}}

function have_visited(x, y)
  if not visited[x] then 
    visited[x] = {[y] = true}
    return 1
  end
  if not visited[x][y] then
    visited[x][y] = true
    return 1
  end
  return 0
end

for idx = 1, #input do
  if input:byte(idx) == string.byte("^") then
    x = x + 1
    count = count + have_visited(x, y)
  elseif input:byte(idx) == string.byte("v") then
    x = x - 1
    count = count + have_visited(x, y)
  elseif input:byte(idx) == string.byte(">") then
    y = y + 1
    count = count + have_visited(x, y)
  elseif input:byte(idx) == string.byte("<") then
    y = y - 1
    count = count + have_visited(x, y)
  else
    print("Char not found: ", input:byte(idx))
  end
end
print("Part 1:", count)

 -- part 2
local soln2 = 1
local x2 = 1
local y2 = 1
local visited = {[1] = {[1] = true}}
for idx = 1, #input, 2 do
  if input:byte(idx) == string.byte("^") then
    x = x + 1
    soln2 = soln2 + have_visited(x, y)
  elseif input:byte(idx) == string.byte("v") then
    x = x - 1
    soln2 = soln2 + have_visited(x, y)
  elseif input:byte(idx) == string.byte(">") then
    y = y + 1
    soln2 = soln2 + have_visited(x, y)
  elseif input:byte(idx) == string.byte("<") then
    y = y - 1
    soln2 = soln2 + have_visited(x, y)
  else
    print("Char not found: ", input:byte(idx))
  end

  if input:byte(idx+1) == string.byte("^") then
    x2 = x2 + 1
    soln2 = soln2 + have_visited(x2, y2)
  elseif input:byte(idx+1) == string.byte("v") then
    x2 = x2 - 1
    soln2 = soln2 + have_visited(x2, y2)
  elseif input:byte(idx+1) == string.byte(">") then
    y2 = y2 + 1
    soln2 = soln2 + have_visited(x2, y2)
  elseif input:byte(idx+1) == string.byte("<") then
    y2 = y2 - 1
    soln2 = soln2 + have_visited(x2, y2)
  else
    print("Char not found: ", input:byte(idx+1))
  end
end

print("Part 2:", soln2)
f:close()
