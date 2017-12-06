(ns advent.2017.6-test
  (:require [advent.2017.6 :as sut]
            [clojure.test :refer :all]))

(deftest solve-1
  (is (= (sut/solve-1 "0 2 7 0") 5)))

(deftest solve-2
  (is (= (sut/solve-2 "0 2 7 0") 4)))
