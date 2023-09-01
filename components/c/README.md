# C

```
/opt/wasi-sdk/bin/clang \
    greeter_adapter.c component.c \
    greeter_adapter_component_type.o \
    -o greet-module.c.wasm -mexec-model=reactor

wasm-tools component new greet-module.c.wasm -o ../greet.c.wasm
```