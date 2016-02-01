#include "astinput.h"

#include <cassert>

#ifdef DEBUG_ASTINPUT
#include <iostream>
#endif

using namespace simplex;

ASTInput::ASTInput(const char *stream, size_t len) :
#ifdef DEBUG_ASTINPUT
  stream(stream), len(len), originalStream(stream), originalLen(len) {
  std::cout << "DEBUG: input stream at "
            << std::hex
            << "0x"
            << reinterpret_cast<size_t>(this->originalStream)
            << ", size "
            << std::dec
            << this->originalLen
            << std::endl;
  if (this->originalLen <= 80) {
    std::cout << '"'
              << this->originalStream
              << '"'
              << std::endl;
  }
}
#else
  stream(stream), len(len) { }
#endif

const char *ASTInput::get() {
  return stream;
}

char ASTInput::next() {
  char ret = this->peek();
  this->advance(1);
  return ret;
}

void ASTInput::advance(size_t n) {
  assert(this->size() >= n);
  this->stream += n;
  this->len -= n;
}

char ASTInput::peek() const {
  assert(this->size() != 0);
  return this->stream[0];
}

size_t ASTInput::size() const {
  return this->len;
}

std::string ASTInput::remaining() const {
  return std::string(this->stream, this->len);
}
