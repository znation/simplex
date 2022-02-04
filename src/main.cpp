#include "errors.h"
#include "evaluator.h"
#include "repl.h"

#include <unistd.h>

#include <fstream>
#include <iostream>
#include <string>

using namespace simplex;

const char *DASHES =
"--------------------------------------------------------------------------------";

template<typename T>
int evaluate(T& src) {
  Evaluator e;
  try {
    e.eval(src);
    return 0;
  } catch (const Error& e) {
    std::cerr << DASHES << std::endl;
    std::cerr << "Unhandled exception!" << std::endl << std::endl;
    std::cerr << e.what() << std::endl;
    std::cerr << DASHES << std::endl;
    return 1;
  }
}

int main(int argc, char* argv[]) {
  if (argc > 1) {
    std::ifstream contents(argv[1]);
    return evaluate(contents);
  } else if (isatty(fileno(stdin))) {
    Repl r;
    r.run();
  } else {
    return evaluate(std::cin);
  }
}
