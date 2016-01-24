#define UNIT_TESTING
#include "../evaluator.h"

#include "catch.h"

using namespace simplex;


TEST_CASE("Evaluator") {
  Evaluator e;

  SECTION("math") {
    CHECK(e.eval("(+ 4)") == 4);
    CHECK(e.eval("(+ 4 3)") == 7);
    CHECK(e.eval("(+ 34.2 5)") == 39.2);
    CHECK(e.eval("(- 2)") == -2);
    CHECK(e.eval("(- 3.45)") == -3.45);
    CHECK(e.eval("(- 10 2)") == 8);
  }
}
