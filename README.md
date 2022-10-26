# WasmEdge AOT Memory out of bounds error
A WasmEdge application where a Rust Host runs a Rust-WASM Guest application that runs Javascript using Wasmedge-QuickJS.

Run ```make run``` to run tests. It will build and run a custom Docker container with Rust, 
Wasmedge and some dependencies needed for font and canvas rendering. 
(Actual canvas drawing operations) were removed from the JS and Rust Host.

It runs the app in interpreter mode, loading the prerenderer_wasm.wasm file, and then in AOT mode using the prerenderer_wasm_aot.so file.
The AOT test will fail with the following error:

>  [error] execution failed: out of bounds memory access, Code: 0x88
| [error]     When executing module name: "prerenderer" , function name: "_start"