local md5 = require('md5') -- keplar project md5 compiled for lua 5.1 and luajit
-- file io
local f = io.open("../inputs/day04.txt", "r")
local input = f:read("*l")
f:close()

print("Input:", input)

local m5hash = nil
local m6hash = nil

local i = 1
while true do
  local sum = md5.sumhexa(input .. i)
  if sum:match("^00000.*") and not m5hash then
    m5hash = i
  end
  if sum:match("^000000.*") and not m6hash then
    m6hash = i
  end
  if m5hash and m6hash then
    break
  end
  i = i + 1
end

print("Part 1:", m5hash)
print("Part 2:", m6hash)
