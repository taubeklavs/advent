(ns advent.2017.1-test
  (:require [advent.2017.1 :as sut]
            [clojure.test :refer :all]))

(deftest captcha-next
  (is (= (sut/captcha-next "1122") 3))
  (is (= (sut/captcha-next "1111") 4))
  (is (= (sut/captcha-next "1234") 0))
  (is (= (sut/captcha-next "91212129") 9)))

(deftest captcha-halfway
  (is (= (sut/captcha-halfway "1212") 6))
  (is (= (sut/captcha-halfway "123425") 4))
  (is (= (sut/captcha-halfway "123123") 12))
  (is (= (sut/captcha-halfway "12131415") 4)))
