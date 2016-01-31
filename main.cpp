#include "parser.h"

#include <iostream>
#include <string>

using namespace simplex;

int main(int argc, char* argv[]) {
  ASTNode root = Parser::parse(std::cin);
  std::cout << root;
  return 0;
}
