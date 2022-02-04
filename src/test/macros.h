#ifndef _MACROS_H
#define _MACROS_H

#define STR(s) #s
#define CHECK_MATH_1(op, p, result) \
  CHECK(e.eval(STR((op p))) == result); \
  CHECK(e.eval(STR((= (op p) result))) == true); \


#define CHECK_MATH_2(op, p1, p2, result) \
  CHECK(e.eval(STR((op p1 p2))) == result); \
  CHECK(e.eval(STR((= (op p1 p2) result))) == true); \

#endif
