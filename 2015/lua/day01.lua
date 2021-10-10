local f = io.open("../inputs/day01.txt", "r")
io.input(f)
--local input = f:read("a")

input = io.read("a")
local count = 0
local soln2 = nil

for idx = 1, #input do
  if input:byte(idx) == string.byte("(") then
    count = count + 1
  elseif input:byte(idx) == string.byte(")") then
    count = count - 1
  else
    print("Char not found: ", input:byte(idx))
  end

  -- part 2
  if count == -1 and soln2 == nil then
    soln2 = idx
  end
end

print("Part 1:", count)
print("Part 2:", soln2)
f:close()
