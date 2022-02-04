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


SRCS=$(wildcard src/*.cpp) \
		 $(wildcard src/test/*.cpp) \


OBJECTS=$(patsubst %.cpp,%.o,$(SRCS))
DEPENDS=${OBJECTS:.o=.d}

default: debug

debug: CXXFLAGS += -DDEBUG -g -O0
debug: CCFLAGS += -DDEBUG -g -O0
debug: target/simplex

release: CXXFLAGS += -O3
release: CCFLAGS += -O3
release: src/simplex

clean:
	rm -f ${OBJECTS} ${DEPENDS} build/* target/*

build/%.o: src/%.cpp
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -c $< -o $@

target/simplex: src/simplex.cpp \
	build/astinput.o \
	build/astnode.o \
	build/errors.o \
	build/evaluator.o \
	build/main.o \
	build/nodekind.o \
	build/parser.o \
	build/repl.o \
	build/stdlib.o \
	build/structure.o \
	build/symboltable.o
	mkdir -p target
	$(CXX) $(LDFLAGS) $^ $(LOADLIBES) $(LDLIBS) -o $@

test: CXXFLAGS += -DDEBUG -g -O0
test: CCFLAGS += -DDEBUG -g -O0
test: test/test
	./test/test -d yes

test/catch.h:
	curl -L -o $@ https://github.com/catchorg/Catch2/releases/download/v2.13.8/catch.hpp

# extra dependencies
src/evaluator.cpp: src/simplex_stdlib.h
test/astnode.cpp: test/catch.h
test/evaluator.cpp: test/catch.h
test/parser.cpp: test/catch.h
test/runner.cpp: test/catch.h
test/stdlib.cpp: test/catch.h

test/test: test/test.cpp \
	build/test/astnode.o \
	build/test/evaluator.o \
	build/test/parser.o \
	build/test/runner.o \
	build/test/stdlib.o \
	build/test/structure.o \
	build/astinput.o \
	build/astnode.o \
	build/errors.o \
	build/evaluator.o \
	build/nodekind.o \
	build/parser.o \
	build/stdlib.o \
	build/structure.o \
	build/symboltable.o \


src/simplex_stdlib.h: src/stdlib.simplex
	xxd -i $< > $@
