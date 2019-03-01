cmake_minimum_required(VERSION 3.9)
project (AdventOfCode2017 VERSION 0.0.1)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS ON)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++17")
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
include(ExternalProject)

set(THREADS_PREFER_PTHREAD_FLAG ON)
find_package(Threads REQUIRED)

find_program(CCACHE_FOUND ccache)
if(CCACHE_FOUND)
    set_property(GLOBAL PROPERTY RULE_LAUNCH_COMPILE ccache)
    set_property(GLOBAL PROPERTY RULE_LAUNCH_LINK ccache)
endif(CCACHE_FOUND)

# FMT
ExternalProject_Add(Fmt
  PREFIX fmt
  GIT_REPOSITORY https://github.com/fmtlib/fmt
  GIT_TAG 4.0.0
  CMAKE_CACHE_ARGS  "-DFMT_DOC:BOOL=OFF" "-DFMT_INSTALL:BOOL=ON" "-DFMT_TEST:BOOL=OFF"
  CMAKE_ARGS
	-DCMAKE_INSTALL_PREFIX=<INSTALL_DIR>
)
set(FMT_INCLUDE_DIR "${CMAKE_BINARY_DIR}/fmt/include")
if (WIN32)
   set(FMT_LIB "${CMAKE_BINARY_DIR}/fmt/lib/fmt.lib")
else()
   set(FMT_LIB "${CMAKE_BINARY_DIR}/fmt/lib/libfmt.a")
endif()

add_executable(aoc
    src/Advent.cpp
)

add_dependencies(aoc Fmt)
target_include_directories(aoc PUBLIC
    ${FMT_INCLUDE_DIR}
    include
)
target_link_libraries(aoc 
    ${FMT_LIB}
    Threads::Threads
)
install(TARGETS aoc RUNTIME DESTINATION ${PROJECT_SOURCE_DIR})
