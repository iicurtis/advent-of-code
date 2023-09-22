-- local f = assert(io.open("../inputs/day01.txt", "r"))
-- local inp = f:read("*all")
-- f:close()

local elfs = {}
local sum = 0
for line in io.lines("../inputs/day01.txt") do
	if line == "" then
		table.insert(elfs, sum)
		sum = 0
	else
		sum = sum + tonumber(line)
	end
end

table.sort(elfs)

print("Part 1: ", elfs[#elfs])
print("Part 2: ", elfs[#elfs] + elfs[#elfs - 1] + elfs[#elfs - 2])
