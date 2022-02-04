#include "astnode.h"
#include "structure.h"
#include "symboltable.h"

namespace simplex {
  class Evaluator {
    private:
      SymbolTable m_symbols;

      Structure evalCondExpression(const ASTNode&);
      Structure evalExpression(const ASTNode&);
      Structure evalIdentifier(const ASTNode&);
      Structure evalIfExpression(const ASTNode&);
      Structure evalLambdaExpression(const ASTNode&);
      Structure evalLetExpression(const ASTNode&);
      Structure evalLiteral(const ASTNode&);
      std::vector<Structure> evalParameters(const ASTNode&);
      Structure evalProgram(const ASTNode&);
      Structure eval(const ASTNode&);

    public:
      Evaluator(std::istream& input = std::cin, std::ostream& output = std::cout);
      Evaluator(const SymbolTable&);
      Structure eval(const std::string&);
      Structure eval(std::istream& input);
  };
};
