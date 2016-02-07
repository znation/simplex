#include "errors.h"
#include "structure.h"

#include <cassert>
#include <sstream>

using namespace simplex;

Structure::Structure() : m_kind(StructureKind::invalid) { }
Structure::Structure(bool b) : m_kind(StructureKind::boolean), m_bool(b) { }
Structure::Structure(uint8_t b) : m_kind(StructureKind::byte), m_byte(b) { }
Structure::Structure(int64_t i) : m_kind(StructureKind::integer), m_int(i) { }
Structure::Structure(double d) : m_kind(StructureKind::floatingPoint), m_float(d) { }
Structure::Structure(const char * str, size_t len) : m_kind(StructureKind::cons) {
  this->cons_from_string(str, len);
}
Structure::Structure(const std::string& s) : m_kind(StructureKind::cons) {
  const char * str = s.c_str();
  size_t len = s.size();
  this->cons_from_string(str, len);
}
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
    case StructureKind::byte:
      return "byte";
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
    case StructureKind::byte:
      return m_byte == s.byte();
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
      // not implemented
      assert(false);
      break;
    case StructureKind::nil:
      return true; // TODO should this be false?
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
  if (m_kind != StructureKind::cons) {
    return false;
  }
  size_t len = std::strlen(str);
  if (len == 0) {
    return m_car->m_kind == StructureKind::nil;
  }
  if (m_car->m_kind != StructureKind::byte) {
    return false;
  }
  if (m_car->m_byte != str[0]) {
    return false;
  }
  return *m_cdr == &str[1];
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

uint8_t Structure::byte() const {
  assert(m_kind == StructureKind::byte);
  return m_byte;
}

double Structure::floatingPoint() const {
  assert(m_kind == StructureKind::floatingPoint);
  return m_float;
}

int64_t Structure::integer() const {
  assert(m_kind == StructureKind::integer);
  return m_int;
}

std::string Structure::to_string() const {
  std::stringstream ss;
  switch (m_kind) {
    case StructureKind::boolean:
      ss << m_bool;
      break;
    case StructureKind::byte:
      ss << m_byte;
      break;
    case StructureKind::cons:
      ss << "(cons ";
      ss << *m_car;
      ss << " ";
      ss << *m_cdr;
      ss << ")";
      break;
    case StructureKind::function:
      // not implemented
      assert(false);
      break;
    case StructureKind::integer:
      ss << m_int;
      break;
    case StructureKind::invalid:
      // not implemented
      assert(false);
      break;
    case StructureKind::floatingPoint:
      ss << m_float;
      break;
    case StructureKind::nil:
      ss << "()";
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

void Structure::cons_from_string(const char * str, size_t len) {
  assert(len != 0);
  this->m_car = std::make_shared<Structure>(static_cast<uint8_t>(str[0]));
  if (len == 1) {
    this->m_cdr = std::make_shared<Structure>(Structure::Nil());
  } else {
    this->m_cdr = std::make_shared<Structure>(&str[1], len-1);
  }
}
