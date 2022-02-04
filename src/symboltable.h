#ifndef _SYMBOLTABLE_H
#define _SYMBOLTABLE_H

#include "structure.h"

#include <unordered_map>

namespace simplex {
  class SymbolTable : public std::unordered_map<std::string, Structure> {
    public:
      std::istream& input;
      std::ostream& output;
      SymbolTable(std::istream& input, std::ostream& output);
      SymbolTable augment(const std::unordered_map<std::string, Structure>&) const;
  };
};

#endif
