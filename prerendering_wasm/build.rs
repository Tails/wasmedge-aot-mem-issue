use std::env;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

/// build the input WASM (interpreted)
fn build_js() {
    println!("building standard wasm binary...");

    let res = Command::new("make")
        .arg("compile-js-dev")
        .output()
        .expect("Failed to execute command");

    if !res.status.success() {
        panic!("{}", String::from_utf8(res.stderr).unwrap());
    }
}

pub fn main() {
    // build_js();
}