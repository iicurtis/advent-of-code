;;;; Day 1: Calorie Counting
(in-package :aoc-2022)

(defparameter *day1-input* #P"../inputs/day01.txt")
(defparameter *day1-test-input* "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000")

(defun run-day1 ()
  (let ((input (uiop:read-file-lines *day1-input*)))
    (time (format t "Part 1: ~a~%" (day01-part1 input)))
    (time (format t "Part 2: ~a~%" (day01-part2 input)))))

(defun day01-part1 (input)
  (reduce #'max (read-cals input)))

(defun day01-part2 (input)
  (reduce #'+ (read-cals input) :end 3))

(defun read-cals (input)
  "Convert input into groups of calorie sums"
  (let ((cals (sort
                (mapcar (lambda (nums) (reduce #'+ (mapcar #'parse-integer nums)))
                        (split-sequence:split-sequence "" input :test #'equal))
                #'>)))
      cals))

(run-day1)
