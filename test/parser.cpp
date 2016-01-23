#include "catch.h"
#include "../parser.h"

using namespace simplex::Parser;

TEST_CASE("parser") {
  CHECK(parse("(+ 3 4)") == parse(" ( +  3  4 ) "));
}
