#include "evaluator.h"
#include "repl.h"

#include <unistd.h>

#include <iostream>
#include <string>

using namespace simplex;

int main(int argc, char* argv[]) {
  Evaluator e;
  if (isatty(fileno(stdin))) {
    Repl r;
    r.run();
  } else {
    std::cout << e.eval(std::cin);
  }
}
