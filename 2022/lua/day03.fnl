(local input-file :../inputs/day03.txt)
(local input (with-open [file (assert (io.open input-file))]
               (icollect [i _ (file:lines)] i)))

(fn read-n-lines [n]
  "reads n lines from input"
  (var i 1)
  (var total (length input))
  (fn []
    (set i (+ n i))
    (if (> i total) nil (fcollect [j (- i n) (- i 1)] (. input j)))))

(fn split [[line]]
  (local half (/ (length line) 2))
  [(line:sub 1 half) (line:sub (+ half 1))])

(fn get-set-from-str [s]
  (let [unique {}]
    (s:gsub "." #(tset unique $1 true))
    unique))

(fn get-value [c] ; branchless alternative to if isLower(c)
  (math.fmod (- (c:byte) 38) 58))

(fn get-matches [groups]
  "for each group in groups, calculate set, and find intersection"
  (var current-matches (get-set-from-str (. groups 1)))
  (for [i 2 (length groups)]
    (->> (accumulate [matches {} f _ (pairs (get-set-from-str (. groups i)))]
           (if (. current-matches f)
               (doto matches (tset f true))
               matches))
         (set current-matches)))
  (accumulate [value 0 f _ (pairs current-matches)] (get-value f)))

(fn part-1 []
  (accumulate [total 0 lines (read-n-lines 1) &until (= 0 (length lines))]
    (->> lines
         (split)
         (get-matches)
         (+ total))))

(fn part-2 []
  (accumulate [total 0 lines (read-n-lines 3) &until (= 0 (length lines))]
    (->> lines
         (get-matches)
         (+ total))))

(print "Part 1:" (part-1))
(print "Part 2:" (part-2))

