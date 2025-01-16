cmake_minimum_required(VERSION 3.10.2)
project(tests)

include(FetchContent)

FetchContent_Declare(
  googletest
  GIT_REPOSITORY https://github.com/google/googletest.git
  GIT_TAG        release-1.11.0
)
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

add_executable(
        vector_operations_tests
        vector_operations_tests.cc
)
add_executable(
        matrix_operations_tests
        matrix_operations_tests.cc
)

target_include_directories(vector_operations_tests PUBLIC include)
target_include_directories(matrix_operations_tests PUBLIC include)
target_link_libraries(
        vector_operations_tests
        vector_operations
        GTest::gtest_main
)
target_link_libraries(
        matrix_operations_tests
        matrix_operations
        GTest::gtest_main
)

include(GoogleTest)
gtest_add_tests(
        TARGET vector_operations_tests
)
gtest_add_tests(
        TARGET matrix_operations_tests
)
