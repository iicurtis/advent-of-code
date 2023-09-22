module Day05
using AdventOfCode2022

function day05(input::String = joinpath(@__DIR__, "..", "..", "inputs", "day05.txt"))
    input = open(input, "r") do file
        read(file, String)
    end
    stack_input, proc_input = split(input, "\n\n")
    stack_lines = split(stack_input, "\n")
    num_stacks = (length(stack_lines[end]) + 1) / 4
    stacks = [Char[] for _ in 1:num_stacks]
    for line in stack_lines
        for (i, c) in enumerate(line[2:4:end])
            if c != ' '
                push!(stacks[i], c)
            end
        end
    end
    for stack in stacks
        reverse!(stack)
    end

    for line in split(rstrip(proc_input), "\n")
        command = split(line)
        num = parse.(Int, command[2])
        from = parse.(Int, command[4])
        to = parse.(Int, command[6])
        for _ = 1:num
            push!(stacks[to], pop!(stacks[from]))
        end
    end
    part1 = join(map(last, stacks))
    return [part1, part2(input)]
end

function part2(input::String)
    stack_input, proc_input = split(input, "\n\n")
    stack_lines = split(stack_input, "\n")
    num_stacks = (length(stack_lines[end]) + 1) / 4
    stacks = [Char[] for _ in 1:num_stacks]
    for line in stack_lines
        for (i, c) in enumerate(line[2:4:end])
            if c != ' '
                push!(stacks[i], c)
            end
        end
    end
    for stack in stacks
        reverse!(stack)
    end

    for line in split(rstrip(proc_input), "\n")
        num, from, to = parse.(Int, split(line)[2:2:6])
        len = length(stacks[from])
        push!(stacks[to], splice!(stacks[from], len-num+1:len)...)
    end
    return join(map(last, stacks))
end


end #module
