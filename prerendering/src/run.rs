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
use crate::{CanvasRPC};

use crate::HtmlCanvasApi;

// the initial canvas height before we clip it
const PRE_CLIP_CANVAS_HEIGHT : u32 = 600;

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

// command name for setting canvas size
const CMD_RENDER_WIDTH : &str = "set_render_width";

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
        // .dump_ir(true)
        // .generic_binary(true)
        // .interruptible(true)
        // .optimization_level(CompilerOptimizationLevel::O3)
        // .out_format(CompilerOutputFormat::Native)
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
    macro_rules! single_string {
        ($fn:ident) => {
            {let stringarg = read_string(
                mem.expect("memory should have been retrieved!"),
                &args[0],
                &args[1]
            )?;

            println!("read {} string arg from memory: {}", stringify!($fn), &stringarg);

            stringarg.to_string()}
        }
    };
    Ok(match cmd.as_str() {
        // command to close channel (or it hangs)
        // todo: dont parse into rpc enum but pass through channel
        CHANNEL_CLOSE_CMD => CanvasRPC::Quit,

        // init canvas
        CMD_RENDER_WIDTH => CanvasRPC::Init(args.first().ok_or(anyhow!("did not receive render width arg"))?.to_i64() as u32),

        // todo: somehow generate these bindings through def_fn_callback!() ?
        // todo: translate() not used?

        // no arg functions
        "save" => CanvasRPC::Save,
        "restore" => CanvasRPC::Restore,
        "begin_path" => CanvasRPC::BeginPath,
        "close_path" => CanvasRPC::ClosePath,
        "stroke" => CanvasRPC::Stroke,
        "fill" => CanvasRPC::Fill,

        // x,y point args
        "scale" => CanvasRPC::Scale(args[0].to_f64(), args[1].to_f64()),
        "resize" => CanvasRPC::Resize(args[0].to_f64(), args[1].to_f64()),
        "move_to" => CanvasRPC::MoveTo(args[0].to_f64(), args[1].to_f64()),
        "line_to" => CanvasRPC::LineTo(args[0].to_f64(), args[1].to_f64()),

        // f64 float arg
        "shadow_blur" => CanvasRPC::ShadowBlur(args[0].to_f64()),
        "line_width" => CanvasRPC::LineWidth(args[0].to_f64()),

        // x, y, w, h
        "fill_rect" => CanvasRPC::FillRect(args[0].to_f64(), args[1].to_f64(), args[2].to_f64(), args[3].to_f64()),
        "rect" => CanvasRPC::Rect(args[0].to_f64(), args[1].to_f64(), args[2].to_f64(), args[3].to_f64()),
        "clear_rect" => CanvasRPC::ClearRect(args[0].to_f64(), args[1].to_f64(), args[2].to_f64(), args[3].to_f64()),

        // (f64, f64, f64, f64, f64, u8)
        "arc" => CanvasRPC::Arc(args[0].to_f64(), args[1].to_f64(), args[2].to_f64(), args[3].to_f64(), args[4].to_f64(), args[5].to_i32() != 0),

        "bezier_curve_to" => CanvasRPC::BezierCurveTo(args[0].to_f64(), args[1].to_f64(), args[2].to_f64(), args[3].to_f64(), args[4].to_f64(), args[5].to_f64()),
        "quadratic_curve_to" => CanvasRPC::QuadraticCurveTo(args[0].to_f64(), args[1].to_f64(), args[2].to_f64(), args[3].to_f64()),

        // (i32, u8, f64, f64)
        "fill_text" => {
            let stringarg = read_string(
                mem.expect("memory should have been retrieved!"),
                &args[0],
                &args[1]
            )?;

            println!("read fill_text string arg from memory: {}", &stringarg);

            CanvasRPC::FillText(stringarg, args[2].to_f64(), args[3].to_f64())
        },

        // (i32, u8)
        "fill_style" => CanvasRPC::FillStyle(single_string!(fill_style)),
        "background_fill_style" => CanvasRPC::BackgroundFillStyle(single_string!(background_fill_style)),
        "stroke_style" => CanvasRPC::StrokeStyle(single_string!(stroke_style)),
        "font" => CanvasRPC::Font(single_string!(font)),
        "shadow_color" => CanvasRPC::ShadowColor(single_string!(shadow_color)),
        "line_cap" => CanvasRPC::LineCap(single_string!(line_cap)),
        "line_dash" => {
            todo!();
            CanvasRPC::LineDash(vec!())
        },

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

    // todo: pass path or json of input score here
    println!("running prerenderer...");
    let handle = thread::spawn(move|| {
        let results = vm.run_func(
            Some(PRERENDERER_WASM_MODULE_NAME),
            PRERENDERER_WASM_MODULE_ENTRYPOINT,
            params!()
        );

        assert!(results.is_ok());

        // just to make sure the original thread gets to opening the tx
        std::thread::sleep(
            std::time::Duration::from_millis(2000));

        // sadly we have to send an inelegant closing command
        // because we cannot close from the sending side
        tx
            .send(CanvasRPC::Quit)
            .expect("failed to send loop quit cmd");
    });

    println!("starting draw command listener loop...");
    loop {
        if let Some(cmd) = rx.recv().await {
            println!("received command: {:?}", &cmd);

            if let CanvasRPC::Quit = &cmd {
                break;
            }
        }

        // empty buffer, close stream
        else {
            println!("draw cmd stream ended, all senders dropped or stream empty");
            break;
        }
    }

    handle.join().unwrap();

    Ok(())
}