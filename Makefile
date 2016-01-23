.PHONY: test

CXXFLAGS=\
	  -DDEBUG \
		-g \
		--std=c++14 \
		-Wall \
		-Werror \

all: simplex

simplex: simplex.cpp \
	astinput.o \
	astnode.o \
	errors.o \
	nodekind.o \
	parser.o \

test: test/test
	./test/test

test/catch.h:
	curl -o $@ https://raw.githubusercontent.com/philsquared/Catch/v1.3.3/single_include/catch.hpp

test/test.cpp: test/catch.h

test/test: test/test.cpp \
	test/parser.o \
	astinput.o \
	astnode.o \
	errors.o \
	nodekind.o \
	parser.o \
