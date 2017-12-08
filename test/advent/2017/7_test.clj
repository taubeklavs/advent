(ns advent.2017.7-test
  (:require [advent.2017.7 :as sut]
            [clojure.test :refer :all]))

(def input
  "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)")

(deftest find-parent
  (is (= (sut/solve-1 input) "tknk")))

(deftest balance-tree
  (is (= (sut/solve-2 input) 60)))
