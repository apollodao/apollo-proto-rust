## apollo-proto-rust

### Description
This repo contains Rust types generated from Protobuf types, for use in interacting with custom Cosmos SDK modules from WASM.

### Usage
1. uncomment `[build-dependencies]` in `Cargo.toml` and what's in `build.rs`
2. run `sudo make gen`
3. re-comment out what was uncommented in (1) in order to build for prod