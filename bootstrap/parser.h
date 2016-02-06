#include "astnode.h"

#include <iostream>
#include <string>

namespace simplex {
  namespace Parser {
    ASTNode parse(const std::string& input);
    ASTNode parse(std::istream& input);
  };
};
