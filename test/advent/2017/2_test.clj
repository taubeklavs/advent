(ns advent.2017.2-test
  (:require [advent.2017.2 :as sut]
            [clojure.test :refer :all]))

(def input
  (str "5 1 9 5\n"
       "7 5 3\n"
       "2 4 6 8"))

(deftest checksum-max-min-test
  (is (= (sut/checksum-max-min input) 18)))

(def input-2
  (str "5 9 2 8\n"
       "9 4 7 3\n"
       "3 8 6 5"))

(deftest checksum-evenly-divisable
  (is (= (sut/checksum-evenly-divisable input-2) 9)))
