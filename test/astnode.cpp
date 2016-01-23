#include "catch.h"

#define UNIT_TESTING
#include "../astnode.h"

using namespace simplex;

static void append(std::vector<std::string>& vec, std::vector<std::string> other) {
  vec.insert(vec.end(), other.begin(), other.end());
}

static std::vector<std::string> whitespaceVariations(std::vector<std::string> input) {
  // inserts whitespace before, after, and before/after
  std::vector<std::string> ret;
  for (const auto& str : input) {
    ret.push_back(str);
    ret.push_back("  " + str);
    ret.push_back(str + "  ");
    ret.push_back(" " + str + " ");
  }
  return ret;
}

TEST_CASE("ASTNode") {
  std::vector<std::string> identifiers = {
      "identifier",
      "foo",
      "@#*&%&$#",
      "...",
      "💩"
  };

  std::vector<std::string> expressions = {
    "( + 3 4)",
    " (  +  3  4  ) "
  };
  append(expressions, whitespaceVariations(identifiers));

  std::vector<std::string> programs = expressions;
  for (size_t i=0; i<expressions.size() - 1; i++) {
    // test multiple expressions with or without whitespace between
    programs.push_back(expressions[0] + expressions[1]);
  }

  SECTION("parseProgram") {
    for (const auto& str : programs) {
      ASTInput input(str.c_str(), str.size());
      ASTNode result;
      CHECK_NOTHROW(result = ASTNode::parseProgram(input));
      CHECK(result.kind() == NodeKind::program);
    }
    /*
    CHECK_NOTHROW(ASTNode::parseProgram(input("128.45")));
    CHECK_NOTHROW(ASTNode::parseProgram(input("'foo bar'")));
    CHECK_NOTHROW(ASTNode::parseProgram(input("identifier")));
    */
  }
}
