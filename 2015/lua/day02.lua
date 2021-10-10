local f = io.open("../inputs/day02.txt", "r")

function paper_needed(d)
  assert(#d == 3 and type(d[1] == 'number'))
  table.sort(d)
  return 2 * (d[1] * d[2] + d[1] * d[3] + d[2] * d[3]) + d[1] * d[2]
end

function ribbon_needed(d)
  assert(#d == 3 and type(d[1] == 'number'))
  table.sort(d)
  return 2 * (d[1] + d[2]) + d[1] * d[2] * d[3]
end


local n = 0
local ribbon = 0

for line in f:lines() do
  local dimensions = {}
  for d in line:gmatch('%d+') do
    dimensions[#dimensions + 1] = tonumber(d)
  end
  n = n + paper_needed(dimensions)
  ribbon = ribbon + ribbon_needed(dimensions)
end

print("Part 1:", n)
print("Part 2:", ribbon)
f:close()
