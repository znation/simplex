#define UNIT_TESTING
#include "../evaluator.h"
#include "../errors.h"

#include "catch.h"
#include "macros.h"

using namespace simplex;

TEST_CASE("append") {
  Evaluator e;
  CHECK(e.eval("(append (list) (list 1 2 3))") == e.eval("(list 1 2 3)"));
  CHECK(e.eval("(append (list 1) (list 2 3))") == e.eval("(list 1 2 3)"));
  CHECK(e.eval("(append (list 1 2) (list 3 4))") == e.eval("(list 1 2 3 4)"));
  CHECK(e.eval("(append (list 1 2) (list 3))") == e.eval("(list 1 2 3)"));
  CHECK(e.eval("(append (list 1 2 3) (list))") == e.eval("(list 1 2 3)"));
}

TEST_CASE("operators [stdlib]") {
  Evaluator e;
  CHECK_MATH_2(<=, 2, 3, true);
  CHECK_MATH_2(<=, 3, 3, true);
  CHECK_MATH_2(<=, 4, 3, false);
  CHECK_MATH_2(>=, 2, 3, false);
  CHECK_MATH_2(>=, 3, 3, true);
  CHECK_MATH_2(>=, 4, 3, true);
}

TEST_CASE("len") {
  Evaluator e;
  CHECK(e.eval("(len (list))") == 0);
  CHECK(e.eval("(len (list 1))") == 1);
  CHECK(e.eval("(len (list 1 2))") == 2);
  CHECK(e.eval("(len (list 1 2 3))") == 3);
}

TEST_CASE("reverse") {
  Evaluator e;
  CHECK(e.eval("(reverse (list))") == e.eval("(list)"));
  CHECK(e.eval("(reverse (list 1))") == e.eval("(list 1)"));
  CHECK(e.eval("(reverse (list 1 2))") == e.eval("(list 2 1)"));
  CHECK(e.eval("(reverse (list 1 2 3))") == e.eval("(list 3 2 1)"));
  CHECK(e.eval("(reverse '')") == e.eval("''"));
  CHECK(e.eval("(reverse 'a')") == e.eval("'a'"));
  CHECK(e.eval("(reverse 'ab')") == e.eval("'ba'"));
  CHECK(e.eval("(reverse 'hello')") == e.eval("'olleh'"));
}

TEST_CASE("readLine") {
  std::stringstream input;
  Evaluator e(input, std::cout);
  input << "a\nb\n+ 3 4\n- 5 6";
  CHECK(e.eval("(readLine)") == e.eval("'a'"));
  CHECK(e.eval("(readLine)") == e.eval("'b'"));
  CHECK(e.eval("(readLine)") == e.eval("'+ 3 4'"));
  CHECK(e.eval("(readLine)") == e.eval("'- 5 6'"));
}
