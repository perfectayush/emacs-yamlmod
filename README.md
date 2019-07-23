YamlMod
=======

## Overview
**YamlMod** is an emacs-module to parse yaml, written in Rust.

## Usage
YamlMod provides functions for parsing yaml string (`yamlmod-load`) 
and yaml file (`yamlmod-read-file`). Additionally, it provides a 
function `yamlmod-ypath-search` to get character index to navigate in a
yaml string. The output from `yamlmod-ypath-search` can be used to jump 
in a yaml file with `goto-char`

### Elisp Examples
```elisp
(yamlmod-load "yes")                                                  ;; => 't
(yamlmod-load "no")                                                   ;; => nil
(yamlmod-load "[1 2 3]")                                              ;; => ["1 2 3"]
(yamlmod-load "{a: 1, b: 2}")                                         ;; => #<hash-table equal 2/65 0x4e436fe1>

(setq test (yamlmod-load "{a: 1, b: 1}"))
(gethash "a" test)                                                    ;; => 2 (#o2, #x2, ?\C-b)


;; Parse a yaml file to elisp data structure
(setq file-yaml (yamlmod-read-file "/path/to/filename.yml"))

;; Get character index in a yaml string using ypath (a nested 
;; key lookup separated by period). The index starts with 1 for 
;; use with `goto-char`
(yamlmod-ypath-search "{a: [1, 2], b: {c: [3, 4]}}" "a")              ;; => 2
(yamlmod-ypath-search "{a: [1, 2], b: {c: [3, 4]}}" "a.2")            ;; => 9
(yamlmod-ypath-search "{a: [1, 2], b: {c: [3, 4]}}" "a.b")            ;; => 13
(yamlmod-ypath-search "{a: [1, 2], b: {c: [3, 4]}}" "a.b.2")          ;; => nil for not found

```

## Installation

### Build instruction
```shell
git clone https://github.com/perfectayush/emacs-yamlmod.git
cd emacs-yamlmod
make
```
Build requires `rust` and `cargo` to be pre-installed.

### Load in Emacs
```elisp
(add-to-load-path "/path/to/directory/for/emacs-yamlmod")
(require 'yamlmod-wrapper)
```
This module depends on [f.el](https://github.com/rejeep/f.el). It should be 
present on load path


## Motivation
This is an attempt to hack Emacs, along with learning some Rust. It's a simple
project with quickly perceivable results.

Another yaml parsing dynamic emacs module is available at
[emacs-libyaml](https://github.com/syohex/emacs-libyaml). I have hacked up on
emacs-libyaml in past, but it felt very low-level to code in C. Rust seemed a
good fit to achieve same feature set in a much simpler codebase.

Thanks to [emacs-module-rs](https://github.com/ubolonton/emacs-module-rs) and
[yaml-rust](https://github.com/chyh1990/yaml-rust) for making this
implementation a lot easier.

## Known Issues
- Currently, all boolean like strings "yes", "Yes", "no", "No", "True", "False"
  etc. are always parsed as 't or nil, even if they are quoted. 
  
## Todos
- Write a dump function to dump elisp data structure to yaml
