module Day01

function day01(input::String=joinpath(@__DIR__, "..", "..", "inputs", "day01.txt"))
  input = open(input, "r") do file
    read(file, String)
  end
  cal_sum = map(x -> parse.(Int, eachsplit(x)) |> sum, eachsplit(input, "\n\n"))
  partialsort!(cal_sum, 1:3, rev=true)
  return [cal_sum[1], cal_sum[1:3] |> sum]
end

end # module

Day01.day01()
