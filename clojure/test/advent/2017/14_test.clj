(ns advent.2017.14-test
  (:require [advent.2017.14 :as sut]
            [clojure.test :refer :all]))

(deftest solve-1
  (is (= (sut/solve-1 "flqrgnkx") 8108)))
