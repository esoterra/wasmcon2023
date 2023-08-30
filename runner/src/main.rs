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
    input: PathBuf,
}

use wasmtime_component_macro::bindgen;
bindgen!({
    inline: "
        package wasmcon2023:greet

        world greeter {
            export greet: func() -> string
        }
    ",

});

fn main() -> Result<()> {
    let args = Args::parse();

    let component_bytes = fs::read(args.input)?;

    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    let component = Component::new(&engine, component_bytes)?;

    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    let (greeter, _) = Greeter::instantiate(&mut store, &component, &linker)?;

    println!("{}", greeter.call_greet(&mut store)?);

    Ok(())
}
