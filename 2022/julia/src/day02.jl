module Day02

using AdventOfCode2022

function day02(input::String = joinpath(@__DIR__, "..", "..", "inputs", "day02.txt"))
    input = open(input, "r") do file
        read(file, String)
    end
    rounds = map.(x -> Int(x[1]), eachsplit.(eachsplit(rstrip(input), "\n")))
    part1 = 0
    part2 = 0
    for x in rounds
        player = x[2] - Int('X')
        opponent = x[1] - Int('A')
        outcome = [3, 6, 0][(player + 3 - opponent) % 3 + 1]
        part1 += outcome + player + 1
        outcome = (opponent + player + 2) % 3 + 1 + player * 3
        part2 += outcome
    end
    return [part1, part2]
end
end # module
