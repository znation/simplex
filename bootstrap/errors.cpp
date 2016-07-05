#include "errors.h"

#include <sstream>
#include <string>

using namespace simplex;

void ParseError::set_message(
    NodeKind kind,
    const char * expected,
    const char * actual,
    size_t line,
    size_t col) {
  std::snprintf(
      m_message,
      m_message_size,
      "%u|%u: parse error while attempting to parse %s: expected %s, found %s",
      static_cast<unsigned int>(line), // TODO use the correct format above so as not to need a cast
      static_cast<unsigned int>(col),
      NodeKindName(kind),
      expected,
      actual);
}

const char * Error::what() const noexcept {
  return m_message; 
}

ParseError::ParseError(NodeKind kind, const char * expected, const char * actual, size_t line, size_t col) {
  this->set_message(kind, expected, actual, line, col);
}

ParseError::ParseError(NodeKind kind, const char * expected, char actual, size_t line, size_t col) {
  char actual_str[2];
  actual_str[0] = actual;
  actual_str[1] = 0;
  this->set_message(kind, expected, actual_str, line, col);
}

RuntimeError::RuntimeError(const ASTNode& node, const char * str) {
  std::snprintf(
    m_message,
    m_message_size,
    "%u|%u: ",
    static_cast<unsigned int>(node.line()), // TODO use the correct format above so as not to need a cast
    static_cast<unsigned int>(node.col())
  );
  const size_t line_col_size = std::strlen(m_message);
  std::strncpy(m_message + line_col_size, str, m_message_size - line_col_size);
}

TypeMismatchError::TypeMismatchError(const ASTNode& node, StructureKind expected, StructureKind found) {
  std::snprintf(
    m_message,
    m_message_size,
    "%u|%u: type mismatch error: expected %s, found %s",
    static_cast<unsigned int>(node.line()), // TODO use the correct format above so as not to need a cast
    static_cast<unsigned int>(node.col()),
    StructureKindName(expected),
    StructureKindName(found)
  );
}
