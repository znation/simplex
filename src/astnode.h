#ifndef _ASTNODE_H
#define _ASTNODE_H

#include "astinput.h"
#include "nodekind.h"

#include <string>
#include <vector>

namespace simplex {
  class ASTNode {
    private:
      ASTNode(NodeKind kind, size_t line, size_t col);
      NodeKind m_kind;
      union {
        int64_t m_int;
        double m_float;
      };
      // TODO use C++11 union to incorporate these as well
      // see http://cpp11standard.blogspot.com/2012/11/c11-standard-explained-1-unrestricted.html
      std::string m_string;
      std::vector<ASTNode> m_children;
      size_t m_line;
      size_t m_col;

#ifdef UNIT_TESTING
    public:
#endif
      static ASTNode parseProgram(ASTInput& input);
      static ASTNode parseExpression(ASTInput& input);
      static ASTNode parseOptionalParameterList(ASTInput& input);
      void parseParameterList(ASTInput& input);
      static ASTNode parseLiteral(ASTInput& input);
      static ASTNode parseNumber(ASTInput& input);
      static void parseOptionalWhitespace(ASTInput& input);
      static void parseWhitespace(ASTInput& input);
      static ASTNode parseIdentifier(ASTInput& input);
      static ASTNode parseString(ASTInput& input);

    public:
      ASTNode(); // produces invalid node! should only use for testing
      std::string toString() const;
      NodeKind kind() const;
      const std::vector<ASTNode> children() const;
      int64_t integer() const;
      double floatingPoint() const;
      const std::string& string() const;
      void toString(std::stringstream& ss) const;
      static ASTNode parseProgram(const std::string&, const char *, size_t);
      bool operator==(const ASTNode& other) const;
      size_t line() const;
      size_t col() const;
  };

  std::ostream& operator<<(std::ostream&, const ASTNode&);
};

#endif
