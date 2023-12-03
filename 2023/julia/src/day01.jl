module Day01

function day01(input::String=joinpath(@__DIR__, "..", "..", "inputs", "day01.txt"))
  input = open(input, "r") do file
    read(file, String)
  end
  lines = split(rstrip(input), "\n")
  digits_to_num(s::AbstractString) = parse(Int, s[1] * s[end])  # concat chars first * last
  part1 = 0
  part2 = 0
  for line in lines
    nums_in_string = String([x for x in line if isdigit(x)])
    line_first_end_num = digits_to_num(nums_in_string)
    part1 += line_first_end_num
    replacements = [
      "one" => "o1e",
      "two" => "t2o",
      "three" => "t3e",
      "four" => "f4r",
      "five" => "f5e",
      "six" => "s6x",
      "seven" => "s7n",
      "eight" => "e8t",
      "nine" => "n9e"
    ]
    line = replace(line, replacements...)
    line = replace(line, replacements...)
    nums_in_string = String([x for x in line if isdigit(x)])
    line_first_end_num = digits_to_num(nums_in_string)
    part2 += line_first_end_num
  end
  return [part1, part2]
end

end # module Day01

Day01.day01()
