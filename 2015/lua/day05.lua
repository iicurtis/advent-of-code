local f = io.open("../inputs/day05.txt", "r")
local nice_count = 0
local nice_count_part2 = 0
for line in f:lines() do
  line = line:lower()
  if line:find("[aeiou].*[aeiou].*[aeiou]") and line:find("(.)%1")
    and not ( line:find("ab") or line:find("cd") or line:find("pq") or line:find("xy")) then
      nice_count = nice_count + 1
  end

  if line:find("(..).*%1") and line:find("(.).%1") then
    nice_count_part2 = nice_count_part2 + 1
  end
end

print('Part 1: ', nice_count)
print('Part 2: ', nice_count_part2)
f:close()
