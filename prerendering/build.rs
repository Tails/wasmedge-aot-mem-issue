use wasmedge_sdk::{error::HostFuncError, params, wat2wasm, CallingFrame, Executor, ImportObjectBuilder, Module, Store, WasmValue, config::Config, Vm, AsInstance, CompilerOutputFormat, CompilerOptimizationLevel, WasmEdgeResult, WasmValTypeList, ImportObject, Compiler};
use wasmedge_sdk::config::{ConfigBuilder, HostRegistrationConfigOptions, CommonConfigOptions, CompilerConfigOptions};
use wasmedge_sdk::WasmVal;
use wasmedge_sys::Memory;

use std::env;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

// todo: resolve this from parture folder helper path util
const WASM_BLOB_PATH : &str
    = "../prerendering_wasm/dist/prerendering_wasm.wasm";

const AOT_OUTPUT_PATH : &str
    = "../prerendering_wasm/dist/prerendering_wasm_aot.so";

const OPT_LEVEL: CompilerOptimizationLevel
    = CompilerOptimizationLevel::O0;

// const WASM_BLOB : &[u8] = include_bytes!("../partage_prerendering_wasm/partage_prerendering_wasm.wasm");

// create vm config
fn config() -> Config {
    let common_options = CommonConfigOptions::default()
        // .bulk_memory_operations(true)
        // .multi_value(true)
        // .mutable_globals(true)
        // .non_trap_conversions(true)
        // .reference_types(true)
        // .sign_extension_operators(true)
        // .simd(true)
        // .multi_memories(true)
        ;

    let compiler_options = CompilerConfigOptions::default()
        .dump_ir(false)
        .generic_binary(false)
        .interruptible(false)
        .optimization_level(OPT_LEVEL)
        .out_format(CompilerOutputFormat::Native)
        ;

    let host_reg_options = HostRegistrationConfigOptions::new()
        .wasi(true);

    let result = ConfigBuilder::new(common_options)
        .with_host_registration_config(host_reg_options)
        .with_compiler_config(compiler_options)
        .build();

    assert!(result.is_ok());
    result.unwrap()
}

/// build the input WASM (interpreted)
fn build_wasm() -> WasmEdgeResult<()> {
    println!("building standard wasm binary...");

    // tell cargo to invalidate build cache if the _wasm package changes
    println!("cargo:rerun-if-changed=../prerendering_wasm/src/main.rs");

    let res = Command::new("make")
        .current_dir("../prerendering_wasm")
        .arg("build")
        .output()
        .expect("Failed to execute command");

    if !res.status.success() {
        panic!("{}", String::from_utf8(res.stderr).unwrap());
    }

    Ok(())
}

/// produce native optimized binary
fn compile_sdk() -> WasmEdgeResult<()> {
    println!("compiling AOT wasm binary through Rust SDK...");

    std::fs::create_dir_all("../prerendering_wasm/dist/")
        .expect("failed to make output dirs");

    Compiler::new(Some(&config()))?
        .compile_from_file(WASM_BLOB_PATH,  AOT_OUTPUT_PATH)
        // .compile_from_bytes(WASM_BLOB, "./aot/aot.wasm")
}

/// produce native optimized binary
fn compile_cli() {
    println!("building standard wasm binary using wasmedge CLI...");

    let res = Command::new("make")
        .current_dir("../prerendering_wasm")
        .arg("build-bin-aot")
        .output()
        .expect("Failed to execute command");

    if !res.status.success() {
        panic!("{}", String::from_utf8(res.stderr).unwrap());
    }
}

fn compile() {
    println!("cargo:rerun-if-changed=../prerendering_wasm/dist/prerenderer_wasm.wasm");

    // do not compile here if we are not running --release because it takes ages
    // #[cfg(not(debug_assertions))] {
    //     compile_sdk();
    // }
    //
    // // if we run in release mode, no need for external binary
    // #[cfg(debug_assertions)] {
    //     compile_cli();
    // }

    compile_sdk();
}

pub fn main() {
    build_wasm().and_then(|()|
        Ok(compile()));
}