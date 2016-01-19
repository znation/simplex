#include "nodekind.h"

const char* simplex::NodeKindName(simplex::NodeKind kind) {
  switch (kind) {
    case NodeKind::program: return "program";
    case NodeKind::expression: return "expression";
    case NodeKind::optionalParameterList: return "optional parameter list";
    case NodeKind::parameterList: return "parameter list";
    case NodeKind::literal: return "literal";
    case NodeKind::number: return "number";
    case NodeKind::integer: return "integer";
    case NodeKind::floatingPoint: return "float";
    case NodeKind::identifier: return "identifier";
    case NodeKind::string: return "string";
    case NodeKind::whitespace: return "whitespace";
  }

}
