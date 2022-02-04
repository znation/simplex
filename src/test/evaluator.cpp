#include <sstream>

#define UNIT_TESTING
#include "../evaluator.h"
#include "../errors.h"

#include "catch.h"
#include "macros.h"

using namespace simplex;

TEST_CASE("math [evaluator]") {
  Evaluator e;
  CHECK_MATH_1(+, 4, 4);
  CHECK_MATH_2(+, 4, 3, 7);
  CHECK_MATH_2(+, 34.2, 5, 39.2);
  CHECK_MATH_1(-, 2, (- 2));
  CHECK_MATH_1(-, 3.45, (- 3.45));
  CHECK_MATH_2(-, 10, 2, 8);
  CHECK_MATH_2(*, 8, 2, 16);
  CHECK_MATH_2(*, (- 2), 24, (- 48));
  CHECK_MATH_2(*, (- 1.5), 2, (- 3.0));
  CHECK_MATH_2(/, 8, 2, 4);
  CHECK_MATH_2(/, (- 2), 24, 0);
  CHECK_MATH_2(/, (- 58), 3, (- 19));
  CHECK_MATH_2(/, (- 1.5), 2, (- 0.75));
  CHECK_MATH_2(/, 0.5, 2.0, 0.25);
}

TEST_CASE("comparison [evaluator]") {
  Evaluator e;
  CHECK_MATH_2(=, 3, 3, true);
  CHECK_MATH_2(<, 2, 3, true);
  CHECK_MATH_2(<, 3, 3, false);
  CHECK_MATH_2(<, 4, 3, false);
  CHECK_MATH_2(>, 2, 3, false);
  CHECK_MATH_2(>, 3, 3, false);
  CHECK_MATH_2(>, 4, 3, true);
}

TEST_CASE("lambda [evaluator]") {
  Evaluator e;
  CHECK(e.eval("((lambda x y (+ x y)) 3 4)") == 7);
  CHECK(e.eval("((lambda x (+ x 5)) 2)") == 7);
}

TEST_CASE("let [evaluation]") {
  Evaluator e;
  CHECK(e.eval("(let add (lambda x y (+ x y)))") == true);
  CHECK(e.eval("(let five 5)") == true);
  CHECK(e.eval("(+ five (add 1 2))") == 8);
}

TEST_CASE("sequence [evaluation]") {
  Evaluator e;
  CHECK(e.eval("(sequence (+ 3 4) (- 5 2))") == 3);
  CHECK(e.eval("(sequence (+ 3 4) (= 5 2))") == false);
}

TEST_CASE("let with sequence [evaluation]") {
  Evaluator e;
  CHECK(e.eval("(sequence (let a 2) (let b 9) (+ a b))") == 11);
}

TEST_CASE("cons") {
  Evaluator e;
  CHECK(e.eval("(let a (cons 3 4))") == true);
  CHECK(e.eval("(car a)") == 3);
  CHECK(e.eval("(cdr a)") == 4);
  CHECK(e.eval("(= (list 1 2 3) (cons 1 (cons 2 (cons 3 nil))))") == true);
}

TEST_CASE("dict") {
  Evaluator e;
  CHECK(e.eval("(dict.get 'x' (dict 'x' 3 'y' 4))") == 3);
  CHECK(e.eval("(dict.get 'y' (dict 'x' 3 'y' 4))") == 4);
  CHECK(e.eval("(dict.set 'x' 4 (dict 'x' 3))") == Structure(Structure::Dict({
    {"x", Structure(static_cast<int64_t>(4))}
  })));
  CHECK(e.eval("(dict.set 'x' 4 (dict 'y' 3))") == Structure(Structure::Dict({
    {"x", Structure(static_cast<int64_t>(4))},
    {"y", Structure(static_cast<int64_t>(3))}
  })));
}

TEST_CASE("conditionals") {
  Evaluator e;
  CHECK(e.eval("(if true 'hello' 'world')") == Structure(std::string("hello")));
  CHECK(e.eval("(if false 'hello' 'world')") == Structure(std::string("world")));
  CHECK(e.eval("(cond false 'foo' true 'bar' false 'baz')") == Structure(std::string("bar")));
  CHECK(e.eval("(cond false 'foo' false 'bar' true 'baz')") == Structure(std::string("baz")));
  CHECK_THROWS_AS(e.eval("(cond false 'foo' false 'bar' false 'qux')"), RuntimeError);
  CHECK_THROWS_AS(e.eval("(cond false 'foo' 'bar' true 'baz' 'qux')"), TypeMismatchError);
}

TEST_CASE("conversion") {
  Evaluator e;
  CHECK(e.eval("(string 3)") == Structure(std::string("3")));
  CHECK(e.eval("(string true)") == Structure(std::string("true")));
  CHECK(e.eval("(string 'abc')") == Structure(std::string("abc")));
  CHECK(e.eval("(string (cons 1 2))") == Structure(std::string("(cons 1 2)")));
  CHECK(e.eval("(string 3.842)") == Structure(std::string("3.842")));
  CHECK(e.eval("(car '\n')") == Structure(static_cast<uint8_t>('\n')));
}

TEST_CASE("i/o") {
  std::stringstream input;
  std::stringstream output;
  Evaluator e(input, output);

  e.eval("(print (string (+ 3 4)) endl)");
  CHECK(output.str() == "7\n");

  e.eval("(print 'hello')");
  CHECK(output.str() == "7\nhello");

  input << "a\nb";
  CHECK(e.eval("(read)") == Structure(static_cast<uint8_t>('a')));
  CHECK(e.eval("(read)") == Structure(static_cast<uint8_t>('\n')));
  CHECK(e.eval("(read)") == Structure(static_cast<uint8_t>('b')));
  CHECK(e.eval("(read)") == Structure::Nil());
}
