(ns advent.2017.3-test
  (:require [advent.2017.3 :as sut]
            [clojure.test :refer :all]))

(deftest manhattan-distance
  (are [input result] (= result (sut/manhattan-distance input))
    1 0
    12 3
    23 2
    1024 31))

(deftest smallest-larger
  (are [input result] (= result (sut/smallest-larger input))
    1 2
    4 5
    23 25
    26 54
    747 806))
