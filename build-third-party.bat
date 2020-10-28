cd %~dp0

cd vcpkg

if not exist vcpkg.exe (
  bootstrap-vcpkg.bat 
)

vcpkg.exe install harfbuzz[icu]:x64-windows


