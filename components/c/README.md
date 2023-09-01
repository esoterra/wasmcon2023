# C

```
/opt/wasi-sdk/bin/clang \
    greeter_adapter.c component.c \
    critter_component_type.o \
    -o critter.wasm -mexec-model=reactor \

wasm-tools component new critter.wasm -o critter.component.wasm
```