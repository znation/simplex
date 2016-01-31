#include "astnode.h"
#include "structure.h"
#include "symboltable.h"

namespace simplex {
  class Evaluator {
    private:
      SymbolTable m_symbols;
      Structure evalExpression(const ASTNode&);
      Structure evalLambdaExpression(const ASTNode&);
      Structure evalLiteral(const ASTNode&);
      std::vector<Structure> evalParameters(const ASTNode&);
      Structure evalProgram(const ASTNode&);
      Structure eval(const ASTNode&);

    public:
      Evaluator();
      Evaluator(const SymbolTable&);
      Structure eval(const std::string&);
  };
};
