#ifndef _STRUCTURE_H
#define _STRUCTURE_H

#include <functional>
#include <iostream>
#include <memory>
#include <string>
#include <vector>

namespace simplex {
  enum class StructureKind {
    cons,
    function,
    integer,
    invalid,
    floatingPoint,
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
      };
      std::string m_string;
      Function m_function;

    public:
      Structure();
      explicit Structure(int64_t);
      explicit Structure(double);
      explicit Structure(const std::string& s);
      explicit Structure(Function fn);
      Structure(std::shared_ptr<Structure>&& car, std::shared_ptr<Structure>&& cdr);

      // operators
      Structure operator()(std::vector<Structure> params);
      std::ostream& operator<<(std::ostream& os) const;

      // accessors
      StructureKind kind() const;
      int64_t integer() const;
      double floatingPoint() const;

      // comparison
      bool operator==(int64_t);
      bool operator==(double);
      bool operator==(int);
  };
};

#endif
