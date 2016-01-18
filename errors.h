#include <exception>
#include <string>

#include "nodekind.h"

namespace simplex {
  class ParseError : public std::exception {
    private:
      std::string m_message;

    public:
      ParseError(NodeKind kind, const std::string& expected, const std::string& actual);
      virtual const char* what() const noexcept;
  };
};
