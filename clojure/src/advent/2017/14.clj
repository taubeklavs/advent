(ns advent.2017.14
  (:require [advent.2017.10 :as ten]
            [advent.2017.12 :as twelve]
            [clojure.string :refer [split]]))

(defn binary
  [input]
  (let [ints (map #(Integer/parseInt % 16) (split input #""))
        binaries (map #(Integer/toString % 2) ints)]
    (apply str binaries)))

(defn grid
  [input]
  (apply str (map #(binary (ten/solve-2 (str input "-" %)))
                  (range 128))))

(defn solve-1 [input]
  (->> input
       grid
       (apply str)
       (re-seq #"1")
       count))
