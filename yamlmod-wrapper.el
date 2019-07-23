;;; yamlmod-wrapper.el - Wraps yamlmod dynamic module in elisp

;; Copyright (C) 2019 by Ayush Goyal

;; Author: Ayush Goyal <perfectayush@gmail.com>
;; URL: https://github.com/perfectayush/emacs-yamlmod
;; Version: 0.1.0


(require 'yamlmod)
(require 'f)

;;;###autoload
(defun yamlmod-read-file (file)
  (yamlmod-load (f-read-text file 'utf-8)))

(provide 'yamlmod-wrapper)
