(ns advent.2017.12
  (:require [clojure.string :refer [split]]))

(defn parse-connections
  [connections]
  (map keyword (split connections #", ")))

(defn parse-row
  [row]
  (let [[_ id connections] (re-find #"(\d+) <-> (.+)" row)]
    [(keyword id) {:connections (parse-connections connections)}]))

(defn parse-input
  [input]
  (let [rows (split input #"\n")]
    (into {} (map parse-row rows))))

(defn visit-node
  [id graph]
  (assoc-in graph [id :visited?] true))

(declare travel-group)

(defn travel-connections
  [neighbours graph]
  (reduce (fn [graph neighbour] (travel-group neighbour graph))
          graph
          neighbours))

(defn travel-group
  [id graph]
  (let [{:keys [connections visited?]} (graph id)]
    (if visited?
      graph
      (travel-connections connections
                          (visit-node id graph)))))

(defn solve-1
  [input]
  (let [graph (parse-input input)
        travelled (travel-group :0 graph)]
    (->> travelled
         (map #(get-in % [1 :visited?]))
         (filter identity)
         count)))

(defn unvisited-node
  [graph]
  (->> graph
       (filter (complement #(get-in % [1 :visited?])))
       first))

(defn count-groups
  [graph]
  (loop [graph graph
         group-count 0]
    (if-let [[unvisited-id _] (unvisited-node graph)]
      (recur (travel-group unvisited-id graph) (inc group-count))
      group-count)))

(defn solve-2
  [input]
  (let [graph (parse-input input)]
    (count-groups graph)))
