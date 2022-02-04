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
  CHECK(parse("(+ 3 4)(- 3 4)") == parse(" ( + 3 4)\n\r\n( - 3 4  )\n"));
  CHECK(parse("'\nasdf\r\n'").children()[0].children()[0].string() == std::string("\nasdf\r\n"));
}
