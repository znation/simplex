#include "errors.h"

#include <sstream>
#include <string>

using namespace simplex;

ParseError::ParseError(NodeKind kind, const std::string& expected, const std::string& actual) {
  std::stringstream ss;
  ss << "Parse error while attempting to parse ";
  ss << NodeKindName(kind);
  ss << ". Expected:" << std::endl << '\t';
  ss << expected << std::endl;
  ss << "Found:" << std::endl << '\t';
  ss << actual << std::endl;
  m_message = ss.str();
}

const char* ParseError::what() const noexcept {
  return m_message.c_str();
}
