cmake_minimum_required(VERSION 3.10)

project(Data-Science-from-Scratch)

# Set the C standard (optional)
set(CMAKE_C_STANDARD 99)

# Add scalar operations library
add_library(scalar_operations
    src/scalar_operations.c
)

target_include_directories(scalar_operations PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/src
)

add_executable(
    main
    main.c
)

target_link_libraries(main PRIVATE scalar_operations)

include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/03597a01ee50ed33e9dfd640b249b4be3799d395.zip
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

enable_testing()

# Add the test executable
add_executable(
    scalar_operations_test
    tests/scalar_operations_test.cpp
)

target_link_libraries(
    scalar_operations_test
    GTest::gtest_main
    scalar_operations
)

include(GoogleTest)
gtest_discover_tests(scalar_operations_test)
