(ns advent.2017.5-test
  (:require [advent.2017.5 :as sut]
            [clojure.test :refer :all]))

(deftest escape-maze
  (is (= 5 (sut/escape-maze "0 3 0 1 -3"))))
