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
    CHECK(e.eval("(* 8 2)") == 16);
    CHECK(e.eval("(* (- 2) 24)") == -48);
    CHECK(e.eval("(* (- 1.5) 2)") == -3.0);
    CHECK(e.eval("(/ 8 2)") == 4);
    CHECK(e.eval("(/ (- 2) 24)") == 0);
    CHECK(e.eval("(/ (- 58) 3)") == -19);
    CHECK(e.eval("(/ (- 1.5) 2)") == -0.75);
    CHECK(e.eval("(/ 0.5 2.0)") == 0.25);
  }
}