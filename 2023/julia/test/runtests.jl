using AdventOfCode2023
using Test

@testset "Day 1" begin
  sample = """
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
    """
  @test AdventOfCode2023.Day01.day01(sample) == [142, 142]
  @test AdventOfCode2023.Day01.day01() == [55123, 55260]
end
