#define UNIT_TESTING
#include "../evaluator.h"
#include "../errors.h"

#include "catch.h"

using namespace simplex;

TEST_CASE("append") {
  Evaluator e;
  CHECK(e.eval("(append (list) (list 1 2 3))") == e.eval("(list 1 2 3)"));
  CHECK(e.eval("(append (list 1) (list 2 3))") == e.eval("(list 1 2 3)"));
  CHECK(e.eval("(append (list 1 2) (list 3 4))") == e.eval("(list 1 2 3 4)"));
  CHECK(e.eval("(append (list 1 2) (list 3))") == e.eval("(list 1 2 3)"));
  CHECK(e.eval("(append (list 1 2 3) (list))") == e.eval("(list 1 2 3)"));
}
