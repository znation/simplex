#ifndef _STRUCTURE_H
#define _STRUCTURE_H

#include <functional>
#include <iostream>
#include <memory>
#include <string>
#include <vector>

namespace simplex {
  enum class StructureKind {
    boolean,
    cons,
    function,
    integer,
    invalid,
    floatingPoint,
    nil,
    string
  };

  class Structure {
    public:
      typedef std::function<Structure(std::vector<Structure>)> Function;

    private:
      StructureKind m_kind;
      std::shared_ptr<Structure> m_car;
      std::shared_ptr<Structure> m_cdr;
      union {
        int64_t m_int;
        double m_float;
        bool m_bool;
      };
      std::string m_string;
      Function m_function;

    public:
      Structure();
      explicit Structure(bool);
      explicit Structure(int64_t);
      explicit Structure(double);
      explicit Structure(const std::string& s);
      explicit Structure(Function fn);
      Structure(std::shared_ptr<Structure>&& car, std::shared_ptr<Structure>&& cdr);
      static Structure Nil();

      // operators
      Structure operator()(std::vector<Structure> params);

      // accessors
      StructureKind kind() const;
      bool boolean() const;
      int64_t integer() const;
      double floatingPoint() const;
      std::string string() const;
      const Structure& car() const;
      const Structure& cdr() const;

      // comparison
      bool operator==(const Structure&) const;
      bool operator==(bool) const;
      bool operator==(int64_t) const;
      bool operator==(double) const;
      bool operator==(int) const;
  };

  std::ostream& operator<<(std::ostream&, const Structure&);
};


#endif
