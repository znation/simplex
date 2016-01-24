#ifndef _ASTINPUT_H
#define _ASTINPUT_H

#include <string>

namespace simplex {
  class ASTInput {
    private:
      const char *stream;
      size_t len;
#ifdef DEBUG
      const char *originalStream;
      size_t originalLen;
#endif

    public:
      ASTInput(const char *stream, size_t len);
      void advance(size_t);
      const char *get();
      char next();
      char peek() const;
      std::string remaining() const;
      size_t size() const;
  };
};

#endif
