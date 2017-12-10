(ns advent.2017.9-test
  (:require [advent.2017.9 :as sut]
            [clojure.test :refer :all]))

(deftest solve-1
  (are [input result] (= result (sut/solve-1 input))
    "{}" 1
    "{{{}}}" 6
    "{{},{}}" 5
    "{{{},{},{{}}}}" 16
    "{<a>,<a>,<a>,<a>}" 1
    "{{<ab>},{<ab>},{<ab>},{<ab>}}" 9
    "{{<!!>},{<!!>},{<!!>},{<!!>}}" 9
    "{{<a!>},{<a!>},{<a!>},{<ab>}}" 3))

(deftest solve-2
  (are [input result] (= result (sut/solve-2 input))
    "{<>}" 0
    "{<random characters>}" 17
    "{<<<<>}" 3
    "{<{!>}>}" 2
    "{<!!>}" 0
    "{<!!!>>}" 0
    "{<{o\"i!a,<{i<a>}" 10))
