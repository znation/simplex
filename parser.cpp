#include "parser.h"

#ifdef DEBUG
#include <iostream>
#endif

using namespace simplex;


ASTNode Parser::parse(const std::string& input) {
  auto ret = ASTNode::parseProgram(input.c_str(), input.size());
#ifdef DEBUG
  std::cout << ret << std::endl;
#endif
  return ret;
}

ASTNode Parser::parse(std::istream& input) {
  std::string buffer;
  for (std::string line; std::getline(input, line);) {
    buffer.append(line);
    buffer.push_back('\n');
  }

  return Parser::parse(buffer);
}
