(local input-file :../inputs/day02.txt)

(local outcomes [3 6 0])

(fn get-line [abc xyz]
  (local abc (- (string.byte abc) (string.byte :A)))
  (local xyz (- (string.byte xyz) (string.byte :X)))
  (values abc xyz))

(fn get-round [abc xyz]
  (local j (+ 1 (math.fmod (- xyz abc -3) 3)))
  (+ xyz (. outcomes j) 1))

(fn get-round-part2 [abc xyz]
  (+ 1 (math.fmod (+ abc xyz 2) 3) (* xyz 3)))

(fn play [get-round]
  (with-open [f (io.open input-file :r)]
             (accumulate [score 0 line (f:lines) &until (= line "")]
                         (-> (line:match "(.) (.)")
                             (get-line)
                             (get-round)
                             (+ score)))))

(print "Part 1:" (play get-round))
(print "Part 2:" (play get-round-part2))
