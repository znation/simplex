#define UNIT_TESTING
#include "../evaluator.h"

#include "catch.h"

using namespace simplex;

#define STR(s) #s
#define CHECK_MATH_1(op, p, result) \
  CHECK(e.eval(STR((op p))) == result); \
  CHECK(e.eval(STR((= (op p) result))) == true); \


#define CHECK_MATH_2(op, p1, p2, result) \
  CHECK(e.eval(STR((op p1 p2))) == result); \
  CHECK(e.eval(STR((= (op p1 p2) result))) == true); \


TEST_CASE("math [evaluator]") {
  Evaluator e;
  CHECK_MATH_1(+, 4, 4);
  CHECK_MATH_2(+, 4, 3, 7);
  CHECK_MATH_2(+, 34.2, 5, 39.2);
  CHECK_MATH_1(-, 2, (- 2));
  CHECK_MATH_1(-, 3.45, (- 3.45));
  CHECK_MATH_2(-, 10, 2, 8);
  CHECK_MATH_2(*, 8, 2, 16);
  CHECK_MATH_2(*, (- 2), 24, (- 48));
  CHECK_MATH_2(*, (- 1.5), 2, (- 3.0));
  CHECK_MATH_2(/, 8, 2, 4);
  CHECK_MATH_2(/, (- 2), 24, 0);
  CHECK_MATH_2(/, (- 58), 3, (- 19));
  CHECK_MATH_2(/, (- 1.5), 2, (- 0.75));
  CHECK_MATH_2(/, 0.5, 2.0, 0.25);
}

TEST_CASE("lambda [evaluator]") {
  Evaluator e;
  CHECK(e.eval("((lambda x y (+ x y)) 3 4)") == 7);
  CHECK(e.eval("((lambda x (+ x 5)) 2)") == 7);
}

TEST_CASE("let [evaluation]") {
  Evaluator e;
  CHECK(e.eval("(let add (lambda x y (+ x y)))") == true);
  CHECK(e.eval("(let five 5)") == true);
  CHECK(e.eval("(+ five (add 1 2))") == 8);
}

TEST_CASE("sequence [evaluation]") {
  Evaluator e;
  CHECK(e.eval("(sequence (+ 3 4) (- 5 2))") == 3);
  CHECK(e.eval("(sequence (+ 3 4) (= 5 2))") == false);
}

TEST_CASE("let with sequence [evaluation]") {
  Evaluator e;
  CHECK(e.eval("(sequence (let a 2) (let b 9) (+ a b))") == 11);
}

TEST_CASE("cons") {
  Evaluator e;
  CHECK(e.eval("(let a (cons 3 4))") == true);
  CHECK(e.eval("(car a)") == 3);
  CHECK(e.eval("(cdr a)") == 4);
  CHECK(e.eval("(= (list 1 2 3) (cons 1 (cons 2 (cons 3 nil))))") == true);
}
