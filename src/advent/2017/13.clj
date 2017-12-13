(ns advent.2017.13
  (:require [clojure.string :refer [split]]))

(defn parse-row
  [row]
  (->> row
       (re-find #"(\d+): (\d+)")
       rest
       (map #(Integer/parseInt %))))

(defn parse-input
  [input]
  (map parse-row (split input #"\n")))

(defn scanner-position
  [[layer depth]]
  (/ (mod layer (* 2 (dec depth))) 2))

(defn layer-severity
  [[layer depth :as firewall]]
  (if (zero? (scanner-position firewall))
    (* layer depth)
    0))

(defn solve-1
  [input]
  (let [firewalls (parse-input input)]
    (loop [severity 0
           firewalls firewalls]
      (if-let [firewall (first firewalls)]
        (recur (+ severity (layer-severity firewall))
               (rest firewalls))
        severity))))

(defn solve-2
  [input]
  (let [firewalls (parse-input input)]
    (loop [picoseconds-delayed 0]
      (if (not-any? (fn [[layer depth]]
                      (zero? (scanner-position [(+ picoseconds-delayed layer)
                                                depth])))
                    firewalls)
        picoseconds-delayed
        (recur (inc picoseconds-delayed))))))
