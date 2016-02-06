#include "errors.h"
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
  size_t paramsSize = params.size();
  assert(paramsSize >= 2);
  const auto& reference = params[0];
  bool ret = true;
  for (size_t i=1; i<paramsSize; i++) {
    const auto& param = params[i];
    ret = ret && (reference == param);
  }
  return Structure(ret);
}

static Structure sequence(std::vector<Structure> params) {
  // rely on the interpreter itself being sequential (single threaded)
  // simply return the last accumulated result
  size_t paramsSize = params.size();
  assert(paramsSize != 0);
  return params[paramsSize-1];
}

static Structure cons(std::vector<Structure> params) {
  assert(params.size() == 2);
  return Structure(
      std::make_shared<Structure>(params[0]),
      std::make_shared<Structure>(params[1])
  );
}

static Structure car(std::vector<Structure> params) {
  assert(params.size() == 1);
  const auto& cons = params[0];
  assert(cons.kind() == StructureKind::cons);
  return cons.car();
}

static Structure cdr(std::vector<Structure> params) {
  assert(params.size() == 1);
  const auto& cons = params[0];
  assert(cons.kind() == StructureKind::cons);
  return cons.cdr();
}

static Structure list_impl(const std::vector<Structure>& params, size_t idx) {
  size_t size = params.size() - idx;
  assert(size >= 1);
  if (size == 1) {
    return Structure(
      std::make_shared<Structure>(params[idx]),
      std::make_shared<Structure>(Structure::Nil())
    );
  } else {
    return Structure(
      std::make_shared<Structure>(params[idx]),
      std::make_shared<Structure>(list_impl(params, idx+1))
    );
  }
}

static Structure list(std::vector<Structure> params) {
  return list_impl(params, 0);
}

static Structure print(std::vector<Structure> params) {
  for (const auto& param : params) {
    std::cout << param;
  }
  return Structure::Nil();
}

void stdlib::addSymbols(SymbolTable& symbols) {
  // math & comparison operators
  symbols["+"] = Structure(static_cast<Structure::Function>(plus));
  symbols["-"] = Structure(static_cast<Structure::Function>(minus));
  symbols["*"] = Structure(static_cast<Structure::Function>(times));
  symbols["/"] = Structure(static_cast<Structure::Function>(divide));
  symbols["="] = Structure(static_cast<Structure::Function>(equals));

  // control flow
  symbols["sequence"] = Structure(static_cast<Structure::Function>(sequence));

  // structural operators
  symbols["cons"] = Structure(static_cast<Structure::Function>(cons));
  symbols["car"] = Structure(static_cast<Structure::Function>(car));
  symbols["cdr"] = Structure(static_cast<Structure::Function>(cdr));
  symbols["list"] = Structure(static_cast<Structure::Function>(list));

  // values
  const static std::string endl("\n");
  symbols["endl"] = Structure(endl);
  symbols["nil"] = Structure::Nil();

  // i/o
  symbols["print"] = Structure(static_cast<Structure::Function>(print));
}
