#ifndef _ASTINPUT_H
#define _ASTINPUT_H

#include <string>

namespace simplex {
  class ASTInput {
    private:
      const char *m_stream;
      size_t m_len;
      size_t m_line;
      size_t m_col;
      const std::string m_filename;
#ifdef DEBUG_ASTINPUT
      const char *m_originalStream;
      size_t m_originalLen;
#endif

    public:
      ASTInput(const std::string& filename, const char *stream, size_t len);
      void advance(size_t);
      const char *get();
      char next();
      char peek() const;
      std::string remaining() const;
      size_t size() const;
      size_t line() const;
      size_t col() const;
  };
};

#endif
