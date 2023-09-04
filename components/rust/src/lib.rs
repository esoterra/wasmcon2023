cargo_component_bindings::generate!();

use bindings::{
    wasmcon2023::greet::interface as import,
    exports::wasmcon2023::greet::interface::Guest
};

struct Component;

impl Guest for Component {
    fn greet() -> String {
        import::greet() + " and Rust 🦀!"
    }
}
