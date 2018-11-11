(ns advent.2017.2
  (:require [clojure.string :refer [split]]))

(defn parse-row [row]
  (let [cells (split row #"\s+")]
    (map #(Integer/parseInt %) cells)))

(defn parse-input [s]
  (let [rows (split s #"\n")]
    (map parse-row rows)))

(defn max-min
  [row]
  (let [min-val (apply min row)
        max-val (apply max row)]
    (- max-val min-val)))

(defn checksum [row-result-fn input]
  (let [rows (parse-input input)]
    (reduce + (map row-result-fn rows))))

(def checksum-max-min (partial checksum max-min))

(defn remaining-map
  [row k]
  (let [[left right] (split-at k row)]
    (concat (rest right) left)))

(defn even-division [v coll]
  (let [xf (comp (map #(/ v %))
                 (filter integer?))]
    (transduce xf + coll)))

(defn evenly-divisable
  [row]
  (reduce-kv
    (fn [r k v]
      (let [rem (remaining-map row k)
            division (even-division v rem)]
        (if (integer? division)
          (+ r division)
          r)))
    0
    (vec row)))

(def checksum-evenly-divisable (partial checksum evenly-divisable))
