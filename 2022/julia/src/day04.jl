module Day04
using AdventOfCode2022

function day04(input::String = joinpath(@__DIR__, "..", "..", "inputs", "day04.txt"))
    input = open(input, "r") do file
        read(file, String)
    end
    input_lines = map(s -> parse.(Int, s), split.(split(rstrip(input), "\n"), c -> c in ",-"))
    part1 = 0
    part2 = 0
    for (a,b,c,d) in input_lines
        if a:b ⊆ c:d || c:d ⊆ a:b
            part1 += 1
        end
        if !isempty(intersect(a:b, c:d))
            part2 += 1
        end
    end
    return [part1, part2]
end

end #module
