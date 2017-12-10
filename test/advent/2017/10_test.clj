(ns advent.2017.10-test
  (:require [advent.2017.10 :as sut]
            [clojure.test :refer :all]))

(deftest solve-1
  (is (= (sut/solve-1 "3,4,1,5" (range 0 5)) 12)))

(deftest solve-2
  (are [input result] (= result (sut/solve-2 input))
    "" "a2582a3a0e66e6e86e3812dcb672a272"
    "AoC 2017" "33efeb34ea91902bb2f59c9920caa6cd"
    "1,2,3" "3efbe78a8d82f29979031a4aa0b16a9d"
    "1,2,4" "63960835bcdc130f0b66d7ff4f6a5a8e"))
