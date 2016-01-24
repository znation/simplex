#include "catch.h"

#define UNIT_TESTING
#include "../evaluator.h"

using namespace simplex;


TEST_CASE("Evaluator") {
  Evaluator e;

  SECTION("math") {
    CHECK(e.eval("(+ 4 3)") == 7);
  }
}
