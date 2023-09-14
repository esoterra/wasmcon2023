# Python

Requirements:
* `componentize-py` - `pip install componentize-py==0.4.2`
* `wasi-virt` - `cargo install --git https://github.com/bytecodealliance/wasi-virt --rev c485962`

```
componentize-py \
    --wit-path ../../wit/greeter.wit \
    --world proxy-greeter \
    componentize \
    greet \
    -o ./greet-wasi.py.wasm

wasi-virt ./greet-wasi.py.wasm -o ../greet.py.wasm
```