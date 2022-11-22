#pragma once

#include <iostream>
#include <string>

class TestClass {

public:
  std::string data_;

  TestClass(std::string data);
  void print_data();
};
