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
  (print "Part 1: " (in inp 0))
  (print "Part 2: " (sum inp)))
