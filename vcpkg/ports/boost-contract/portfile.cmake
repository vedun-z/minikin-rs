# Automatically generated by boost-vcpkg-helpers/generate-ports.ps1

vcpkg_from_github(
    OUT_SOURCE_PATH SOURCE_PATH
    REPO boostorg/contract
    REF boost-1.73.0
    SHA512 670e7d9e64d838bda32ec8748213961b8b7ee96d2918f2e6b71005b488de22b5f69c113de01908c88ab9dcdba6f69a0a2f64b10a4b5d5d319a7e76de17be2d72
    HEAD_REF master
)

include(${CURRENT_INSTALLED_DIR}/share/boost-build/boost-modular-build.cmake)
boost_modular_build(SOURCE_PATH ${SOURCE_PATH})
include(${CURRENT_INSTALLED_DIR}/share/boost-vcpkg-helpers/boost-modular-headers.cmake)
boost_modular_headers(SOURCE_PATH ${SOURCE_PATH})