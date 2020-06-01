// Based on https://github.com/dtolnay/syn/tree/1.0.5/codegen.
//
// This crate generates the Syn trait in syn-serde programmatically from
// the syntax tree description.

#![recursion_limit = "128"]
#![warn(rust_2018_idioms, single_use_lifetimes, unreachable_pub)]

const SYN_JSON: &str = "../syn.json";

mod convert;
mod file;
mod gen;

use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let defs = fs::read_to_string(SYN_JSON)?;
    let defs = serde_json::from_str(&defs)?;

    convert::generate(&defs)?;
    Ok(())
}
