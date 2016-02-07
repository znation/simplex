#include "catch.h"
#include "../errors.h"
#include "../parser.h"

using namespace simplex;
using namespace simplex::Parser;

TEST_CASE("parser") {
  CHECK(parse("(+ 3 4)") == parse(" ( +  3  4 ) "));
  CHECK_THROWS_AS(parse("(let 'asdf' 3"), ParseError);
  CHECK(parse("(let asdf 3)").kind() == NodeKind::program);
  CHECK(parse("(let asdf' 3)").kind() == NodeKind::program);
}
