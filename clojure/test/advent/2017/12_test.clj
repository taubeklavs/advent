(ns advent.2017.12-test
  (:require [advent.2017.12 :as sut]
            [clojure.test :refer :all]))

(def input
  "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5")

(deftest solve-1
  (is (= (sut/solve-1 input) 6)))
