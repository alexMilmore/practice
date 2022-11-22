cmake_minimum_required(VERSION 3.12)
project(CppAptPkg)

include_directories(include)

# build
add_library(test_class_lib SHARED include/test_class.hpp src/test_class.cpp)

add_executable(hello_world_pkg src/main.cpp)
target_link_libraries(hello_world_pkg test_class_lib)

# install
set(CMAKE_INSTALL_PREFIX "${CMAKE_CURRENT_SOURCE_DIR}/install")
install(
  TARGETS
    test_class_lib
    hello_world_pkg
  ARCHIVE DESTINATION lib
  LIBRARY DESTINATION lib
  RUNTIME DESTINATION bin
)

install(
  FILES
    include/test_class.hpp
  DESTINATION include
)

# package
set(CPACK_DEBIAN_PACKAGE_MAINTAINER "alex")
set(CPACK_DEBIAN_PACKAGE_NAME "CppAptPkg")
set(CPACK_DEBIAN_PACKAGE_VERSION 1)
set(CPACK_DEBIAN_PACKAGE_ARCHITECTURE "arm64")
set(CPACK_DEBIAN_FILE_NAME "CppAptPkg_0.0.1_1_arm64.deb")
include(CPack) # values have to be before CPack include
