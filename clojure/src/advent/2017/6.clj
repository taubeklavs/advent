(ns advent.2017.6
  (:require [clojure.string :refer [split]]))

(defn parse-input
  [input]
  (let [splat (split input #"\s+")]
    (vec (map #(Integer/parseInt %) splat))))

(defn max-item [coll]
  (->> coll
       (map-indexed vector)
       reverse
       (apply max-key second)))

(defn redistribute-partial
  [max-idx max-val coll]
  (let [indexes (->> coll
                     count
                     range
                     cycle
                     (drop (inc max-idx))
                     (take max-val))]
    (reduce (fn [r v] (update r v inc)) coll indexes)))

(defn redistribute
  [coll]
  (let [[max-idx max-val] (max-item coll)
        incrementor (int (/ max-val (count coll)))
        remainder (rem max-val (count coll))
        dropped-max (assoc (vec coll) max-idx 0)
        incremented (mapv #(+ incrementor %) dropped-max)]
    (redistribute-partial max-idx remainder incremented)))

(defn solve
  [res-fn input]
  (let [parsed (parse-input input)]
    (loop [coll parsed
           states {parsed 0}]
      (let [redistributed (redistribute coll)]
        (if (states redistributed)
          (res-fn redistributed states)
          (recur redistributed
                 (assoc states redistributed (count states))))))))

(defn state-count
  [_ states]
  (count states))

(def solve-1 (partial solve state-count))

(defn loop-size
  [redistributed states]
  (- (count states) (states redistributed)))

(def solve-2 (partial solve loop-size))
