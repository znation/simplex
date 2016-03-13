#define UNIT_TESTING
#include "../structure.h"
#include "../errors.h"

#include "catch.h"

using namespace simplex;

TEST_CASE("boolean [structure]") {
  std::vector<bool> bools = { false, true };
  for (bool b : bools) {
    Structure s(b);
    CHECK(s.kind() == StructureKind::boolean);
    CHECK(s.boolean() == b);
  }
  CHECK(Structure(true).to_string() == "true");
  CHECK(Structure(false).to_string() == "false");
}

TEST_CASE("byte [structure]") {
  std::vector<uint8_t> bytes = { 0, 1, 2, 254, 255 };
  for (uint8_t b : bytes) {
    Structure s(b);
    CHECK(s.kind() == StructureKind::byte);
    CHECK(s.byte() == b);
    std::stringstream ss;
    ss << b;
    CHECK(s.to_string() == ss.str());
  }
}

TEST_CASE("cons [structure]") {
  std::vector<std::string> strings = {
    "",
    "a",
    "ab",
    "abc"
  };
  for (const auto& str : strings) {
    Structure s(str);
    CHECK(s.kind() == StructureKind::cons);
    CHECK(s.string() == str);
  }
}
