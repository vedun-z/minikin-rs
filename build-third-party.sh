#!/bin/bash

set -euo pipefail
set -x


SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

cd vcpkg
if [ ! -f ./vcpkg ] ; then
  ./bootstrap-vcpkg.sh
fi  

if [[ "$OSTYPE" == "linux-gnu" ]]; then
  PACKAGE_NAME="x64-linux"
else
  PACKAGE_NAME="x64-osx"
fi

./vcpkg install harfbuzz[icu]:$PACKAGE_NAME
