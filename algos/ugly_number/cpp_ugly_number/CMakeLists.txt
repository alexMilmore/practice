cmake_minimum_required(VERSION 3.12)
project(ugly_number)

# build
include_directories(include)

add_library(ans STATIC include/ans.hpp src/ans.cpp)

# test
include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/03597a01ee50ed33e9dfd640b249b4be3799d395.zip
)
FetchContent_MakeAvailable(googletest)
enable_testing()

add_executable(test_ans test/test_ans.cpp)
target_link_libraries(test_ans ans GTest::gtest_main)
add_test(gtest_ans test_ans)
