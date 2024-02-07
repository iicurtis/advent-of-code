; Magic function sourced from https://github.com/verdammelt/advent-of-code
; Seems to allow me to automatically grab every file in a directory
(defclass auto-module (module)
  ((file-cache :initform (make-hash-table))))

(defmethod component-children ((self auto-module)
                               &aux (file-cache (slot-value self 'file-cache)))
  (mapcar (lambda (p &aux (existing (gethash p file-cache)))
            (if existing
                existing
                (setf (gethash p file-cache)
                      (make-instance 'cl-source-file :type "lisp"
                                     :pathname p
                                     :name (pathname-name p)
                                     :parent (component-parent self)))))
          (directory-files (component-pathname self)
                           (make-pathname :directory nil :name *wild* :type "lisp"))))

(asdf:defsystem "advent-of-code-2022"
  :description "Advent of Code 2022"
  :version "0.0.0"
  :author "Isaac Curtis <iicurtis@outlook.com>"
  :licence "GNU General Public License (GPL) version 3"
  :depends-on ("fset" "alexandria" "serapeum" "uiop" "swank")
  :components ((:file "package")
               (:module "utils" :depends-on ("package") :pathname "" :components ((:file "utils")))
               (:auto-module "src" :depends-on ("utils"))))
