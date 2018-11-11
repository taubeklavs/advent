(ns advent.2017.5
  (:require [clojure.string :refer [split]]))

(defn parse-input
  [input]
  (vec (map #(Integer/parseInt %) (split input #"\n"))))

(defn escape-maze
  [update-fn input]
  (let [initial-maze (parse-input input)]
    (loop [maze initial-maze
           sum 0
           location 0]
      (if (>= location (count maze))
        sum
        (let [v (nth maze location)]
          (recur (update maze location (update-fn v))
                 (inc sum)
                 (+ location v)))))))

(defn escape-maze-part-1
  [input]
  (escape-maze (fn [_] inc) input))

(defn escape-maze-part-2
  [input]
  (escape-maze #(if (< % 3) inc dec) input))
