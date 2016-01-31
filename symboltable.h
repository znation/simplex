#ifndef _SYMBOLTABLE_H
#define _SYMBOLTABLE_H

#include "structure.h"

#include <unordered_map>

namespace simplex {
  class SymbolTable : public std::unordered_map<std::string, Structure> {
    public:
      SymbolTable();
      SymbolTable augment(const std::unordered_map<std::string, Structure>&) const;
  };
};

#endif
