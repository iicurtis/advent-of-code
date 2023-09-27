module Day03

function day03(input::String=joinpath(@__DIR__, "..", "..", "inputs", "day03.txt"))
  input = open(input, "r") do file
    read(file, String)
  end
  input_lines = split(rstrip(input), "\n")
  part1 = [(pack[1:length(pack)÷2]∩pack[length(pack)÷2+1:end])[1] for pack ∈ input_lines]
  part2 = [intersect(input_lines[i:i+2]...)[1] for i ∈ 1:3:length(input_lines)]
  # for each line, split line in 2 halves
  #seen1 = seen1 & (1 << value(first[i]))
  #seen2 = seen2 & (1 << value(second[i]))
  return [value.(part1) |> sum, value.(part2) |> sum]
end

function value(c::Char)
  return islowercase(c) ? c - 'a' + 1 : c - 'A' + 27
end

end #module

Day03.day03()
