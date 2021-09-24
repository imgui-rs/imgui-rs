#!/bin/bash
set -euo pipefail

SCRIPT_DIR=$(dirname ${0} | python3 -c 'import os, sys; print(os.path.abspath(sys.stdin.read().strip()))' )
CIMGUI_DIR=${1:?}

echo "${SCRIPT_DIR}"

pushd "${CIMGUI_DIR}"/generator > /dev/null

# Check if ${CIMGUI_DIR}/link exists as a symlink.
# If so, good, and we can tinker with it.
# Otherwise, ask user to remove it for us.
if [ -e ${CIMGUI_DIR}/imgui ] && [ ! -h ${CIMGUI_DIR}/imgui ]; then
   echo "Please remove ${CIMGUI_DIR}/imgui so this script can link it to correct imgui version"
   exit 1
fi

# Remove old symlink
rm ${CIMGUI_DIR}/imgui || echo "..."

# Link to C++ code contained in imgui-rs, not whatever if in cimgui's repo
ln -s ${SCRIPT_DIR}/imgui ${CIMGUI_DIR}/imgui

# Run the generator
luajit generator.lua gcc false -DIMGUI_USE_WCHAR32

# Tidy up
rm ${CIMGUI_DIR}/imgui # Remove symlink (no recursive rm)

popd > /dev/null

cp "${CIMGUI_DIR}"/cimgui.{h,cpp} "${SCRIPT_DIR}"/
cp "${CIMGUI_DIR}"/generator/output/* "${SCRIPT_DIR}"/
