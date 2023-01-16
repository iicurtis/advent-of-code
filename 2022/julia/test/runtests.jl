using AdventOfCode2022
using Test

@testset "Day 1" begin
  sample = """
    1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000
    """
  @test AdventOfCode2022.Day01.day01(sample) == [24000, 45000] 
  @test AdventOfCode2022.Day01.day01() == [69501, 202346]
end

@testset "Day 4" begin
  sample = """
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    """
  @test AdventOfCode2022.Day04.day04(sample) == [2, 4] 
  @test AdventOfCode2022.Day04.day04() == [562, 924]
end
