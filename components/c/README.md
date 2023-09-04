# C

```
# Generates bindings header, source, and object files
wit-bindgen c ../../wit/greeter.wit --world proxy-greeter

/opt/wasi-sdk/bin/clang \
    proxy_greeter.c component.c \
    proxy_greeter_component_type.o \
    -o greet-module.c.wasm -mexec-model=reactor

wasm-tools component new greet-module.c.wasm -o ../greet.c.wasm
```