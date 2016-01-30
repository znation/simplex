#include "stdlib.h"

#include <cassert>

using namespace simplex;

Structure unaryPlus(Structure n) {
  assert(n.kind() == StructureKind::integer ||
         n.kind() == StructureKind::floatingPoint);
  return n; // don't modify
}

Structure unaryMinus(Structure n) {
  if (n.kind() == StructureKind::integer) {
    return Structure(-(n.integer()));
  } else {
    assert(n.kind() == StructureKind::floatingPoint);
    return Structure(-(n.floatingPoint()));
  }
}

Structure plus(std::vector<Structure> params) {
  if (params.size() == 1) {
    return unaryPlus(params[0]);
  }
  assert(params.size() == 2);
  if (params[0].kind() == params[1].kind()) {
    if (params[0].kind() == StructureKind::floatingPoint) {
      return Structure(params[0].floatingPoint() +
                       params[1].floatingPoint());
    } else {
      assert(params[0].kind() == StructureKind::integer);
      return Structure(params[0].integer() +
                       params[1].integer());
    }
  } else {
    if (params[0].kind() == StructureKind::floatingPoint) {
      return Structure(params[0].floatingPoint() +
          static_cast<double>(params[1].integer()));
    } else {
      return Structure(
          static_cast<double>(params[0].integer()) +
          params[0].integer());
    }
  }
}

Structure minus(std::vector<Structure> params) {
  if (params.size() == 1) {
    return unaryMinus(params[0]);
  }
  assert(params.size() == 2);
  if (params[0].kind() == params[1].kind()) {
    if (params[0].kind() == StructureKind::floatingPoint) {
      return Structure(params[0].floatingPoint() -
                       params[1].floatingPoint());
    } else {
      assert(params[0].kind() == StructureKind::integer);
      return Structure(params[0].integer() -
                       params[1].integer());
    }
  } else {
    if (params[0].kind() == StructureKind::floatingPoint) {
      return Structure(params[0].floatingPoint() -
          static_cast<double>(params[1].integer()));
    } else {
      return Structure(
          static_cast<double>(params[0].integer()) -
          params[1].floatingPoint());
    }
  }
}

Structure times(std::vector<Structure> params) {
  assert(params.size() >= 1);
  bool allInteger = true;
  for (const Structure& param : params) {
    if (param.kind() != StructureKind::integer) {
      allInteger = false;
    }
  }
  if (allInteger) {
    int64_t ret = 1;
    for (const Structure& param : params) {
      ret *= param.integer();
    }
    return Structure(ret);
  } else {
    double ret = 1.0;
    for (const Structure& param : params) {
      if (param.kind() == StructureKind::integer) {
        ret *= param.integer();
      } else {
        ret *= param.floatingPoint();
      }
    }
    return Structure(ret);
  }
}

void stdlib::addSymbols(SymbolTable& symbols) {
  symbols["+"] = plus;
  symbols["-"] = minus;
  symbols["*"] = times;
}
