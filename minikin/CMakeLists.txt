cmake_minimum_required(VERSION 3.10)
project(minikin)

set(CMAKE_CXX_STANDARD 17)

option(MSVC_DYNAMIC_RT OFF)


if (UNIX)
  if(APPLE)
    set(VCPKG_NAME x64-osx)
  else()  
    set(VCPKG_NAME x64-linux)
  endif()
else() 
  set(VCPKG_NAME x64-windows-static)
  set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} /MT")
  set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} /MTd")
endif()	


include_directories(${CMAKE_SOURCE_DIR}/../vcpkg/installed/${VCPKG_NAME}/include/)
include_directories(${CMAKE_SOURCE_DIR}/../vcpkg/installed/${VCPKG_NAME}/include/harfbuzz)
include_directories(src)

FILE(GLOB minikin src/minikin/*.cpp)
FILE(GLOB log src/log/*.cc)
FILE(GLOB utils src/utils/*.cpp)

add_library(minikin STATIC library.cpp library.h ${minikin} ${log} ${utils})
set_property(TARGET minikin PROPERTY MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>$<$<BOOL:${MSVC_DYNAMIC_RT}>:DLL>")
install (TARGETS minikin  DESTINATION .)