use wasmedge_sdk::{error::HostFuncError, params, wat2wasm, CallingFrame, Executor, ImportObjectBuilder, Module, Store, WasmValue, config::Config, Vm, AsInstance, CompilerOutputFormat, CompilerOptimizationLevel, WasmEdgeResult, WasmValTypeList, ImportObject};
use wasmedge_sdk::config::{ConfigBuilder, HostRegistrationConfigOptions, CommonConfigOptions, CompilerConfigOptions};
use wasmedge_sdk::WasmVal;
use wasmedge_sys::Memory;

use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender, UnboundedReceiver, UnboundedSender};
use std::thread;
use anyhow::anyhow;

// name of the command sent over the mspc channel to make it close
const CHANNEL_CLOSE_CMD: &str = "quit";

// name of the module for Rust SDK imports as it will appear in the prerenderer_wasm
const IMPORT_OBJECT_MODULE_NAME: &str = "env";

// like docker mounts for WASI, which dirs to mount
const WASI_VOLUME_MOUNTS: &str = ".:.";

// name of prerenderer wasm module
const PRERENDERER_WASM_MODULE_NAME : &str = "prerenderer";

// function to be called as entrypoint
const PRERENDERER_WASM_MODULE_ENTRYPOINT : &str = "_start";

#[derive(Clone, Debug)]
pub enum CanvasRPC {
    // todo: this should be taken into the tx<>, not here
    Quit,
}

// create vm config
fn config() -> Config {
    let common_options = CommonConfigOptions::default();

    let compiler_options = CompilerConfigOptions::default();

    let host_reg_options = HostRegistrationConfigOptions::new()
        .wasi(true);

    let result = ConfigBuilder::new(common_options)
        .with_host_registration_config(host_reg_options)
        .with_compiler_config(compiler_options)
        .build();

    assert!(result.is_ok());
    result.unwrap()
}

// read string value from wasm memory using pointer
fn read_string(mem_ref: wasmedge_sys::instance::memory::Memory, offset: &WasmValue, len: &WasmValue) -> anyhow::Result<String> {
    println!("trying to read string of length {} from WASM memory...", len.to_i32());
    let str_pointer = offset.to_i32();
    let str_len = len.to_i32();
    let str_bytes = mem_ref.get_data(str_pointer as u32, str_len as u32)?;
    println!("read bytes: {:#?}", &str_bytes);
    Ok(String::from_utf8(str_bytes)?)
}

fn parse_cmd(cmd: String, args: Vec<WasmValue>, mem: Option<Memory>) -> anyhow::Result<CanvasRPC> {
    Ok(match cmd.as_str() {
        // command to close channel (or it hangs)
        // todo: dont parse into rpc enum but pass through channel
        CHANNEL_CLOSE_CMD => CanvasRPC::Quit,

        // ... left out variants

        // if we forgot something
        _ => panic!("unrecognized command! {}", cmd)
    })
}

// create VM for running prerenderer code
fn vm(file: &str, tx: UnboundedSender<CanvasRPC>) -> WasmEdgeResult<Vm> {
    let mut vm = Vm::new(Some(config()))?;

    // this is essential, without it the runtime cannot read files and will error out
    // https://github.com/WasmEdge/WasmEdge/issues/1872
    println!("configuring wasi...");
    let mut wasi_module = vm.wasi_module()?;
    wasi_module.initialize(None, None, Some(vec!(WASI_VOLUME_MOUNTS)));

    // our code
    let module = Module::from_file(Some(&config()), file).unwrap();

    // dbg info
    for ex in &module.exports() {
        println!("export name={}, ty={:#?}", ex.name(), ex.ty().unwrap());
    }

    vm
        // .register_import_module(import_object(tx)?)?
        .register_module(Some(PRERENDERER_WASM_MODULE_NAME), module)
}

pub async fn run_wasm(file: &str) -> anyhow::Result<()> {
    // open channels for receiving draw commands from callbacks
    let (tx, mut rx) : (UnboundedSender<CanvasRPC>, UnboundedReceiver<CanvasRPC>)
        = tokio::sync::mpsc::unbounded_channel();

    // create a vm with the config settings
    println!("creating vm...");
    let vm = vm(file, tx.clone())?;

    vm.run_func(
        Some(PRERENDERER_WASM_MODULE_NAME),
        PRERENDERER_WASM_MODULE_ENTRYPOINT,
        params!()
    )?;

    Ok(())
}