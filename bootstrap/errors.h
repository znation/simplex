#include <exception>
#include <string>

#include "nodekind.h"
#include "structure.h"

namespace simplex {
  class Error : public std::exception {
    private:
      std::string m_message;
    public:
      Error(const std::string& message);
      virtual const char* what() const noexcept;
  };
  class ParseError : public Error {
    public:
      ParseError(NodeKind kind, const std::string& expected, const std::string& actual);
      ParseError(NodeKind kind, const std::string& expected, char actual);
  };
  class RuntimeError : public Error {
    public:
      RuntimeError(const std::string& message);
  };
  class TypeMismatchError : public RuntimeError {
    public:
      TypeMismatchError(const Structure& s, StructureKind expected);
  };
};
