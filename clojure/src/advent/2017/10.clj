(ns advent.2017.10
  (:require [clojure.string :refer [split]]))

(defn reverse-subseq
  [length idx s]
  (let [idx (rem idx (count s))
        overflow (- (+ idx length) (count s))
        underflow (- (count s) idx length)
        left (take (- idx (if (pos? overflow) overflow 0)) (drop overflow s))
        reversed (reverse (take length (drop idx (cycle s))))
        right (take underflow (drop (+ idx length) s))]
    (concat (take-last overflow reversed)
            left
            (drop-last overflow reversed)
            right)))

(defn reduce-step
  [{:keys [values index skip-size]} length]
  (let [next-values (reverse-subseq length index values)
        next-index (+ index length skip-size)
        next-skip-size (inc skip-size)]
    {:values next-values
     :index next-index
     :skip-size next-skip-size}))

(defn solve-1
  ([input] (solve-1 input (range 0 256)))
  ([input values]
   (let [lengths (map #(Integer/parseInt %) (split input #","))]
    (let [{:keys [values]} (reduce reduce-step
                                   {:values values
                                    :index 0
                                    :skip-size 0}
                                   lengths)]
      (* (first values) (second values))))))

(def salt '(17 31 73 47 23))

(defn solve-2
  ([input] (solve-2 input (range 0 256)))
  ([input values]
   (let [lengths (concat (map int input) salt)]
     (let [{:keys [values]} (reduce reduce-step
                                    {:values values
                                     :index 0
                                     :skip-size 0}
                                    (take (* (count lengths) 64)
                                          (cycle lengths)))
           dense-hash (map #(apply bit-xor %) (partition 16 values))]
       (apply str (map #(format "%02x" %) dense-hash))))))
