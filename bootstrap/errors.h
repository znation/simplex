#include <exception>
#include <string>

#include "astnode.h"
#include "nodekind.h"
#include "structure.h"

namespace simplex {
  class Error : public std::exception {
    protected:
      const static size_t m_message_size = 1024;
      char m_message[m_message_size] = "";
    public:
      virtual const char * what() const noexcept;
  };

  class ParseError : public Error {
    private:
      void set_message(NodeKind kind, const char * expected, const char * actual, size_t line, size_t col);

    public:
      ParseError(NodeKind kind, const char * expected, const char * actual, size_t line, size_t col);
      ParseError(NodeKind kind, const char * expected, char actual, size_t line, size_t col);
  };

  class RuntimeError : public Error {
    public:
      RuntimeError(const ASTNode& node, const char * message);
  };

  class TypeMismatchError : public Error {
    public:
      TypeMismatchError(const ASTNode& node, StructureKind expected, StructureKind found);
  };
};
