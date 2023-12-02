#!/bin/env janet
(defn pri [a b] (- (a 0) b))
(defn com [arrs] ((invert (frequencies (mapcat keys (map frequencies arrs)))) (length arrs)))
(defn main [input-file]
  (def sacks (peg/match ~{
                          :up (* (<- (range "AZ")) (constant 38))
                          :lo (* (<- (range "az")) (constant 96))
                          :sack (* (cmt (some (cmt (+ :up :lo) ,pri)) ,tuple) "\n")
                          :main (some :sack)}
                        (slurp input-file)))
  (printf "Day 03:")
  (printf " Part 1: %P" (+;(map |(com (partition (/ (length $) 2) $)) sacks)))
  (printf " Part 2: %P" (+;(map com (partition 3 sacks)))))

(defn run [] (main "../inputs/day03.txt"))
(run)
