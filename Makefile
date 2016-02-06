.PHONY: test

CXXFLAGS=\
	  -DDEBUG \
		-g \
		--std=c++14 \
		-O0 \
		-MMD \
		-MP \
		-Wall \
		-Werror \

SRCS=$(wildcard *.cpp) \
		 $(wildcard test/*.cpp) \

OBJECTS=$(patsubst %.cpp,%.o,$(SRCS))
DEPENDS=${OBJECTS:.o=.d}

default: bootstrap/simplex

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

test: bootstrap/test/test
	./bootstrap/test/test -d yes

bootstrap/test/catch.h:
	curl -o $@ https://raw.githubusercontent.com/philsquared/Catch/v1.3.3/single_include/catch.hpp

bootstrap/test/%.cpp: bootstrap/test/catch.h

bootstrap/test/test: bootstrap/test/test.cpp \
	bootstrap/test/astnode.o \
	bootstrap/test/evaluator.o \
	bootstrap/test/parser.o \
	bootstrap/test/runner.o \
	bootstrap/astinput.o \
	bootstrap/astnode.o \
	bootstrap/errors.o \
	bootstrap/evaluator.o \
	bootstrap/nodekind.o \
	bootstrap/parser.o \
	bootstrap/stdlib.o \
	bootstrap/structure.o \
	bootstrap/symboltable.o \

-include ${DEPENDS}
