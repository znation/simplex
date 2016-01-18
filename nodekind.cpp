#include "nodekind.h"

const char* simplex::NodeKindName(simplex::NodeKind kind) {
  switch (kind) {
    case NodeKind::program: return "program";
    case NodeKind::expression: return "expression";
    case NodeKind::optionalParameterList: return "optional parameter list";
    case NodeKind::parameterList: return "optional parameter list";
    case NodeKind::literal: return "parameter list";
    case NodeKind::number: return "literal";
    case NodeKind::integer: return "number";
    case NodeKind::floatingPoint: return "integer";
    case NodeKind::identifier: return "float";
    case NodeKind::string: return "identifier";
    case NodeKind::whitespace: return "whitespace";
  }

}
