#!/bin/bash
set -euo pipefail

SCRIPT_DIR=$(dirname ${0})
CIMGUI_DIR=${1:?}

COMMIT_MAIN=e5cb04b132cba94f902beb6186cb58b864777012
COMMIT_DOCKING=ac08593b9645aee7e086b1e9b98a6a1d79d09210

echo "${SCRIPT_DIR}"

OUTPUT_MAIN="${SCRIPT_DIR}/main"
OUTPUT_DOCKING="${SCRIPT_DIR}/docking"

mkdir -p "${OUTPUT_MAIN}"
mkdir -p "${OUTPUT_DOCKING}"

pushd "${CIMGUI_DIR}"/generator > /dev/null
(cd ../imgui && git reset --hard "$COMMIT_MAIN")
luajit generator.lua gcc false
popd > /dev/null

cp "${CIMGUI_DIR}"/cimgui.{h,cpp} "${OUTPUT_MAIN}"/
cp "${CIMGUI_DIR}"/generator/output/* "${OUTPUT_MAIN}"/


pushd "${CIMGUI_DIR}"/generator > /dev/null
(cd ../imgui && git reset --hard "$COMMIT_DOCKING")
luajit generator.lua gcc false
popd > /dev/null

cp "${CIMGUI_DIR}"/cimgui.{h,cpp} "${OUTPUT_DOCKING}"/
cp "${CIMGUI_DIR}"/generator/output/* "${OUTPUT_DOCKING}"/

cat <<EOF > "${SCRIPT_DIR}"/IMGUI_VERSIONS
$COMMIT_MAIN;$COMMIT_DOCKING;
EOF
