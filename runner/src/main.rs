use std::{fs, path::PathBuf, sync::Arc};

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

        world greet {
            export greet: func() -> string
        }
    ",

});

fn main() -> Result {
    let args = Args::parse();

    let binary = fs::read(args.input)?;

    

    Ok(())
}
