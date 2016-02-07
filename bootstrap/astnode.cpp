#include "astnode.h"
#include "errors.h"

#include <cassert>
#include <cctype>
#include <sstream>

using namespace simplex;

static bool isWhitespace(char c) {
  return strchr(" \t\n\r", c) != nullptr;
}

static void expect(NodeKind kind, ASTInput& input, const std::string& token) {
  size_t tokenSize = token.size();
  if (tokenSize > input.size()) {
    const auto remaining = input.remaining();
    throw ParseError(kind, token.c_str(), remaining.c_str());
  }
  const std::string shouldBeToken(input.get(), tokenSize);
  if (shouldBeToken != token) {
    throw ParseError(kind, token.c_str(), shouldBeToken.c_str());
  }
  input.advance(tokenSize);
}

ASTNode::ASTNode(NodeKind kind) : m_kind(kind) { }

void ASTNode::toString(std::stringstream& ss) const {
  static int indentLevel = 0;
  assert(indentLevel >= 0);
  for (size_t i=0; i<indentLevel; i++) {
    ss << " ";
  }
  ss << NodeKindName(m_kind);
  ss << std::endl;
  indentLevel++;
  for (const auto& child : m_children) {
    child.toString(ss);
  }
  indentLevel--;
}

std::string ASTNode::toString() const {
  std::stringstream ss;
  this->toString(ss);
  return ss.str();
}

std::ostream& simplex::operator<<(std::ostream& stream, const ASTNode& node) {
  std::string result = node.toString();
  stream << result;
  return stream;
}

ASTNode ASTNode::parseProgram(const char *inputStream, size_t len) {
  ASTInput input(inputStream, len);
  return parseProgram(input);
}

ASTNode ASTNode::parseProgram(ASTInput& input) {
  auto ret = ASTNode(NodeKind::program);
  ret.m_children.push_back(parseExpression(input));
  if (input.size() > 0) {
    ret.m_children.push_back(parseProgram(input));
  }
  return ret;
}

ASTNode ASTNode::parseExpression(ASTInput& input) {
  NodeKind kind = NodeKind::expression;
  ASTNode ret(kind);
  parseOptionalWhitespace(input);
  if (input.size() == 0) {
    throw ParseError(kind, "(", "EOF");
  }
  char next = input.peek();
  if (next == '(') {
    expect(kind, input, "(");
    ret.m_children.push_back(parseExpression(input));
    ret.m_children.push_back(parseOptionalParameterList(input));
    parseOptionalWhitespace(input);
    expect(kind, input, ")");
  } else if (next == '\'' || std::isdigit(next)) {
    ret = parseLiteral(input);
  } else {
    ret = parseIdentifier(input);
  }
  parseOptionalWhitespace(input);
  return ret;
}

ASTNode ASTNode::parseLiteral(ASTInput& input) {
  NodeKind kind = NodeKind::literal;
  ASTNode ret(kind);
  if (input.size() == 0) {
    throw ParseError(kind, "any valid literal", "EOF");
  }
  if (input.peek() == '\'') {
    // string
    ret.m_children.push_back(parseString(input));
  } else {
    ret.m_children.push_back(parseNumber(input));
  }
  return ret;
}

ASTNode ASTNode::parseNumber(ASTInput& input) {
  NodeKind kind = NodeKind::number;
  
  // read digits until whitespace or decimal
  std::stringstream ss;
  bool isFloat = false;
  size_t inputLen = input.size();
  for (size_t i=0; i<inputLen; i++) {
    char next = input.peek();
    if (i > 0 && next == '.') {
      ss << next;
      isFloat = true;
    } else {
      if (isWhitespace(next) || next == ')') {
        // number is done
        break;
      }
      if (!std::isdigit(next)) {
        throw ParseError(kind, "digits 0 through 9", next);
      }
      ss << next;
    }
    input.next();
  }

  // broke out early or, hit EOF?
  // maybe we have a valid number at this point
  auto result = ss.str();
  if (isFloat) {
    kind = NodeKind::floatingPoint;
    ASTNode ret(kind);
    ret.m_float = std::atof(result.c_str());
    return ret;
  } else {
    kind = NodeKind::integer;
    ASTNode ret(kind);
    ret.m_int = std::atol(result.c_str());
    return ret;
  }
}

ASTNode ASTNode::parseString(ASTInput& input) {
  NodeKind kind = NodeKind::string;
  ASTNode ret(kind);
  std::stringstream ss;
  bool foundEndOfString = false;
  expect(kind, input, "'");
  while (input.size() != 0) {
    char next = input.peek();
    if (next == '\'') {
      foundEndOfString = true;
      break;
    }
    if (next == '\\') {
      // escape char
      if (input.size() < 2) {
        throw ParseError(kind, "any character followed by escape sequence (\\)", "EOF");
      }
      ss << input.next();
      next = input.peek();
    }

    ss << next;
    input.next();
  }
  if (!foundEndOfString) {
    throw ParseError(kind, "end of string marker (')", "EOF");
  }

  expect(kind, input, "'");
  ret.m_string = ss.str();
  return ret;
}

ASTNode ASTNode::parseIdentifier(ASTInput& input) {
  NodeKind kind = NodeKind::identifier;
  ASTNode ret(kind);
  if (input.size() == 0) {
    throw ParseError(kind, "any valid identifier", "EOF");
  }
  char next = input.peek();
  if (next == '\'') {
    throw ParseError(kind, "non-whitespace character other than '\\'', '(' and ')'", next);
  }
  std::stringstream ss;
  while (input.size() != 0) {
    next = input.peek();
    if (isWhitespace(next) ||
        next == ')') {
      break;
    }
    if (next == '(') {
      throw ParseError(kind, "non-whitespace character other than '('", next);
    }
    ss << next;
    input.next();
  }
  ret.m_string = ss.str();
  assert(ret.m_string.size() != 0);
  return ret;
}

void ASTNode::parseOptionalWhitespace(ASTInput& input) {
  if (input.size() == 0) {
    return;
  }
  if (!isWhitespace(input.peek())) {
    return;
  }
  parseWhitespace(input);
}

void ASTNode::parseWhitespace(ASTInput& input) {
  bool foundWhitespace = false;
  while (input.size() != 0) {
    char next = input.peek();
    if (isWhitespace(next)) {
      foundWhitespace = true;
    } else if (!foundWhitespace) {
      throw ParseError(NodeKind::whitespace, "Any of: ' ', \\r, \\n, \\t", next);
    } else {
      break;
    }
    input.next();
  }
}

ASTNode ASTNode::parseOptionalParameterList(ASTInput& input) {
  NodeKind kind = NodeKind::optionalParameterList;
  ASTNode ret(kind);
  if (input.peek() != ')') {
    ASTNode parameterList(NodeKind::parameterList);
    parameterList.parseParameterList(input);
    ret.m_children.push_back(parameterList);
  }
  return ret;
}

void ASTNode::parseParameterList(ASTInput& input) {
  m_children.push_back(parseExpression(input));
  parseOptionalWhitespace(input);
  if (input.size() == 0) {
    throw ParseError(NodeKind::parameterList, "end of expression ')'", "EOF");
  }
  if (input.peek() == ')') {
    // hit end of parameter list
    return;
  }
  // more parameters left to parse
  parseParameterList(input);
}

bool ASTNode::operator==(const ASTNode& other) const {
  if (m_kind != other.m_kind) {
    return false;
  }
  switch (m_kind) {
    case NodeKind::integer:
      return m_int == other.m_int;
    case NodeKind::floatingPoint:
      return m_float == other.m_float;
    case NodeKind::identifier:
    case NodeKind::string:
      return m_string == other.m_string;
    default:
      if (m_children != other.m_children) {
        return false;
      }
  }
  return true;
}

NodeKind ASTNode::kind() const {
  return m_kind;
}

const std::vector<ASTNode> ASTNode::children() const {
  return m_children;
}

double ASTNode::floatingPoint() const {
  assert(m_kind == NodeKind::floatingPoint);
  return m_float;
}

int64_t ASTNode::integer() const {
  assert(m_kind == NodeKind::integer);
  return m_int;
}

const std::string& ASTNode::string() const {
  assert(m_kind == NodeKind::string ||
         m_kind == NodeKind::identifier);
  return m_string;
}

ASTNode::ASTNode() : m_kind(NodeKind::invalid) { }
