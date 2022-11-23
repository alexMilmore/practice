cmake_minimum_required(VERSION 3.12)
project(min_genetic_mutation)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_BUILD_TYPE Debug)

# build
include_directories(include)
add_library(ans STATIC include/solution.hpp src/solution.cpp)

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
add_test(gtest_test test_ans)
