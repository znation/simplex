#include "errors.h"

#include <sstream>
#include <string>

using namespace simplex;

void ParseError::init(
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
  m_message = ss.str();
}

ParseError::ParseError(NodeKind kind, const std::string& expected, const std::string& actual) {
  this->init(kind, expected, actual);
}

ParseError::ParseError(NodeKind kind, const std::string& expected, char actual) {
  this->init(kind, expected, std::string(&actual, 1));
}

const char* ParseError::what() const noexcept {
  return m_message.c_str();
}
