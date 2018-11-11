(ns advent.2017.8-test
  (:require [advent.2017.8 :as sut]
            [clojure.test :refer :all]))

(def input
  "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10")

(deftest solve-1
  (is (= (sut/solve-1 input) 1)))

(deftest solve-2
  (is (= (sut/solve-2 input) 10)))
