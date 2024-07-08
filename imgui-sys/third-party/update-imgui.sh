#!/bin/bash
set -euo pipefail

IMGUI_DIR=${1:-~/code/vendor/imgui}
CIMGUI_DIR=${2:-~/code/vendor/imgui}
SCRIPT_DIR=$(dirname ${0} | python3 -c 'import os, sys; print(os.path.abspath(sys.stdin.read().strip()))' )

cd ${SCRIPT_DIR}

./_update-imgui.sh $IMGUI_DIR v1.90.1 ./imgui-master/imgui
./_update-imgui.sh $IMGUI_DIR v1.90.1-docking ./imgui-docking/imgui

./_update-imgui.sh $IMGUI_DIR v1.90.1 ./imgui-master-freetype/imgui
./_update-imgui.sh $IMGUI_DIR v1.90.1-docking ./imgui-docking-freetype/imgui

if [ -n "$CIMGUI_DIR" ]; then
    for subdir in imgui-{master,docking}{,-freetype}; do
        (cd ./$subdir/ && ./update-cimgui-output.sh $CIMGUI_DIR)
    done
fi
