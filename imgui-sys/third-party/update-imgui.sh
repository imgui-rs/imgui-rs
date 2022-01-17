#!/bin/bash
set -euo pipefail

SCRIPT_DIR=$(dirname ${0} | python3 -c 'import os, sys; print(os.path.abspath(sys.stdin.read().strip()))' )

cd ${SCRIPT_DIR}
./_update-imgui.sh ~/code/vendor/imgui v1.86 ./imgui-master/imgui
./_update-imgui.sh ~/code/vendor/imgui 15b4a064f9244c430e65214f7249b615fb394321 ./imgui-docking/imgui
