(ns advent.2017.9)

(defn score
  [input]
  (loop [idx 0
         closing-chars-needed 0
         current-score 0
         not-junk? true
         junk-amount 0]
    (if (< idx (count input))
      (let [c (nth input idx)]
        (cond
          (= c \!) (recur (+ 2 idx)
                          closing-chars-needed
                          current-score
                          not-junk?
                          junk-amount)
          (and (= c \{) not-junk?) (recur (inc idx)
                                          (inc closing-chars-needed)
                                          current-score
                                          not-junk?
                                          junk-amount)
          (and (= c \}) not-junk?) (recur (inc idx)
                                          (dec closing-chars-needed)
                                          (+ current-score closing-chars-needed)
                                          not-junk?
                                          junk-amount)
          (and (= c \<) not-junk?) (recur (inc idx)
                                          closing-chars-needed
                                          current-score
                                          false
                                          junk-amount)
          (and (= c \>)) (recur (inc idx)
                                closing-chars-needed
                                current-score
                                true
                                junk-amount)
          (and (= c \,) not-junk?) (recur (inc idx)
                                          closing-chars-needed
                                          current-score
                                          not-junk?
                                          junk-amount)
          :else (recur (inc idx)
                       closing-chars-needed
                       current-score
                       not-junk?
                       (inc junk-amount))))
      [current-score junk-amount])))

(defn solve-1
  [input]
  (first (score input)))

(defn solve-2
  [input]
  (second (score input)))
