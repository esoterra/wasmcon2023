# Wasm Components for Every Language (Demo)

This is the demo source code for the talk I gave titled "Wasm Components for Every Language" at WasmCon 2023.

> **Note:** the talks title isn't meant to imply that we already have fully built toolchains for every language rather that Wasm Components as a technology aren't limited to any one language and that we have techniques that can be applied to any language to componentize it.

You can find the talks abstract, slides, and recording on [Sched](https://wasmcon2023.sched.com/event/1PCLo/wasm-components-for-every-language-kyle-brown-singlestore).

I am not intending to keep this up to date with newer versions of the component model or guest language tooling, but if you run into any issues running the demo under the intended versions it was originally run with please feel free to file an issue.

## WITs

The WIT file defines one interface `%interface` and two worlds `greeter` and `proxy-greeter`.

* `%interface` is just a simple interface with a `greet` function that returns a string
* `greeter` is just a world that exports `%interface`
* `proxy-greeter` both imports and exports the `%interface` interface

## Components

Components for the demo are implemented in the WebAssembly Text Format (WAT), C, Rust, JavaScript, and Python.
All of the Components implement `proxy-greeter` except WAT which implements `greeter`.

The WIP Go implementation was never finished.

Each language folder has a README with instructions for building its source into a component. The resulting components are also included pre-built in the components folder for convenience.

## Composition

The demo shown in the talk uses the [Wasm Builder app tool](https://wasmbuilder.app/) which runs in the browser and can be used to compose arbitrary components together graphically. In the talk, I use a window where I have already uploaded each of the components included in the components folder.

The base WAT `greeter` can be composed with any number of `proxy-greeters` to create a final `greeter` whose message reads "Hello from WAT!" with "and [language]!" clauses matching the order you composed the `proxy-greeters`.

## Runner

The crate in the `runner` folder is used to run the resulting `greeter` printing out its message.

This is done by passing the path to the component with the `--component` or `-c` flag.

> **Note:** Build the runner using the `--release` flag if you want it to go faster!

It also has an alternate mode triggered by passing `--input` or `-i` and a string that will require the component to be a `proxy-greeter` and when the it calls its import it will pass the input value in. This was used to test each component during development but not shown in the demo.

## Tools Used

* [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen/)
* [wasm-tools](https://github.com/bytecodealliance/wasm-tools/)
* [WASI-Virt](https://github.com/bytecodealliance/WASI-Virt/)
* [WASI SDK](https://github.com/webassembly/wasi-sdk)
* [Cargo Component](https://github.com/bytecodealliance/cargo-component)
* [Componentize-Js](https://github.com/bytecodealliance/componentize-js/)
* [JCO](https://github.com/bytecodealliance/jco/)
* [Componentize-Py](https://github.com/bytecodealliance/componentize-py/)
