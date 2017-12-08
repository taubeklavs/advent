(ns advent.2017.7
  (:require [clojure.string :refer [split]]))

(defn parse-weight
  [weight]
  (Integer/parseInt (subs weight 1 (dec (count weight)))))

(defn parse-node
  [node]
  (let [splat (split node #"\s+")]
    [(keyword (first splat)) (parse-weight (second splat))]))

(defn parse-children
  [children]
  (when children (map keyword (split children #",\s+"))))

(defn parse-row
  [rows row]
  (let [[node children] (split row #" -> ")
        [name weight] (parse-node node)
        children (parse-children children)]
    (assoc rows name {:children children
                      :weight weight})))

(defn apply-children
  [name children nodes]
  (if children
    (reduce (fn [r child] (assoc-in r [child :parent] name)) nodes children)
    nodes))

(defn apply-parents
  [nodes]
  (let [nodes-with-children (filter :children nodes)]
    (reduce (fn [r [name {:keys [children]}]]
              (apply-children name children r))
            nodes
            nodes)))

(defn parentless-node
  [nodes]
  (into {} (filter #(-> % second :parent not) nodes)))

(defn parse-input
  [input]
  (let [rows (split input #"\n")]
    (reduce parse-row {} rows)))

(defn solve-1 [input]
  (let [parsed (parse-input input)]
    (-> parsed apply-parents parentless-node key)))

(defn children
  [root tree]
  (select-keys tree (:children root)))

(defn subtree-weight
  [node tree]
  (let [children (children node tree)]
    (+ (if (not-empty children)
         (apply + (map (fn [[_ v]] (subtree-weight v tree)) children))
         0)
       (:weight node))))

(defn children-weights
  [children tree]
  (map (fn [[k v]] [k (subtree-weight v tree)]) children))

(defn imbalanced-node
  [weights]
  (let [grouped-weights (group-by second weights)]
    (when (> (count grouped-weights) 1)
      (->> grouped-weights
           (sort-by #(-> % second count))
           first
           second
           first
           first))))

(defn parent
  [{:keys [parent]} tree]
  (tree parent))

(defn balance-weight
  [imbalanced-node-weight imbalanced-subtree balanced-subtree]
  (- (+ imbalanced-node-weight balanced-subtree) imbalanced-subtree))

(defn balance-siblings
  [node tree]
  (let [siblings (children (parent node tree) tree)
        weights (map (fn [[_ v]]
                       [(:weight v) (subtree-weight v tree)])
                     siblings)
        [imbalanced balanced] (->> weights
                                   (group-by second)
                                   (sort-by #(-> % second count))
                                   (map (comp first second)))
        [imbalanced-node-weight imbalanced-subtree] imbalanced
        [_ balanced-subtree] balanced]
    (balance-weight imbalanced-node-weight
                    imbalanced-subtree
                    balanced-subtree)))

(defn solve-2
  [input]
  (let [tree (apply-parents (parse-input input))]
    (loop [root (first (vals (parentless-node tree)))]
      (let [children (children root tree)
            children-weights (children-weights children tree)
            imbalanced-node-name (imbalanced-node children-weights)]
        (if imbalanced-node-name
          (recur (tree imbalanced-node-name))
          (balance-siblings root tree))))))
