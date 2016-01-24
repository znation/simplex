#include "structure.h"

#include <cassert>

using namespace simplex;

Structure::Structure() : m_kind(StructureKind::invalid) { }
Structure::Structure(int64_t i) : m_kind(StructureKind::integer), m_int(i) { }
Structure::Structure(double d) : m_kind(StructureKind::floatingPoint), m_float(d) { }
Structure::Structure(const std::string& s) :
  m_kind(StructureKind::string), m_string(s) { }
Structure::Structure(Function fn) :
  m_kind(StructureKind::function), m_function(fn) { }
Structure::Structure(std::shared_ptr<Structure>&& car, std::shared_ptr<Structure>&& cdr) :
  m_kind(StructureKind::cons), m_car(std::move(car)), m_cdr(std::move(cdr)) { }

bool Structure::operator==(int64_t i) {
  if (m_kind != StructureKind::integer) {
    return false;
  }
  return m_int == i;
}

bool Structure::operator==(double d) {
  if (m_kind != StructureKind::floatingPoint) {
    return false;
  }
  return m_float == d;
}

bool Structure::operator==(int i) {
  return (*this) == static_cast<int64_t>(i);
}

// operators
Structure Structure::operator()(std::vector<Structure> params) {
  assert(m_kind == StructureKind::function);
  return m_function(params);
}
std::ostream& Structure::operator<<(std::ostream& os) const {
  switch (m_kind) {
    case StructureKind::cons:
      throw "not implemented";
    case StructureKind::floatingPoint:
      os << m_float;
      break;
    case StructureKind::function:
      throw "not implemented";
    case StructureKind::integer:
      os << m_int;
      break;
    case StructureKind::invalid:
      throw "not implemented";
    case StructureKind::string:
      os << m_string;
      break;
  }
  return os;
}

double Structure::floatingPoint() const {
  assert(m_kind == StructureKind::floatingPoint);
  return m_float;
}

int64_t Structure::integer() const {
  assert(m_kind == StructureKind::integer);
  return m_int;
}

StructureKind Structure::kind() const {
  return m_kind;
}
