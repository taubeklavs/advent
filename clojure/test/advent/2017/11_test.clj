(ns advent.2017.11-test
  (:require [advent.2017.11 :as sut]
            [clojure.test :refer :all]))

(deftest solve-1
  (are [input result] (= (sut/solve-1 input) result)
    "ne,ne,ne" 3
    "ne,ne,sw,sw" 0
    "ne,ne,s,s" 2
    "se,sw,se,sw,sw" 3))
