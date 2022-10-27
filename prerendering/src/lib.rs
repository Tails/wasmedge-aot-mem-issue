// If the version of rust used is less than v1.63, please uncomment the follow attribute.
#![feature(explicit_generic_args_with_impl_trait)]
#![feature(never_type)]

use wasmedge_sdk::{error::HostFuncError, Compiler, params, wat2wasm, CallingFrame, Executor, ImportObjectBuilder, Module, Store, WasmValue, config::Config, Vm, AsInstance, CompilerOutputFormat, CompilerOptimizationLevel, WasmEdgeResult, WasmValTypeList, ImportObject};
use wasmedge_sdk::config::{ConfigBuilder, HostRegistrationConfigOptions, CommonConfigOptions, CompilerConfigOptions};
use wasmedge_sdk::WasmVal;
use wasmedge_sys::Memory;

use std::env;
use std::path::Path;
use std::thread;

use anyhow::anyhow;

// like docker mounts for WASI, which dirs to mount
const WASI_VOLUME_MOUNTS: &str = ".:.";

// name of prerenderer wasm module
const PRERENDERER_WASM_MODULE_NAME : &str = "prerenderer";

// function to be called as entrypoint
const PRERENDERER_WASM_MODULE_ENTRYPOINT : &str = "_start";

// todo: resolve this from parture folder helper path util
const WASM_BLOB_INTERPRETED_PATH : &str
= "../prerendering_wasm/dist/prerendering_wasm.wasm";

const WASM_BLOB_AOT_PATH : &str
= "../prerendering_wasm/dist/prerendering_wasm_aot.so";

const OPT_LEVEL: CompilerOptimizationLevel
= CompilerOptimizationLevel::O0;

// create vm config
pub fn config() -> Config {
    let common_options = CommonConfigOptions::default();

    let compiler_options = CompilerConfigOptions::default()
        .dump_ir(false)
        .generic_binary(true)
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

/// produce native optimized binary
fn compile_sdk() -> WasmEdgeResult<()> {
    println!("compiling AOT wasm binary through Rust SDK...");

    std::fs::create_dir_all("../prerendering_wasm/dist/")
        .expect("failed to make output dirs");

    Compiler::new(Some(&config()))?
        .compile_from_file(
            WASM_BLOB_INTERPRETED_PATH,
            WASM_BLOB_AOT_PATH)
}

// create VM for running prerenderer code
fn vm(file: &str) -> WasmEdgeResult<Vm> {
    let mut vm = Vm::new(Some(config()))?;

    // this is essential, without it the runtime cannot read files and will error out
    // https://github.com/WasmEdge/WasmEdge/issues/1872
    println!("configuring wasi...");
    let mut wasi_module = vm.wasi_module()?;
    wasi_module.initialize(None, None, Some(vec!(WASI_VOLUME_MOUNTS)));

    // our code
    let module = Module::from_file(Some(&crate::config()), file).unwrap();

    // dbg info
    for ex in &module.exports() {
        println!("export name={}, ty={:#?}", ex.name(), ex.ty().unwrap());
    }

    vm
        .register_module(Some(PRERENDERER_WASM_MODULE_NAME), module)
}

pub fn run_wasm(file: &str) -> anyhow::Result<()> {
    // create a vm with the config settings
    println!("creating vm...");
    let vm = vm(file)?;

    vm.run_func(
        Some(PRERENDERER_WASM_MODULE_NAME),
        PRERENDERER_WASM_MODULE_ENTRYPOINT,
        params!()
    )?;

    Ok(())
}

#[test]
fn test_interpreted() {
    let interpreted_res = run_wasm(
        crate::WASM_BLOB_INTERPRETED_PATH);

    assert!(interpreted_res.is_ok())
}

#[test]
fn test_aot() {
    compile_sdk().unwrap();

    let aot_res = run_wasm(
        crate::WASM_BLOB_AOT_PATH);

    assert!(aot_res.is_ok());
}