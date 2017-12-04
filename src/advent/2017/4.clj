(ns advent.2017.4
  (:require [clojure.string :refer [split]]))

(defn parse-passphrase
  [input]
  (split input #"\s+"))

(defn no-duplicate-words?
  [passphrase]
  (= (count (set passphrase)) (count passphrase)))

(defn count-valid-passphrases
  [validity-fn passphrases]
  (let [splat (split passphrases #"\n")]
    (count (filter #(-> % parse-passphrase validity-fn) splat))))

(def count-no-duplicate-passphrases
  (partial count-valid-passphrases no-duplicate-words?))

(defn no-duplicate-anagrams?
  [passphrase]
  (let [freqs (map frequencies passphrase)]
    (= (count (set freqs)) (count passphrase))))

(def count-no-duplicate-anagram-passphrases
  (partial count-valid-passphrases no-duplicate-anagrams?))
