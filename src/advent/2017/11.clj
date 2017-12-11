(ns advent.2017.11
  (:require [clojure.string :refer [split]]))

(def direction-fns
  {:n [identity dec]
   :ne [inc identity]
   :se [inc inc]
   :s [identity inc]
   :sw [dec identity]
   :nw [dec dec]})

(defn apply-direction
  [direction coords]
  (mapv #(%1 %2) (direction-fns direction) coords))

(defn apply-directions
  [directions]
  (reduce #(apply-direction %2 %1) [0 0] directions))

(def not-zero? (complement zero?))

(defn home-step
  [coords steps]
  (let [[x y] coords]
    (cond (every? pos? coords) {:coords (map #(- % (apply min coords)) coords)
                                :steps (+ steps (apply min coords))}
          (every? neg? coords) {:coords (map #(- % (apply max coords)) coords)
                                :steps (+ steps (Math/abs (apply max coords)))}
          (not-zero? y) {:coords [x 0]
                         :steps (+ steps (Math/abs y))}
          (not-zero? x) {:coords [0 y]
                         :steps (+ steps (Math/abs x))}
          :else {:coords coords
                 :steps steps})))

(defn go-home
  [coords]
  (loop [coords coords
         steps 0]
    (let [{:keys [coords steps]} (home-step coords steps)]
      (if (= coords [0 0])
        steps
        (recur coords steps)))))

(defn solve-1
  [input]
  (let [parsed (map keyword (split input #","))
        coords (apply-directions parsed)]
    (go-home coords)))

(defn apply-directions-steps
  [directions]
  (reductions #(apply-direction %2 %1) [0 0] directions))

(defn max-step
  [steps]
  (apply max-key go-home steps))

(defn solve-2
  [input]
  (let [parsed (map keyword (split input #","))
        steps (apply-directions-steps parsed)]
    (go-home (max-step steps))))
