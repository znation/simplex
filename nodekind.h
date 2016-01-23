#ifndef _nodekind_h
#define _nodekind_h

namespace simplex {
  enum class NodeKind {
    program,
    expression,
    optionalParameterList,
    parameterList,
    literal,
    number,
    integer,
    floatingPoint,
    identifier,
    string,
    whitespace,
    invalid
  };

  const char* NodeKindName(NodeKind kind);
};

#endif
