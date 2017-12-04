(ns advent.2017.3)

;; Part 1

(defn diameter
  [v]
  (let [d (int (Math/ceil (Math/sqrt v)))]
    (if (even? d)
      (inc d)
      d)))

(defn corners
  [d]
  (let [max-corner (int (Math/pow d 2))
        min-x-corner (- max-corner (dec d))
        min-corner (- min-x-corner (dec d))
        min-y-corner (- min-corner (dec d))
        max-loc (int (/ d 2))
        min-loc (* max-loc -1)]
    [{:x min-loc
      :y max-loc
      :idx min-x-corner
      :dir-fn #(update %1 :x + %2)}
     {:x min-loc
      :y min-loc
      :idx min-corner
      :dir-fn #(update %1 :y + %2)}
     {:x max-loc
      :y min-loc
      :idx min-y-corner
      :dir-fn #(update %1 :x - %2)}
     {:x max-loc
      :y (dec max-loc)
      :idx (- min-y-corner (- d 2))
      :dir-fn #(update %1 :y - %2)}]))

(defn corner [idx d]
  (let [cs (corners d)]
    (first (filter #(>= idx (:idx %)) cs))))

(defn coords [idx]
  (let [d (diameter idx)
        c (corner idx d)
        delta (- idx (:idx c))]
    (select-keys ((:dir-fn c) c delta) [:x :y])))

(defn manhattan-distance [idx]
  (let [c (coords idx)]
    (apply + (map #(Math/abs %) (vals c)))))

;; Part 2

(defn next-coords
  [{:keys [x y m v] :as coord}]
  (let [min-val (* -1 m)
        max-val m]
    (cond
      (and (= x y) ((complement neg?) x)) {:x (inc x)
                                           :y y
                                           :m (inc max-val)
                                           :v v}
      (and (= x max-val) (> y min-val)) (update coord :y dec)
      (and (= y min-val) (> x min-val)) (update coord :x dec)
      (and (= x min-val) (< y max-val)) (update coord :y inc)
      (and (= y max-val) (< x max-val)) (update coord :x inc))))

(defn adjacent?
  [c1 c2]
  (let [c1-x (:x c1)
        c1-y (:y c1)
        c2-x (:x c2)
        c2-y (:y c2)]
    (and (<= (Math/abs (- c1-x c2-x)) 1)
         (<= (Math/abs (- c1-y c2-y)) 1))))

(defn smallest-larger [v]
  (loop [coord {:x 0 :y 0 :m 0 :v 1}
         coords [coord]]
    (if (> (:v coord) v)
      (:v coord)
      (let [next-coord (next-coords coord)
            adjacent (filter #(adjacent? next-coord %) coords)
            next-coord (-> coord
                           next-coords
                           (assoc :v (apply + (map :v adjacent))))]
        (recur next-coord
               (conj coords next-coord))))))
