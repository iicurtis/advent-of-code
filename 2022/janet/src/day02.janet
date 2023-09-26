#!/bin/env janet
# sourced from https://github.com/tionis/adventofcode/blob/main/2022/2/main.janet
(defn conv [a] (a 0))   # janet lets you call the array and use index as arg; indexing into string gets byte
# xyz + 1 + (xyz - abc + 4)%3 * 3
# (abc + xyz + 2)%3 + 1 + ((abc + xyz + 2)%3 - abc + 4)%3 * 3
(defn outcome [abc xyz] (+ 1 xyz (* (% (- xyz abc -4) 3) 3)))

(defn run []
  (def results (peg/match ~{
                            :l (cmt (<- :a) ,conv)
                            :1 (cmt (* :l (constant ,(chr "A"))) ,-)  # , calls array; (chr "A") = 65
                            :2 (cmt (* :l (constant ,(chr "X"))) ,-)  # , calls array; (chr "A") = 65
                            :line (* (cmt (* :1 " " :2) ,tuple) "\n")
                            :main (some :line)}
                          (slurp "../inputs/day02.txt")))
  (printf "Part 1: %P" (sum (map |(outcome ;$) results)))
  (printf "Part 2: %P" (sum (map |(outcome ($ 0) (% (+ 2 ;$) 3)) results))))  # why does this work????
(run)
