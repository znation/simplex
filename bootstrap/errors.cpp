#include "errors.h"

#include <sstream>
#include <string>

using namespace simplex;

void ParseError::set_message(
    NodeKind kind,
    const char * expected,
    const char * actual) {
  std::snprintf(
      m_message,
      m_message_size,
      "parse error while attempting to parse %s: expected %s, found %s",
      NodeKindName(kind),
      expected,
      actual);
}

const char * Error::what() const noexcept {
  return m_message; 
}

ParseError::ParseError(NodeKind kind, const char * expected, const char * actual) {
  this->set_message(kind, expected, actual);
}

ParseError::ParseError(NodeKind kind, const char * expected, char actual) {
  char actual_str[2];
  actual_str[0] = actual;
  actual_str[1] = 0;
  this->set_message(kind, expected, actual_str);
}

RuntimeError::RuntimeError(const char * str) {
  std::strncpy(m_message, str, m_message_size);
}

TypeMismatchError::TypeMismatchError(StructureKind expected, StructureKind found) {
  std::snprintf(m_message, m_message_size, "type mismatch error: expected %s, found %s", StructureKindName(expected), StructureKindName(found));
}
