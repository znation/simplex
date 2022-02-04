#include "astinput.h"

#include <cassert>

#ifdef DEBUG_ASTINPUT
#include <iostream>
#endif

using namespace simplex;

ASTInput::ASTInput(const std::string& filename, const char *stream, size_t len) :
#ifdef DEBUG_ASTINPUT
  m_stream(stream),
  m_len(len),
  m_line(1),
  m_col(1),
  m_filename(filename),
  m_originalStream(stream),
  m_originalLen(len) {

  std::cout << "DEBUG: input stream at "
            << std::hex
            << "0x"
            << reinterpret_cast<size_t>(this->m_originalStream)
            << ", size "
            << std::dec
            << this->m_originalLen
            << std::endl;
  if (this->m_originalLen <= 80) {
    std::cout << '"'
              << this->m_originalStream
              << '"'
              << std::endl;
  }
}
#else
  m_stream(stream), m_len(len), m_line(1), m_col(1), m_filename(filename) { }
#endif

const char *ASTInput::get() {
  return m_stream;
}

char ASTInput::next() {
  char ret = this->peek();
  this->advance(1);
  return ret;
}

void ASTInput::advance(size_t n) {
  assert(this->size() >= n);
  for (size_t i=0; i<n; i++) {
    char next = this->peek();
    this->m_stream++;
    this->m_len--;
    if (next == '\n') {
      this->m_col = 0;
      this->m_line++;
    } else {
      this->m_col++;
    }
  }
}

char ASTInput::peek() const {
  assert(this->size() != 0);
  return this->m_stream[0];
}

size_t ASTInput::size() const {
  return this->m_len;
}

std::string ASTInput::remaining() const {
  return std::string(this->m_stream, this->m_len);
}

size_t ASTInput::line() const {
  return m_line;
}

size_t ASTInput::col() const {
  return m_col;
}
