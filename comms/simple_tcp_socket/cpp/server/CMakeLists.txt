cmake_minimum_required(VERSION 3.12)
project(simple_tcp_server)

# build
add_executable(server src/server.cpp)

add_executable(client src/client.cpp)
