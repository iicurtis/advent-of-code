(local input-file :../inputs/day03.txt)

(fn get-set-from-str [s]
  (let [unique {}] (s:gsub "." #(tset unique $1 true)) unique))

(fn split [[line]]
  (local half (/ (length line) 2))
  [(line:sub 1 half) (line:sub (+ half 1))])

(fn get-value [c]   ; branchless alternative to if isLower(c)
  (let [b (c:byte)] (math.fmod (- b 38) 58)))

(fn part-1 []
  (with-open [f (io.open input-file :r)]
             (accumulate [total 0 line (f:lines) &until (= line "")]
                        (let [(first last) (split line)
                              first-set (get-set-from-str first) last-set (get-set-from-str last)]
                            (-> (accumulate [c "" f _ (pairs first-set)]
                                  (if (. last-set f) f c))
                                (get-value)
                                (+ total))))))

(local file (assert (io.open input-file :r)))
(local read-line (file:lines))

(fn read-n-lines [n]
  "reads n lines from file"
  (fn []
    (fcollect [_ 1 n] (read-line))))

(fn get-matches [groups]
  "for each group in groups, calculate set, and find intersection"
  (local sets {})
  (each [_ group (ipairs groups)]
    (table.insert sets (get-set-from-str group)))
  (var current-matches (. sets 1))
  (for [i 2 (length sets)]
    (->>  (accumulate [matches {} f _ (pairs (. sets i))]
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


(print "Part 1:" (part-1-2))
(file:seek :set)
(print "Part 2:" (part-2))
