// If the version of rust used is less than v1.63, please uncomment the follow attribute.
#![feature(explicit_generic_args_with_impl_trait)]
#![feature(never_type)]

mod run;

pub const WASM_BLOB_INTERPRETED_PATH: &'static str
    = "../prerendering_wasm/dist/prerendering_wasm.wasm";

pub const WASM_BLOB_AOT_PATH: &'static str
    = "../prerendering_wasm/dist/prerendering_wasm_aot.so";

#[tokio::test]
async fn test_interpreted() {
    let interpreted_res = crate::run::run_wasm(
        crate::WASM_BLOB_INTERPRETED_PATH).await;
}

#[tokio::test]
async fn test_aot() {
    let aot_res = crate::run::run_wasm(
        crate::WASM_BLOB_AOT_PATH).await;
}