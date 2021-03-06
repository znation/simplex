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
Structure::Structure(Dict dict) :
  m_kind(StructureKind::dict), m_dict(dict) { }

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
    case StructureKind::dict:
      return "dict";
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

void Structure::print(std::ostream& stream) const {
  // should only be used on strings
  // otherwise, call to_string() first to get a string representation.
  if (this->kind() != StructureKind::cons) {
    throw TypeMismatchError(ASTNode(), StructureKind::cons, this->kind());
  }
  const auto& car = this->car();
  const auto& cdr = this->cdr();
  if (car.kind() != StructureKind::nil) {
    if (car.kind() != StructureKind::byte) {
      throw TypeMismatchError(ASTNode(), StructureKind::byte, car.kind());
    }
    stream << static_cast<char>(car.byte());
    if (cdr.kind() != StructureKind::nil) {
      cdr.print(stream);
    }
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
    case StructureKind::dict:
      return m_dict == s.m_dict;
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
    throw TypeMismatchError(ASTNode(), StructureKind::boolean, m_kind);
  }
  assert(m_kind == StructureKind::boolean);
  return m_bool;
}

Structure Structure::operator()(const ASTNode& node, std::vector<Structure> params) {
  assert(m_kind == StructureKind::function);
  return m_function(node, params);
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

static void cons_to_string(const Structure& s, std::stringstream& out) {
  const Structure& car = s.car();
  const Structure& cdr = s.cdr();
  if (car.kind() == StructureKind::nil) {
    return;
  }
  out << car;
  if (cdr.kind() == StructureKind::nil) {
    return;
  }
  cons_to_string(cdr, out);
}

std::string Structure::string() const {
  assert(m_kind == StructureKind::cons);
  std::stringstream buffer;
  cons_to_string(*this, buffer);
  return buffer.str();
}

const Structure::Dict& Structure::dict() const {
  assert(m_kind == StructureKind::dict);
  return m_dict;
}

std::string Structure::to_string() const {
  std::stringstream ss;
  switch (m_kind) {
    case StructureKind::boolean:
      if (m_bool) {
        ss << "true";
      } else {
        ss << "false";
      }
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
    case StructureKind::dict:
      {
        ss << "(dict ";
        for (const auto& it : m_dict) {
          ss << std::endl;
          ss << "    '";
          ss << it.first;
          ss << "' ";
          ss << it.second.to_string();
        }
        ss << ")";
      }
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
  if (len == 0) {
    this->m_car = std::make_shared<Structure>(Structure::Nil());
    this->m_cdr = std::make_shared<Structure>(Structure::Nil());
  } else {
    this->m_car = std::make_shared<Structure>(static_cast<uint8_t>(str[0]));
    if (len == 1) {
      this->m_cdr = std::make_shared<Structure>(Structure::Nil());
    } else {
      this->m_cdr = std::make_shared<Structure>(&str[1], len-1);
    }
  }
}
