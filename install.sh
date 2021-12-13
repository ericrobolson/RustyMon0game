# Ensure line endings work properly
git config --global core.autocrlf false

echo "OS: ${OSTYPE}"

INSTALLS=.installs/

WIN_WASM_PACK_URL=https://github.com/rustwasm/wasm-pack/releases/download/v0.10.1/wasm-pack-init.exe
WIN_WASM_PACK_INSTALL=${INSTALLS}wasmpack.exe

# Make install folder
[[ -d ${INSTALLS} ]] && rm -R ${INSTALLS}
[[ ! -d ${INSTALLS} ]] && mkdir ${INSTALLS}

#
# Platform installs
#
# WASM pack
# Yarn
# 

# Linux
if [[ "${OSTYPE}" == "linux-gnu" ]]; then 
	sudo apt-get update &&
	sudo apt install build-essential &&
	sudo apt install yarn
# Windows
elif [[ "${OSTYPE}" == "cygwin" ]]; then
	# Install wasm pack
	curl ${WIN_WASM_PACK_URL} -L --output ${WIN_WASM_PACK_INSTALL}
	./${WIN_WASM_PACK_INSTALL}

	# Install yarn
	choco install yarn -y
elif [[ "${OSTYPE}" == "darwin20" ]]; then
	brew update
	brew install node
	brew install yarn
else
	echo "Unhandled OS: ${OSTYPE}"
	exit 1
fi

# 
# Generic installs
#

# Yarn installs
npm install -g live-server

# Rust installs
rustup update
cargo install wasm-pack || true
