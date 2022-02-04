.PHONY: \
	debug \
	release \
	test \


CXXFLAGS=\
		--std=c++14 \
		-MMD \
		-MP \
		-Wall \
		-Werror \


SRCS=$(wildcard bootstrap/*.cpp) \
		 $(wildcard bootstrap/test/*.cpp) \


OBJECTS=$(patsubst %.cpp,%.o,$(SRCS))
DEPENDS=${OBJECTS:.o=.d}

default: debug

debug: CXXFLAGS += -DDEBUG -g -O0
debug: CCFLAGS += -DDEBUG -g -O0
debug: bootstrap/simplex

release: CXXFLAGS += -O3
release: CCFLAGS += -O3
release: bootstrap/simplex

clean:
	rm -f ${OBJECTS} ${DEPENDS}

bootstrap/simplex: bootstrap/simplex.cpp \
	bootstrap/astinput.o \
	bootstrap/astnode.o \
	bootstrap/errors.o \
	bootstrap/evaluator.o \
	bootstrap/main.o \
	bootstrap/nodekind.o \
	bootstrap/parser.o \
	bootstrap/repl.o \
	bootstrap/stdlib.o \
	bootstrap/structure.o \
	bootstrap/symboltable.o \


test: CXXFLAGS += -DDEBUG -g -O0
test: CCFLAGS += -DDEBUG -g -O0
test: bootstrap/test/test
	./bootstrap/test/test -d yes

bootstrap/test/catch.h:
	curl -L -o $@ https://github.com/catchorg/Catch2/releases/download/v2.13.8/catch.hpp

# extra dependencies
bootstrap/evaluator.cpp: bootstrap/simplex_stdlib.h
bootstrap/test/astnode.cpp: bootstrap/test/catch.h
bootstrap/test/evaluator.cpp: bootstrap/test/catch.h
bootstrap/test/parser.cpp: bootstrap/test/catch.h
bootstrap/test/runner.cpp: bootstrap/test/catch.h
bootstrap/test/stdlib.cpp: bootstrap/test/catch.h

bootstrap/test/test: bootstrap/test/test.cpp \
	bootstrap/test/astnode.o \
	bootstrap/test/evaluator.o \
	bootstrap/test/parser.o \
	bootstrap/test/runner.o \
	bootstrap/test/stdlib.o \
	bootstrap/test/structure.o \
	bootstrap/astinput.o \
	bootstrap/astnode.o \
	bootstrap/errors.o \
	bootstrap/evaluator.o \
	bootstrap/nodekind.o \
	bootstrap/parser.o \
	bootstrap/stdlib.o \
	bootstrap/structure.o \
	bootstrap/symboltable.o \


bootstrap/simplex_stdlib.h: stdlib.simplex
	xxd -i $< > $@

-include ${DEPENDS}
