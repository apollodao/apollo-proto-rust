## apollo-proto-rust

### Description
This repo contains Rust types generated from Protobuf types, for use in interacting with custom Cosmos SDK modules from WASM.

### Usage
1. uncomment `[build-dependencies]` in `Cargo.toml` and what's in `build.rs`
2. add proto files in same directory structure as the package name of the proto you are adding, and make them visible with mod.rs files
3. add paths to protos in `build.rs`
4. run `sudo make gen`
5. re-comment out what was uncommented in (1) in order to build for prod