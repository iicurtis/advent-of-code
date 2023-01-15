def part1:
  [
    [inputs
    | explode
    | unique ]
    | recurse(.[3:]; length > 0)[:3]
    | .[0] - (.[0] - (.[2] - (.[2] - .[1])))
    | (first - 38) % 58
  ] | add
;

def part2:
  [
    [inputs
    | explode
    | unique ]
    | recurse(.[3:]; length > 0)[:3]
    | .[0] - (.[0] - (.[2] - (.[2] - .[1])))
    | (first - 38) % 58
  ] | add
;

part1
