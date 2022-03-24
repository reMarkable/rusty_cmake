cmake_minimum_required(VERSION 3.16)

set(CMAKE_EXPORT_COMPILE_COMMANDS true)
SET(CMAKE_SYSTEM_NAME Linux)
project(rusty_cmake CXX)
#set(Rust_CARGO "/home/klimatt/codex_rust_sdk/sysroots/x86_64-codexsdk-linux/usr/bin/cargo")
message(STATUS "Cargo : ${Rust_CARGO}")
set(Rust_CARGO_TARGET "arm-remarkable-linux-gnueabi")
#if("${CMAKE_SYSTEM_NAME}" STREQUAL "Windows")
#    set(Rust_CARGO_TARGET "x86_64-pc-windows-gnu")
#elseif("${CMAKE_SYSTEM_NAME}" STREQUAL "Linux")s
#    set(Rust_CARGO_TARGET "x86_64-unknown-linux-gnu")
#else()
#    message(FATAL_ERROR "hardcoded platformchecks not supported outside windows-gnu and linux-gnu")
#endif()
#list(APPEND CMAKE_MODULE_PATH "home/klimatt/codex_rust_sdk/sysroots/x86_64-codexsdk-linux/usr/lib/cmake/Corrosion")

SET(CMAKE_CXX_FLAGS "${CONFIGURE_FLAGS} ${CXXFLAGS}")
SET(CMAKE_C_FLAGS "${CONFIGURE_FLAGS} ${CFLAGS}")

set(Corrosion_DIR "/home/klimatt/codex_rust_sdk/sysroots/x86_64-codexsdk-linux/usr/lib/cmake/Corrosion/") # Need to add Corrosion_DIR in sdk env.
message(STATUS "Corrosion_DIR : ${Corrosion_DIR}")

find_package(Corrosion REQUIRED)


include(cmake/corrosion_cxx.cmake)
#add_subdirectory(thirdparty/corrosion)

add_subdirectory(src)
