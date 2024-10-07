#!/bin/bash
set -euo pipefail

SCRIPT_DIR=$(dirname ${0} | python3 -c 'import os, sys; print(os.path.abspath(sys.stdin.read().strip()))' )
IMGUI_DIR=${1:?}
COMMITISH=${2:?}
OUT_DIR=${3:?}

echo "imgui_dir = $IMGUI_DIR";
echo "script_dir = $SCRIPT_DIR";
echo "commit/tag = $COMMITISH";

# Location of temporary checkout of imgui at specified commit (or branch)
CHECKOUT="${SCRIPT_DIR}"/_temp_imgui_worktree

# this can happen on failed runs
if [ -d "${CHECKOUT}" ]; then
  rm -rf "${CHECKOUT}";
fi


# Make checkout
pushd "${IMGUI_DIR}" > /dev/null

# Sanity check the supplied imgui path
git rev-parse HEAD
ls imgui.h

# Get files from specified rev
mkdir "${CHECKOUT}"

git archive "${COMMITISH}" | tar xC "${CHECKOUT}"

popd > /dev/null

# Copy required files
mkdir -p ${OUT_DIR}/
mkdir -p ${OUT_DIR}/misc/freetype/

cp "${CHECKOUT}"/LICENSE.txt "${OUT_DIR}"/
cp "${CHECKOUT}"/*.{h,cpp} "${OUT_DIR}"/
cp -r "${CHECKOUT}"/misc/freetype/ "${OUT_DIR}"/misc/

# Clean up
rm -r "${CHECKOUT}"
