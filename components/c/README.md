# C

Requirements
* `wasi-sdk` - can be installed using the `install-wasi-sdk.sh` script in this repo for Linux.
* `wit-bindgen` - `cargo install --git https://github.com/bytecodealliance/wit-bindgen --tag wit-bindgen-0.11.0 wit-bindgen-cli` 
* `wasm-tools` - `cargo install wasm-tools --version 1.0.40`

```
# Generates bindings header, source, and object files
wit-bindgen c ../../wit/greeter.wit --world proxy-greeter

/opt/wasi-sdk/bin/clang \
    proxy_greeter.c component.c \
    proxy_greeter_component_type.o \
    -o greet-module.c.wasm -mexec-model=reactor

wasm-tools component new greet-module.c.wasm -o ../greet.c.wasm
```