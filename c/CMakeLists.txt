cmake_minimum_required(VERSION 3.10)

project(Data-Science-from-Scratch)

# Set the C standard (optional)
set(CMAKE_C_STANDARD 99)

# Add your library sources here
add_library(myclib
    src/myclib.c
    src/another_source.c
)

# Add include directories if needed
target_include_directories(myclib PUBLIC
    include
)

## CUnit
find_package(CUnit REQUIRED)                            # Find CUnit
add_executable(test_myclib test_myclib.c)               # Add your test executable
target_link_libraries(test_myclib CUnit::CUnit myclib)  # Link CUnit and your C library to the test executable


# Install your library (optional)
install(TARGETS myclib DESTINATION lib)
install(FILES include/myclib.h DESTINATION include)
