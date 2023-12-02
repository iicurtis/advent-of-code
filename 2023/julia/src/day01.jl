module Day01

function day01(input::String=joinpath(@__DIR__, "..", "..", "inputs", "day01.txt"))
  part1 = 0
  input = open(input, "r") do file
    while (true)
      line = readline(file)
      if line == ""
        break
      end

      first_digit = -1
      last_digit = -1

      for char in line
        if char >= '0' && char <= '9'
          if first_digit == -1
            first_digit = char - '0'
          end
          last_digit = char - '0'
        end
      end

      if first_digit != -1
        part1 += first_digit * 10 + last_digit
      end
    end
  end
  return [part1, 0]
end

end # module

Day01.day01()
