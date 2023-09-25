(asdf:defsystem "advent-of-code-2022"
  :description "Advent of Code 2022"
  :version "0.0.1"
  :author "Isaac Curtis <iicurtis@outlook.com>"
  :licence "GNU General Public License (GPL) version 3"
  :depends-on ("fset" "alexandria" "serapeum" "uiop")
  :components ((:file "package")
               (:file "day01" :depends-on ("package"))))
