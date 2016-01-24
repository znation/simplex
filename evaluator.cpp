#include "evaluator.h"
#include "parser.h"
#include "stdlib.h"

#include <cassert>
#include <sstream>

using namespace simplex;

Evaluator::Evaluator() {
  stdlib::addSymbols(m_symbols);
}

std::vector<Structure> Evaluator::evalParameters(const ASTNode& node) {
  if (node.kind() == NodeKind::optionalParameterList) {
    auto children = node.children();
    if (children.size() == 0) {
      return std::vector<Structure>();
    }
    assert(children.size() == 1);
    return evalParameters(children[0]);
  }

  assert(node.kind() == NodeKind::parameterList);
  std::vector<Structure> ret;
  for (const auto& child : node.children()) {
    ret.push_back(evalExpression(child));
  }
  return ret;
}

Structure Evaluator::evalLiteral(const ASTNode& node) {
  assert(node.kind() == NodeKind::literal);
  auto children = node.children();
  assert(children.size() == 1);
  const auto& child = children[0];
  switch (child.kind()) {
    case NodeKind::integer:
      return Structure(child.integer());
    case NodeKind::floatingPoint:
      return Structure(child.floatingPoint());
    default:
      assert(child.kind() == NodeKind::string);
      return Structure(child.string());
  }
}

Structure Evaluator::evalProgram(const ASTNode& node) {
  Structure ret;
  assert(node.kind() == NodeKind::program);
  for (const auto& exp : node.children()) {
    ret = eval(exp);
  }
  return ret;
}

Structure Evaluator::evalExpression(const ASTNode& node) {
  assert(node.kind() == NodeKind::expression);
  auto children = node.children();
  if (children.size() == 1) {
    // literal or identifier
    if (children[0].kind() == NodeKind::identifier) {
      return Structure(m_symbols.at(children[0].string()));
    } else {
      return evalLiteral(children[0]);
    }
  } else {
    assert(children.size() == 2);
    Structure fn = evalExpression(children[0]);
    std::vector<Structure> params = evalParameters(children[1]);
    return fn(params);
  }
}

Structure Evaluator::eval(const ASTNode& node) {
  switch (node.kind()) {
    case NodeKind::program:
      return evalProgram(node);
    case NodeKind::expression:
      return evalExpression(node);
    default:
      std::stringstream ss;
      ss << "not implemented: ";
      ss << NodeKindName(node.kind());
      throw ss.str();
  }
}

Structure Evaluator::eval(const std::string& input) {
  ASTNode ast = Parser::parse(input);
  return eval(ast);
}
