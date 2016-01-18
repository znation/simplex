CXXFLAGS=\
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
