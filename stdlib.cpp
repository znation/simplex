#include "stdlib.h"

#include <cassert>

using namespace simplex;

static double extractFloat(Structure n) {
  if (n.kind() == StructureKind::integer) {
    return static_cast<double>(n.integer());
  }
  return n.floatingPoint();
}

static Structure unaryPlus(Structure n) {
  assert(n.kind() == StructureKind::integer ||
         n.kind() == StructureKind::floatingPoint);
  return n; // don't modify
}

static Structure unaryMinus(Structure n) {
  if (n.kind() == StructureKind::integer) {
    return Structure(-(n.integer()));
  } else {
    assert(n.kind() == StructureKind::floatingPoint);
    return Structure(-(n.floatingPoint()));
  }
}

static Structure plus(std::vector<Structure> params) {
  if (params.size() == 1) {
    return unaryPlus(params[0]);
  }
  assert(params.size() == 2);
  if (params[0].kind() == StructureKind::integer &&
      params[1].kind() == StructureKind::integer) {
    return Structure(params[0].integer() + params[1].integer());
  }
  return Structure(extractFloat(params[0]) + extractFloat(params[1]));
}

static Structure minus(std::vector<Structure> params) {
  if (params.size() == 1) {
    return unaryMinus(params[0]);
  }
  assert(params.size() == 2);
  if (params[0].kind() == StructureKind::integer &&
      params[1].kind() == StructureKind::integer) {
    return Structure(params[0].integer() - params[1].integer());
  }
  return Structure(extractFloat(params[0]) - extractFloat(params[1]));
}

static Structure times(std::vector<Structure> params) {
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
      ret *= extractFloat(param);
    }
    return Structure(ret);
  }
}

static Structure divide(std::vector<Structure> params) {
  assert(params.size() == 2);
  if (params[0].kind() == StructureKind::integer &&
      params[1].kind() == StructureKind::integer) {
    return Structure(params[0].integer() / params[1].integer());
  } else {
    return Structure(extractFloat(params[0]) / extractFloat(params[1]));
  }
}

static Structure equals(std::vector<Structure> params) {
  assert(params.size() >= 2);
  const auto& reference = params[0];
  bool ret = true;
  for (const auto& param : params) {
    ret = ret && (reference == param);
  }
  return Structure(ret);
}

void stdlib::addSymbols(SymbolTable& symbols) {
  symbols["+"] = Structure(static_cast<Structure::Function>(plus));
  symbols["-"] = Structure(static_cast<Structure::Function>(minus));
  symbols["*"] = Structure(static_cast<Structure::Function>(times));
  symbols["/"] = Structure(static_cast<Structure::Function>(divide));
  symbols["="] = Structure(static_cast<Structure::Function>(equals));
}
