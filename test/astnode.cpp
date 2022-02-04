#include "catch.h"

#define UNIT_TESTING
#include "../src/astnode.h"

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

  std::vector<std::string> strings = {
      "'foo bar'",
      "'&\"\\'+~💩$'"
  };

  std::vector<std::string> integers = {
      "0",
      "1",
      "928453821"
  };

  std::vector<std::string> floats = {
      "0.23592",
      "29384."
  };

  std::vector<std::string> literals;
  append(literals, strings);
  append(literals, integers);
  append(literals, floats);

  std::vector<std::string> expressions = {
    "( + 3 4)",
    " (  +  3  4  ) ",
    "(- 1.5)",
    "(* (- 1.5) 2)",
  };
  append(expressions, whitespaceVariations(identifiers));
  append(expressions, whitespaceVariations(literals));

  std::vector<std::string> programs = expressions;
  for (size_t i=0; i<expressions.size() - 1; i++) {
    // test multiple expressions with or without whitespace between
    programs.push_back(expressions[0] + expressions[1]);
  }

  SECTION("parseProgram") {
    for (const auto& str : programs) {
      ASTInput input("TODO-filename", str.c_str(), str.size());
      ASTNode result;
      CHECK_NOTHROW(result = ASTNode::parseProgram(input));
      CHECK(result.kind() == NodeKind::program);
    }
  }

  SECTION("parseExpression") {
    for (const auto& str : expressions) {
      ASTInput input("TODO-filename", str.c_str(), str.size());
      ASTNode result;
      CHECK_NOTHROW(result = ASTNode::parseExpression(input));
      // resulting NodeKind could be any of:
      // * expression
      // * literal
      // * identifier
      // due to simplified parse tree structure (useless expressions are skipped).
    }
  }

  SECTION("parseIdentifier") {
    for (const auto& str : identifiers) {
      ASTInput input("TODO-filename", str.c_str(), str.size());
      ASTNode result;
      CHECK_NOTHROW(result = ASTNode::parseIdentifier(input));
      CHECK(result.kind() == NodeKind::identifier);
    }
  }

  SECTION("parseLiteral") {
    for (const auto& str : literals) {
      ASTInput input("TODO-filename", str.c_str(), str.size());
      ASTNode result;
      CHECK_NOTHROW(result = ASTNode::parseLiteral(input));
      CHECK(result.kind() == NodeKind::literal);
    }
  }

  SECTION("parseString") {
    for (const auto& str : strings) {
      ASTInput input("TODO-filename", str.c_str(), str.size());
      ASTNode result;
      CHECK_NOTHROW(result = ASTNode::parseString(input));
      CHECK(result.kind() == NodeKind::string);
    }
  }

  SECTION("parseNumber") {
    for (const auto& str : floats) {
      ASTInput input("TODO-filename", str.c_str(), str.size());
      ASTNode result;
      CHECK_NOTHROW(result = ASTNode::parseNumber(input));
      CHECK(result.kind() == NodeKind::floatingPoint);
    }
    for (const auto& str : integers) {
      ASTInput input("TODO-filename", str.c_str(), str.size());
      ASTNode result;
      CHECK_NOTHROW(result = ASTNode::parseNumber(input));
      CHECK(result.kind() == NodeKind::integer);
    }
  }
}
