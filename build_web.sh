#!/bin/bash
WASM_PKG=pkg
WEB_CLIENT=web-client
OUT_DIR=${WEB_CLIENT}/${WASM_PKG}

# Init web dir
[[ -d ${OUT_DIR} ]] && rm -R ${OUT_DIR}
[[ ! -d ${OUT_DIR} ]] && mkdir ${OUT_DIR}	

# This script runs a native executable.
cd game
wasm-pack build --target web --out-dir ../${OUT_DIR}
RESULT=$?

exit ${RESULT}
