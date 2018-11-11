(ns advent.2017.8
  (:require [clojure.string :refer [split]]))

(defn str->fn
  [s]
  (condp = s
    "inc" +
    "dec" -
    "==" =
    "!=" not=
    "<" <
    "<=" <=
    ">" >
    ">=" >=))

(defn get-register
  [register registers]
  (or (registers register) 0))

(defn parse-condition
  [s]
  (let [[register conditional value] (split s #"\s+" )
        register (keyword register)
        conditional (str->fn conditional)
        value (Integer/parseInt value)]
    (fn [registers] (conditional (get-register register registers) value))))

(defn parse-instruction
  [instruction]
  (let [[_ register action value condition]
        (re-find #"([^ ]+) ([^ ]+) (\-?\d+) if (.+)" instruction)]
    {:register (keyword register)
     :action (str->fn action)
     :value (Integer/parseInt value)
     :condition (parse-condition condition)}))

(defn apply-action
  [{:keys [register action value]} registers]
  (let [old-value (get-register register registers)
        new-value (action old-value value)]
    (assoc registers register new-value)))

(defn apply-instruction
  [registers instruction]
  (let [{:keys [condition] :as instruction} (parse-instruction instruction)]
    (if (condition registers)
      (apply-action instruction registers)
      registers)))

(defn split-lines [input]
  (split input #"\n"))

(defn solve-1
  [input]
  (let [instructions (split-lines input)]
    (apply max (vals (reduce apply-instruction {} instructions)))))

(defn solve-2
  [input]
  (let [instructions (split-lines input)]
    (->> instructions
         (reductions apply-instruction {})
         (remove empty?)
         (map vals)
         flatten
         (apply max))))
