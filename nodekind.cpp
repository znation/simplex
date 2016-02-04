#include "nodekind.h"

const char* simplex::NodeKindName(simplex::NodeKind kind) {
  switch (kind) {
    case NodeKind::expression: return "expression";
    case NodeKind::floatingPoint: return "float";
    case NodeKind::identifier: return "identifier";
    case NodeKind::integer: return "integer";
    case NodeKind::invalid: return "invalid";
    case NodeKind::literal: return "literal";
    case NodeKind::number: return "number";
    case NodeKind::optionalParameterList: return "optional parameter list";
    case NodeKind::parameterList: return "parameter list";
    case NodeKind::program: return "program";
    case NodeKind::string: return "string";
    case NodeKind::whitespace: return "whitespace";
  }

}
