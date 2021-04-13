#!/bin/bash
set -euo pipefail

SCRIPT_DIR=$(dirname ${0})
CIMGUI_DIR=${1:?}

echo "${SCRIPT_DIR}"

pushd "${CIMGUI_DIR}"/generator > /dev/null
## luajit generator.lua gcc false -DIMGUI_USE_WCHAR32
luajit generator.lua gcc internal -DIMGUI_USE_WCHAR32
popd > /dev/null

cp "${CIMGUI_DIR}"/cimgui.{h,cpp} "${SCRIPT_DIR}"/
cp "${CIMGUI_DIR}"/generator/output/* "${SCRIPT_DIR}"/
