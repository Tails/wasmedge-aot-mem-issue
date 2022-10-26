# WasmEdge AOT Memory out of bounds error
A WasmEdge application where a Rust Host runs a Rust-WASM Guest application that runs Javascript using Wasmedge-QuickJS.

Run ```make run``` to run tests. It will build and run a custom Docker container with Rust, 
Wasmedge and some dependencies needed for font and canvas rendering. 
(Actual canvas drawing operations) were removed from the JS and Rust Host.