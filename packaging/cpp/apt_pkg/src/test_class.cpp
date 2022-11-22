#include "test_class.hpp"

TestClass::TestClass(std::string data) {
  data_ = data;
}

void TestClass::print_data() {
  std::cout << "CLASS: " << data_ << std::endl;
}
