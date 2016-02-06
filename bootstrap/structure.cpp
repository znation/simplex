#include "errors.h"
#include "structure.h"

#include <cassert>
#include <sstream>

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
Structure Structure::Nil() {
  Structure ret;
  ret.m_kind = StructureKind::nil;
  return ret;
}

const char * simplex::StructureKindName(StructureKind kind) {
  switch (kind) {
    case StructureKind::boolean:
      return "boolean";
    case StructureKind::cons:
      return "cons";
    case StructureKind::function:
      return "function";
    case StructureKind::integer:
      return "integer";
    case StructureKind::invalid:
      return "invalid";
    case StructureKind::floatingPoint:
      return "floatingPoint";
    case StructureKind::nil:
      return "nil";
    case StructureKind::string:
      return "string";
  }
}

std::ostream& simplex::operator<<(std::ostream& stream, const Structure& s) {
  stream << s.to_string();
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
      return (*m_car == *(s.m_car)) &&
             (*m_cdr == *(s.m_cdr));
    case StructureKind::floatingPoint:
      return m_float == s.m_float;
    case StructureKind::function:
      throw "doesn't make sense to compare two functions";
    case StructureKind::integer:
      return m_int == s.m_int;
    case StructureKind::invalid:
      throw "not implemented";
    case StructureKind::nil:
      return true; // TODO should this be false?
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

bool Structure::operator==(const char * str) const {
  return m_kind == StructureKind::string &&
         m_string == str;
}

// operators
Structure::operator bool() const {
  if (m_kind != StructureKind::boolean) {
    throw TypeMismatchError(*this, StructureKind::boolean);
  }
  assert(m_kind == StructureKind::boolean);
  return m_bool;
}

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

std::string Structure::to_string() const {
  std::stringstream ss;
  switch (m_kind) {
    case StructureKind::boolean:
      ss << m_bool;
      break;
    case StructureKind::cons:
      throw "not implemented";
    case StructureKind::function:
      throw "not implemented";
    case StructureKind::integer:
      ss << m_int;
      break;
    case StructureKind::invalid:
      throw "not implemented";
    case StructureKind::floatingPoint:
      ss << m_float;
      break;
    case StructureKind::nil:
      ss << "()";
      break;
    case StructureKind::string:
      ss << m_string;
      break;
  }
  return ss.str();
}

const Structure& Structure::car() const {
  return *m_car;
}

const Structure& Structure::cdr() const {
  return *m_cdr;
}

StructureKind Structure::kind() const {
  return m_kind;
}
