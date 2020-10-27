# Automatically generated by boost-vcpkg-helpers/generate-ports.ps1

vcpkg_from_github(
    OUT_SOURCE_PATH SOURCE_PATH
    REPO boostorg/bimap
    REF boost-1.73.0
    SHA512 63b3fb5926ee7e1d86872f766213a6d31d146f0db349c676754cc0c4d5050f8690773ec694cc2e820b96f278907e63d0e32c3e588b74ea415359955f4ec3d9b8
    HEAD_REF master
)

include(${CURRENT_INSTALLED_DIR}/share/boost-vcpkg-helpers/boost-modular-headers.cmake)
boost_modular_headers(SOURCE_PATH ${SOURCE_PATH})