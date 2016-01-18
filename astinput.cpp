#include "astinput.h"

#include <cassert>

using namespace simplex;

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
