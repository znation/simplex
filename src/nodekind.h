#ifndef _nodekind_h
#define _nodekind_h

namespace simplex {
  enum class NodeKind {
    expression,
    floatingPoint,
    identifier,
    integer,
    invalid,
    literal,
    number,
    optionalParameterList,
    parameterList,
    program,
    string,
    whitespace,
  };

  const char* NodeKindName(NodeKind kind);
};

#endif
