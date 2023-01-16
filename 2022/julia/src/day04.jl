module Day04
using AdventOfCode2022

function day04(input::String = joinpath(@__DIR__, "..", "..", "inputs", "day04.txt"))
    input = open(input, "r") do file
        read(file, String)
    end
    input_lines = map(s -> parse.(Int, s), split.(split(rstrip(input), "\n"), c -> c in ",-"))
    part1 = 0
    part2 = 0
    for assignment in input_lines
        if assignment[3] > assignment[1] && (assignment[2] > assignment[4])
            part1 += 1
        elseif (assignment[1] == assignment[3]) || (assignment[2] == assignment[4])
            part1 += 1
        elseif (assignment[1] > assignment[3]) && (assignment[4] > assignment[2])
            part1 += 1
        end
        if (assignment[2] >= assignment[3]) && (assignment[1] <= assignment[4])
            part2 += 1
        elseif (assignment[4] >= assignment[1]) && (assignment[3] <= assignment[2])
            part2 += 1
        end
    end
    return [part1, part2]
end

function value(c::Char) 
    return islowercase(c) ? c - 'a' + 1 : c - 'A' + 27
end

end #module
