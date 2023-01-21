#!/bin/bash
set -euo pipefail

SCRIPT_DIR=$(dirname ${0} | python3 -c 'import os, sys; print(os.path.abspath(sys.stdin.read().strip()))' )

cd ${SCRIPT_DIR}
./_update-imgui.sh ~/code/vendor/imgui v1.89.2 ./imgui-master/imgui
./_update-imgui.sh ~/code/vendor/imgui d822c65317ba881798bed8fce9ffba267d27dada ./imgui-docking/imgui

./_update-imgui.sh ~/code/vendor/imgui v1.89.2 ./imgui-master-freetype/imgui
./_update-imgui.sh ~/code/vendor/imgui d822c65317ba881798bed8fce9ffba267d27dada ./imgui-docking-freetype/imgui
