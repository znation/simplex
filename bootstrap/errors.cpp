#include "errors.h"

#include <sstream>
#include <string>

using namespace simplex;

static std::string parserErrorMessage(
    NodeKind kind,
    const std::string& expected,
    const std::string& actual) {
  std::stringstream ss;
  ss << "Parse error while attempting to parse ";
  ss << NodeKindName(kind);
  ss << ". Expected:" << std::endl << '\t';
  ss << expected << std::endl;
  ss << "Found:" << std::endl << '\t';
  ss << actual << std::endl;
  return ss.str();
}

static std::string typeMismatchErrorMessage(
    const Structure& s,
    StructureKind expected) {
  std::stringstream ss;
  ss << "Type mismatch with value:" << std::endl;
  ss << s;
  ss << "Expected " << StructureKindName(expected);
  ss << ", found " << StructureKindName(s.kind()) << std::endl;
  return ss.str();
}

Error::Error(const std::string& str) : m_message(str) {}

const char* Error::what() const noexcept {
  return m_message.c_str();
}

ParseError::ParseError(NodeKind kind, const std::string& expected, const std::string& actual)
  : Error(parserErrorMessage(kind, expected, actual)) {}

ParseError::ParseError(NodeKind kind, const std::string& expected, char actual)
  : Error(parserErrorMessage(kind, expected, std::string(&actual, 1))) {}

RuntimeError::RuntimeError(const std::string& str) : Error(str) {}

TypeMismatchError::TypeMismatchError(const Structure& s, StructureKind expected) : RuntimeError(typeMismatchErrorMessage(s, expected)) {}
