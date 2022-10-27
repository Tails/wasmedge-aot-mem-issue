# https://github.com/second-state/wasmedge-quickjs

WASM_BIN_OUTPUT_NAME=prerendering_wasm
WASM_BIN_AOT_NAME=${WASM_BIN_OUTPUT_NAME}_aot

run-dev: run-prepare
	source ${HOME}/.wasmedge/env && \
	wasmedge \
		--dir .:. \
		../target/wasm32-wasi/release/${WASM_BIN_OUTPUT_NAME}.wasm

build-run: \
	build \
	run-dev

build: \
	build-bin \
	copy-wasm-bin

build-bin: install-rs
	echo "building prerendering_wasm from Makefile"
	rm -rf ./dist
	cargo build \
		--target wasm32-wasi \
		--manifest-path ./Cargo.toml \
		--release

# https://wasmedge.org/book/en/cli/wasmedgec.html
build-bin-aot:
	wasmedgec \
			--enable-all \
			--optimize 0 \
		../target/wasm32-wasi/release/${WASM_BIN_OUTPUT_NAME}.wasm \
		./dist/${WASM_BIN_AOT_NAME}.so

copy-wasm-bin:
	mkdir -p ./dist
	cp ../target/wasm32-wasi/release/${WASM_BIN_OUTPUT_NAME}.wasm ./dist/
	-cp ../target/wasm32-wasi/release/${WASM_BIN_AOT_NAME}.so ./dist/

#
# CONFIG
#

install: \
	install-rs \
	install-llvm \
	install-cli

install-llvm:
	brew install llvm

install-rs:
	rustup target add wasm32-wasi

install-cli:
	curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh \
		| bash