use wasmedge_sdk::{error::HostFuncError, params, wat2wasm, CallingFrame, Executor, ImportObjectBuilder, Module, Store, WasmValue, config::Config, Vm, AsInstance, CompilerOutputFormat, CompilerOptimizationLevel, WasmEdgeResult, WasmValTypeList, ImportObject, Compiler};
use wasmedge_sdk::config::{ConfigBuilder, HostRegistrationConfigOptions, CommonConfigOptions, CompilerConfigOptions};
use wasmedge_sdk::WasmVal;
use wasmedge_sys::Memory;

use std::env;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::Result;

/// build the input WASM (interpreted)
fn build_wasm() -> anyhow::Result<()> {
    println!("building standard wasm binary...");

    // tell cargo to invalidate build cache if the _wasm package changes
    println!("cargo:rerun-if-changed=../prerendering_wasm/src/main.rs");
    println!("cargo:rerun-if-changed=../prerendering_wasm/Cargo.toml");
    println!("cargo:rerun-if-changed=../prerendering_wasm/build.rs");
    println!("cargo:rerun-if-changed=../prerendering_wasm/dist/*");

    let res = Command::new("make")
        .current_dir("../prerendering_wasm")
        .arg("build")
        .output()?;

    if !res.status.success() {
        panic!("{}", String::from_utf8(res.stderr).unwrap());
    }

    Ok(())
}

pub fn main() {
    build_wasm().unwrap();
}