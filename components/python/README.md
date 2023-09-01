# Python

```
pip install componentize-py
componentize-py \
    --wit-path ../../wit/greeter.wit \
    --world greeter-adapter \
    componentize \
    greet \
    -o ../greet-wasi.py.wasm

wasi-virt ../greet-wasi.py.wasm -o ../greet.py.wasm
```