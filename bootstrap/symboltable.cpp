#include "symboltable.h"

using namespace simplex;

SymbolTable::SymbolTable(std::istream& input, std::ostream& output)
  : input(input), output(output) {}

SymbolTable SymbolTable::augment(const std::unordered_map<std::string, Structure>& symbols) const {
  SymbolTable ret = *this; // shallow copy
  for (const auto& kv : symbols) {
    ret[kv.first] = kv.second;
  }
  return ret;
}
