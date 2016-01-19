#include <exception>
#include <string>

#include "nodekind.h"

namespace simplex {
  class ParseError : public std::exception {
    private:
      std::string m_message;
      void init(NodeKind kind, const std::string& expected, const std::string& actual);

    public:
      ParseError(NodeKind kind, const std::string& expected, const std::string& actual);
      ParseError(NodeKind kind, const std::string& expected, char actual);
      virtual const char* what() const noexcept;
  };
};
