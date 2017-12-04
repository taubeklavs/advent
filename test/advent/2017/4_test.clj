(ns advent.2017.4-test
  (:require [advent.2017.4 :as sut]
            [clojure.test :refer :all]))

(deftest no-duplicate-words?
  (are [input result]
      (= result (sut/no-duplicate-words? (sut/parse-passphrase input)))
    "aa bb cc dd ee" true
    "aa bb cc dd aa" false
    "aa bb cc dd aaa" true))

(deftest no-duplicate-anagrams?
  (are [input result]
      (= result (sut/no-duplicate-anagrams? (sut/parse-passphrase input)))
    "abcde fghij" true
    "abcde xyz ecdab" false
    "a ab abc abd abf abj" true
    "iiii oiii ooii oooi oooo" true
    "oiii ioii iioi iiio" false))
