(ns advent.2017.1)

(defn parse-string [s]
  (->> s
       (map #(Character/getNumericValue %))
       vec))

(defn captcha
  [step s]
  (let [parsed (parse-string s)
        len (count parsed)]
    (reduce-kv
      (fn [r k v]
        (if (= v (nth parsed (mod (+ k step) len)))
          (+ r v)
          r)) 0 parsed)))

(def captcha-next (partial captcha 1))

(defn captcha-halfway
  [s]
  (let [step (/ (count s) 2)]
    (captcha step s)))
