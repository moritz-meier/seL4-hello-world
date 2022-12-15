cmake_minimum_required(VERSION 3.18.4)
project(hello-world C)

add_executable(hello-world src/main.c)
target_link_libraries(hello-world sel4muslcsys  muslc)

DeclareRootserver(hello-world)
