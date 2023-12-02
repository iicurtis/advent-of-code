(def inp (as->
           (slurp "../inputs/day01.txt") _
           (string/trim _)
           (string/split "\n\n" _)
           (map (partial string/split "\n") _)
           (map (partial map scan-number) _)
           (map sum _)
           (sort _ >)
           (take 3 _)))
(defn run []
  (print "Day 01:")
  (printf " Part 1: %P" (in inp 0))
  (printf " Part 2: %P" (sum inp)))
