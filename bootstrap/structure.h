#ifndef _STRUCTURE_H
#define _STRUCTURE_H

#include <functional>
#include <iostream>
#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

#include "astnode.h"

namespace simplex {
  enum class StructureKind {
    boolean,
    byte,
    cons,
    dict,
    floatingPoint,
    function,
    integer,
    invalid,
    nil,
  };

  const char* StructureKindName(StructureKind kind);

  class Structure {
    public:
      typedef std::function<Structure(const ASTNode&, std::vector<Structure>)> Function;
      typedef std::unordered_map<std::string, Structure> Dict;

    private:
      StructureKind m_kind;
      std::shared_ptr<Structure> m_car;
      std::shared_ptr<Structure> m_cdr;
      union {
        int64_t m_int;
        double m_float;
        bool m_bool;
        uint8_t m_byte;
      };
      Function m_function;
      Dict m_dict;

      void cons_from_string(const char * str, size_t len);

    public:
      Structure();
      explicit Structure(bool);
      explicit Structure(uint8_t);
      explicit Structure(int64_t);
      explicit Structure(double);
      explicit Structure(const std::string& s);
      explicit Structure(Function fn);
      explicit Structure(Dict dict);
      Structure(const char * str, size_t len);
      Structure(std::shared_ptr<Structure>&& car, std::shared_ptr<Structure>&& cdr);
      static Structure Nil();

      // operators
      Structure operator()(const ASTNode& node, std::vector<Structure> params);
      operator bool() const;

      // accessors
      StructureKind kind() const;
      bool boolean() const;
      uint8_t byte() const;
      int64_t integer() const;
      double floatingPoint() const;
      const Structure& car() const;
      const Structure& cdr() const;
      std::string string() const;
      const Dict& dict() const;

      // comparison
      bool operator==(const Structure&) const;
      bool operator==(bool) const;
      bool operator==(int64_t) const;
      bool operator==(double) const;
      bool operator==(int) const;
      bool operator==(const std::string&) const;
      bool operator==(const char *) const;

      // conversion
      std::string to_string() const;

      // i/o
      void print(std::ostream& stream) const;
  };

  std::ostream& operator<<(std::ostream&, const Structure&);
};


#endif
