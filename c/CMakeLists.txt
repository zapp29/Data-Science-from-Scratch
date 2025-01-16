cmake_minimum_required(VERSION 3.10)

project(Data-Science-from-Scratch)

# Set the C standard (optional)
set(CMAKE_C_STANDARD 99)

# Add your library sources here
# add_library(myclib
#    src/myclib.c
#    src/another_source.c
# )

add_executable(
    main
    main.c
)

# Add include directories if needed
#target_include_directories(main PUBLIC
#    include
#)

include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/03597a01ee50ed33e9dfd640b249b4be3799d395.zip
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)


# Install your library (optional)
# install(TARGETS myclib DESTINATION lib)
 # install(FILES include/myclib.h DESTINATION include)
