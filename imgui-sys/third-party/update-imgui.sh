#!/bin/bash
set -euo pipefail

SCRIPT_DIR=$(dirname ${0} | python3 -c 'import os, sys; print(os.path.abspath(sys.stdin.read().strip()))' )

cd ${SCRIPT_DIR}
./_update-imgui.sh ~/code/vendor/imgui v1.89.1 ./imgui-master/imgui
./_update-imgui.sh ~/code/vendor/imgui 540909bddf2f2b094a650b4bf5d01757fbd69418 ./imgui-docking/imgui
