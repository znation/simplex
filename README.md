# simplex
A simple language &amp; interpreter. Might be a Lisp.

## Syntax

In BNF notation:

```
<program> ::= <expression> <whitespace> <program> | <expression>

<expression> ::= "(" <optional-whitespace> <identifier> <optional-parameter-list> <optional-whitespace> ")" | <literal>

<optional-parameter-list> ::= <parameter-list> | ""

<parameter-list> ::= <expression> <whitespace> <parameter-list> | <expression>

<literal> ::= <number> | <string>

<number> ::= <float> | <integer>

<integer> ::= <digit> <integer> | <digit>

<float> ::= <integer> "." <integer> | <integer> "."

<digit> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"

<optional-whitespace> ::= <whitespace> | ""

<whitespace> ::= " "  <whitespace>  |
                 "\t" <whitespace>  |
                 "\n" <whitespace>  |
                 "\r" <whitespace>  |
                 " "                |
                 "\t"               |
                 "\n"               |
                 "\r"
```

Identifiers can be any permutation of one or more non-whitespace characters, excluding `(`, `)`, `'`, and numbers.

Strings can be any permutation of characters surrounded by `'`, excluding `'` unless escaped as `\'`.

## Semantics

### Built in identifiers (reserved words)

#### Creation

##### `define arguments expression`

`define` creates a function, taking arguments and an expression that can use those arguments.

##### `arg index`

`arg` accesses arguments to the current expression, by index.

##### `let name expression`

`let` creates a named identifier in the current scope, and assigns it the result of an expression.

##### `cons x y`, `car xs`, `cdr xs`, `list x y z etc.`

`cons` constructs a pair of (x,y).
`car` retrieves the first element of such a pair.
`cdr` retrieves the second element of such a pair.
`list` constructs a list of pairs like (x, (y, (z, etc.)))

See [wikipedia](https://en.wikipedia.org/wiki/Cons) for more details.

#### Computation

##### `+`, `-`, `/`, `*`, '='

Mathematical operators and (type-safe!) equality operator.

#### Future (Roadmap)

##### `import (filename)`

`import` imports the specified file as a module, into the current scope.

##### `sequence (expressions)`

`sequence` runs multiple expressions in sequence. Returns a list of the results of the expressions.

##### `parallel (expressions)`

`parallel` runs multiple expressions in parallel (returning when the last-running expression returns).
