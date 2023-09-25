module Day02

function day02(input::String=joinpath(@__DIR__, "..", "..", "inputs", "day02.txt"))
  input = open(input, "r") do file
    read(file, String)
  end
  rounds = map.(x -> Int(x[1]), eachsplit.(eachsplit(rstrip(input), "\n")))
  part1 = 0
  part2 = 0
  for x in rounds
    xyz = x[2] - Int('X')
    abc = x[1] - Int('A')
    outcome = [3, 6, 0][(xyz+3-abc)%3+1]
    part1 += outcome + xyz + 1
    outcome = (abc + xyz + 2) % 3 + 1 + xyz * 3
    part2 += outcome
  end
  return [part1, part2]
end
end # module

Day02.day02()
