
# Build web
TARGET=Web
echo "${TARGET}"
echo "${TARGET}"
echo "${TARGET}"
echo "${TARGET}"
echo "${TARGET}"
./build_web.sh;
RESULT=$?
[[ ! ${RESULT} == 0 ]] && echo "target ${TARGET} failed, exiting..." && exit ${RESULT}

# Build rest of things
cd game;
function build_target {
	TARGET=${1}
	echo "${TARGET}"
	echo "${TARGET}"
	echo "${TARGET}"
	echo "${TARGET}"
	echo "${TARGET}"

	rustup target add ${TARGET}
	cargo check --target ${TARGET}
	
	RESULT=$?
	[[ ! ${RESULT} == 0 ]] && echo "target ${TARGET} failed, exiting..." && exit ${RESULT}
};


build_target x86_64-pc-windows-msvc
build_target x86_64-pc-windows-gnu
build_target x86_64-unknown-linux-gnu
build_target x86_64-apple-darwin
build_target aarch64-apple-darwin
# TODO: androids
# TODO: iOS
