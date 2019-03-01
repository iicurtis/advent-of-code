add_subdirectory(lib)

add_executable(Advent
  Advent.cpp)

target_compile_options(Advent PUBLIC ${ADVENT_CXX_FLAGS})
target_link_libraries(Advent Threads::Threads)
install(TARGETS Advent RUNTIME DESTINATION ${PROJECT_SOURCE_DIR})
