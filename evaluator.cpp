#include "evaluator.h"
#include "parser.h"
#include "stdlib.h"

#include <cassert>
#include <sstream>

using namespace simplex;

Evaluator::Evaluator() {
  stdlib::addSymbols(m_symbols);
}

Evaluator::Evaluator(const SymbolTable& symbols) : m_symbols(symbols) {}

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
    ret.push_back(eval(child));
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

static std::unordered_map<std::string, Structure> dictOfParams(const std::vector<ASTNode>& parameterList, std::vector<Structure> parameterValues) {
  std::unordered_map<std::string, Structure> ret;
  size_t nParams = parameterList.size() - 1;
  assert(nParams == parameterValues.size());
  for (size_t i=0; i<nParams; i++) {
    const auto& param = parameterList[i];
    assert(param.kind() == NodeKind::identifier);
    const auto& value = parameterValues[i];
    ret[param.string()] = value;
  }
  return ret;
}

Structure Evaluator::evalLambdaExpression(const ASTNode& node) {
  assert(node.kind() == NodeKind::expression);
  const auto& children = node.children();
  assert(children.size() == 2);
  assert(children[0].kind() == NodeKind::identifier &&
         children[0].string() == "lambda");
  assert(children[1].kind() == NodeKind::optionalParameterList);
  const auto& parameterList = children[1].children()[0].children();
  return Structure(static_cast<Structure::Function>([this, parameterList](std::vector<Structure> params) {
    SymbolTable symbols = m_symbols.augment(dictOfParams(parameterList, params));
    Evaluator e(symbols);
    const auto& body = parameterList[parameterList.size()-1];
    return e.eval(body);
  }));
}

Structure Evaluator::evalLetExpression(const ASTNode& node) {
  assert(node.kind() == NodeKind::expression);
  const auto children = node.children();
  assert(children.size() == 2);
  assert(children[0].kind() == NodeKind::identifier &&
         children[0].string() == "let");
  assert(children[1].kind() == NodeKind::optionalParameterList);
  const auto parameterList = children[1].children()[0];
  const auto id_with_value = parameterList.children();
  assert(id_with_value.size() == 2);
  const auto& id = id_with_value[0];
  assert(id.kind() == NodeKind::identifier);
  m_symbols[id.string()] = eval(id_with_value[1]);
  return Structure(true);
}

Structure Evaluator::evalExpression(const ASTNode& node) {
  assert(node.kind() == NodeKind::expression);
  const auto& children = node.children();
  assert(children.size() == 2);
  if (children[0].kind() == NodeKind::identifier) {
    if (children[0].string() == "lambda") {
      return this->evalLambdaExpression(node);
    } else if (children[0].string() == "let") {
      return this->evalLetExpression(node);
    }
  }

  Structure fn = eval(children[0]);
  std::vector<Structure> params = evalParameters(children[1]);
  return fn(params);
}

Structure Evaluator::eval(const ASTNode& node) {
  switch (node.kind()) {
    case NodeKind::program:
      return evalProgram(node);
    case NodeKind::expression:
      return evalExpression(node);
    case NodeKind::identifier:
      if (m_symbols.find(node.string()) == m_symbols.end()) {
        std::stringstream ss;
        ss << "undeclared identifier: ";
        ss << node.string();
        throw ss.str();
      }
      return Structure(m_symbols.at(node.string()));
    case NodeKind::literal:
      return evalLiteral(node);
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

Structure Evaluator::eval(std::istream& input) {
  std::string buffer;
  for (std::string line; std::getline(input, line);) {
    buffer.append(line);
    buffer.push_back('\n');
  }
  return eval(buffer);
}
