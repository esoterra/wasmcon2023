# Python

```
pip install componentize-py
componentize-py \
    --wit-path ../../wit/greeter.wit \
    --world proxy-greeter \
    componentize \
    greet \
    -o ./greet-wasi.py.wasm

wasi-virt ./greet-wasi.py.wasm -o ../greet.py.wasm
```