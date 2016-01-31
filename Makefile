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

all: simplex

simplex: simplex.cpp \
	astinput.o \
	astnode.o \
	errors.o \
	evaluator.o \
	main.o \
	nodekind.o \
	parser.o \
	stdlib.o \
	structure.o \

test: test/test
	./test/test -d yes

test/catch.h:
	curl -o $@ https://raw.githubusercontent.com/philsquared/Catch/v1.3.3/single_include/catch.hpp

test/test.cpp: test/catch.h

test/test: test/test.cpp \
	test/astnode.o \
	test/evaluator.o \
	test/parser.o \
	test/runner.o \
	astinput.o \
	astnode.o \
	errors.o \
	evaluator.o \
	nodekind.o \
	parser.o \
	stdlib.o \
	structure.o \

-include ${DEPENDS}
