cmake_minimum_required(VERSION 3.17)
project(ranges_demo)

include(FetchContent)
FetchContent_Declare(
  ranges
  GIT_REPOSITORY "https://github.com/ericniebler/range-v3.git"
)
FetchContent_MakeAvailable(ranges)
include_directories(${Range-v3_SOURCE_DIR}/include)

add_executable(ranges_demo main.cpp)
set_target_properties(ranges_demo PROPERTIES
  CXX_STANDARD 20
  CXX_EXTENSIONS OFF)
