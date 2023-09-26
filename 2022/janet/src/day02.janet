#!/bin/env janet
# sourced from https://github.com/tionis/adventofcode/blob/main/2022/2/main.janet
(defn outcome [a b] (+ 1 b (* (% (- b a -4) 3) 3)))
(defn conv [a] (a 0))

(defn run []
  (def results (peg/match ~{
                            :l (cmt (<- :a) ,conv)
                            :1 (cmt (* :l (constant ,(chr "A"))) ,-)
                            :2 (cmt (* :l (constant ,(chr "X"))) ,-)
                            :line (* (cmt (* :1 " " :2) ,tuple) "\n")
                            :main (some :line)}
                          (slurp "../inputs/day02.txt")))
  (printf "Part 1: %P" (sum (map |(outcome ;$) results)))
  (printf "Part 2: %P" (sum (map |(outcome ($ 0) (% (+ 2 ;$) 3)) results))))
(run)
