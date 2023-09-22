(local file (assert (io.open :../inputs/day01.txt :r)))
(io.input file)
(file:seek :set)

(fn get-cal [file]
  (accumulate [sum nil line (file:lines) &until (= line "")]
    (+ (or sum 0) (tonumber line))))

(fn iter-cal [file] (fn [] (get-cal file)))

(local collection (icollect [v (iter-cal file)] v))
(table.sort collection #(> $1 $2))
(print "Part 1:" (. collection 1))
(print "Part 2:" (+ (. collection 1) (. collection 2) (. collection 3)))

