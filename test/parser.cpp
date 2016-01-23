#include "catch.h"
#include "../parser.h"

using namespace simplex::Parser;

TEST_CASE("parser") {
  REQUIRE(parse("(+ 3 4)") == parse(" ( +  3  4 ) "));
}
