(ns advent.2017.13-test
  (:require [advent.2017.13 :as sut]
            [clojure.test :refer :all]))

(def input
  "0: 3
1: 2
4: 4
6: 4")

(deftest solve-1
  (is (= (sut/solve-1 input) 24)))

(deftest solve-2
  (is (= (sut/solve-2 input) 10)))
