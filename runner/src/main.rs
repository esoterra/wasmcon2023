use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    component: PathBuf,

    #[arg(short, long)]
    input: Option<String>
}

fn main() -> Result<()> {
    let args = Args::parse();

    let component_bytes = fs::read(args.component)?;

    match args.input {
        Some(input) => ga::exec(component_bytes, input),
        None => g::exec(component_bytes),
    }
}

mod g {
    use super::*;
    use wasmtime::component::bindgen;

    bindgen!("greeter" in "../wit/greeter.wit");

    pub fn exec(component_bytes: Vec<u8>) -> Result<()> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config)?;

        let component = Component::new(&engine, component_bytes)?;

        let linker = Linker::new(&engine);
        let mut store = Store::new(&engine, ());
        let (greeter, _) = Greeter::instantiate(&mut store, &component, &linker)?;

        println!("{}", greeter.interface0.call_greet(&mut store)?);
        Ok(())
    }
}

mod ga {
    use super::*;
    use anyhow::Ok;
    use wasmtime::component::bindgen;

    bindgen!("proxy-greeter" in "../wit/greeter.wit");

    struct InputState {
        input: String
    }

    impl wasmcon2023::greet::interface::Host for InputState {
        fn greet(&mut self,) -> wasmtime::Result<String> {
            Ok(self.input.clone())
        }
    }

    pub fn exec(component_bytes: Vec<u8>, input: String) -> Result<()> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config)?;

        let component = Component::new(&engine, component_bytes)?;

        let mut linker = Linker::new(&engine);
        ProxyGreeter::add_to_linker(&mut linker, |state: &mut InputState| state)?;

        let mut store = Store::new(&engine, InputState { input });
        let (greeter, _) = ProxyGreeter::instantiate(&mut store, &component, &linker)?;

        println!("{}", greeter.interface0.call_greet(&mut store)?);
        Ok(())
    }
}
