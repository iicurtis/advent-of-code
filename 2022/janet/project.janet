(declare-project
  :name "aoc_2022_janet"
  :description ```Advent of Code 2022 ```
  :version "0.0.0")

(declare-source
  :prefix "src"
  :source ["init.janet"
           "day01.janet"])

(declare-executable
  :name "aoc_2022_janet"
  :entry "src/init.janet"
  :install true)
