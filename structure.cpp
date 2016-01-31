#include "structure.h"

#include <cassert>

using namespace simplex;

Structure::Structure() : m_kind(StructureKind::invalid) { }
Structure::Structure(bool b) : m_kind(StructureKind::boolean), m_bool(b) { }
Structure::Structure(int64_t i) : m_kind(StructureKind::integer), m_int(i) { }
Structure::Structure(double d) : m_kind(StructureKind::floatingPoint), m_float(d) { }
Structure::Structure(const std::string& s) :
  m_kind(StructureKind::string), m_string(s) { }
Structure::Structure(Function fn) :
  m_kind(StructureKind::function), m_function(fn) { }
Structure::Structure(std::shared_ptr<Structure>&& car, std::shared_ptr<Structure>&& cdr) :
  m_kind(StructureKind::cons), m_car(std::move(car)), m_cdr(std::move(cdr)) { }

std::ostream& simplex::operator<<(std::ostream& stream, const Structure& s) {
  switch (s.kind()) {
    case StructureKind::boolean:
      stream << s.boolean();
      break;
    case StructureKind::cons:
      throw "not implemented";
    case StructureKind::floatingPoint:
      stream << s.floatingPoint();
      break;
    case StructureKind::function:
      throw "not implemented";
    case StructureKind::integer:
      stream << s.integer();
      break;
    case StructureKind::invalid:
      throw "not implemented";
    case StructureKind::string:
      stream << s.string();
      break;
  }
  return stream;
}

bool Structure::operator==(const Structure& s) const {
  if (m_kind != s.kind()) {
    return false;
  }
  switch (m_kind) {
    case StructureKind::boolean:
      return m_bool == s.boolean();
    case StructureKind::cons:
      throw "not implemented";
    case StructureKind::floatingPoint:
      return m_float == s.m_float;
    case StructureKind::function:
      throw "doesn't make sense to compare two functions";
    case StructureKind::integer:
      return m_int == s.m_int;
    case StructureKind::invalid:
      throw "not implemented";
    case StructureKind::string:
      return m_string == s.m_string;
  }
}

bool Structure::operator==(int64_t i) const {
  return m_kind == StructureKind::integer &&
         m_int == i;
}

bool Structure::operator==(double d) const {
  return m_kind == StructureKind::floatingPoint &&
         m_float == d;
}

bool Structure::operator==(int i) const {
  return (*this) == static_cast<int64_t>(i);
}

bool Structure::operator==(bool b) const {
  return m_kind == StructureKind::boolean &&
         m_bool == b;
}

// operators
Structure Structure::operator()(std::vector<Structure> params) {
  assert(m_kind == StructureKind::function);
  return m_function(params);
}

bool Structure::boolean() const {
  assert(m_kind == StructureKind::boolean);
  return m_bool;
}

double Structure::floatingPoint() const {
  assert(m_kind == StructureKind::floatingPoint);
  return m_float;
}

int64_t Structure::integer() const {
  assert(m_kind == StructureKind::integer);
  return m_int;
}

std::string Structure::string() const {
  assert(m_kind == StructureKind::string);
  return m_string;
}

StructureKind Structure::kind() const {
  return m_kind;
}
