# Rust

Requirements
* Cargo Component - `cargo install --git https://github.com/bytecodealliance/cargo-component --rev 36c221e --locked cargo-component`

```
cargo component build --release --target wasm32-unknown-unknown

cp ./target/wasm32-unknown-unknown/release/rust.wasm ../greet.rs.wasm
```