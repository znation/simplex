# simplex
A simple language &amp; interpreter. Might be a Lisp.

## Syntax

In BNF notation:

```
<program> ::= <expression> <program> | <expression>

<expression> ::= <optional-whitespace> "(" <optional-whitespace> <expression> <whitespace> <optional-parameter-list> <optional-whitespace> ")" <optional-whitespace> | <optional-whitespace> <literal> <optional-whitespace> | <optional-whitespace> <identifier> <optional-whitespace>

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

Identifiers can be any permutation of one or more non-whitespace characters, not starting with a number, and excluding `(`, `)`,  and `'`.

Strings can be any permutation of characters surrounded by `'`, excluding `'` unless escaped as `\'`.

See the [unit tests](test/evaluator.cpp) for some examples of simple programs/expressions.

## Semantics

### Built in identifiers (reserved words)

#### Values

##### `nil`

`nil` represents a lack of something; nothing. Usually the sentinel value at the end of a list.

#### Creation

##### `lambda arg1 arg2... expression`

`lambda` creates a function, taking named (identifier) arguments and an expression that can use those arguments.

##### `let name expression`

`let` creates a named identifier in the current scope, and assigns it the result of an expression.

##### `cons x y`, `car xs`, `cdr xs`, `list x y z etc.`

`cons` constructs a pair of (x,y).
`car` retrieves the first element of such a pair.
`cdr` retrieves the second element of such a pair.
`list` constructs a list of pairs like (x, (y, (z, etc.)))

See [wikipedia](https://en.wikipedia.org/wiki/Cons) for more details.

#### Computation

##### `+`, `-`, `/`, `*`, `=`

Mathematical operators and (type-safe!) equality operator.

#### Control Flow

##### `sequence expr1 expr2 expr3 etc.`

`sequence` runs multiple expressions in sequence. Returns the result of the last expression and discards the results of prior expressions.

##### `if condition expr1 expr2`

* If `condition` evaluates to `true`, evaluates `expr1` and returns the result.
* If `condition` evaluates to `false`, evaluates `expr2` and returns the result.
* If `condition` evaluates to a non-Boolean value, errors.
Notably (and distinctly from non-conditionals), only the returned expression is evaluated.

##### `cond cond1 expr1 cond2 expr2 etc.`

May be used with any number of parameters.
Each pair of two parameters (condition and expression), starting from `cond1 expr1`, gives the following behavior:
* If the condition evaluates to `true`, evaluates the expression and returns the result.
* If the condition evaluates to `false`, does not evaluate the expression, and moves on to the next pair (if any).
* If the condition evaluates to a non-Boolean value, errors.
If no conditional expression evaluates to true, errors.

#### Future (Roadmap)

##### `import filename`

`import` imports the specified file as a module, into the current scope.

##### `parallel expr1 expr2 expr3 etc.`

`parallel` runs multiple expressions in parallel (returning when the last-running expression returns).
