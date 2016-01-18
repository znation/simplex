#include "parser.h"

using namespace simplex;

ASTNode Parser::parse(const std::string& input) {
  return ASTNode::parseProgram(input.c_str(), input.size());
}

ASTNode Parser::parse(std::istream& input) {
  std::string buffer;
  for (std::string line; std::getline(input, line);) {
    buffer.append(line);
    buffer.push_back('\n');
  }

  return Parser::parse(buffer);
}
