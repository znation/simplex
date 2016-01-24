#include "stdlib.h"

#include <cassert>

using namespace simplex;

Structure add(std::vector<Structure> params) {
  assert(params.size() == 2);
  bool isFloat = false;
  for (size_t i=0; i<2; i++) {
    if (params[i].kind() != StructureKind::integer) {
      assert(params[i].kind() == StructureKind::floatingPoint);
      isFloat = true;
    }
  }
  if (isFloat) {
    return Structure(params[0].floatingPoint() + params[1].floatingPoint());
  } else {
    return Structure(params[0].integer() + params[1].integer());
  }
}

void stdlib::addSymbols(SymbolTable& symbols) {
  symbols["+"] = add;
}
