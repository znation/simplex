#include <exception>
#include <string>

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
      void set_message(NodeKind kind, const char * expected, const char * actual);

    public:
      ParseError(NodeKind kind, const char * expected, const char * actual);
      ParseError(NodeKind kind, const char * expected, char actual);
  };

  class RuntimeError : public Error {
    public:
      RuntimeError(const char * message);
  };

  class TypeMismatchError : public Error {
    public:
      TypeMismatchError(StructureKind expected, StructureKind found);
  };
};
