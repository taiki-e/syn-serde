// Based on https://github.com/dtolnay/syn/tree/1.0.5/codegen.
//
// This crate generates the Syn trait in syn-serde programmatically from
// the syntax tree description.

#![warn(rust_2018_idioms, single_use_lifetimes)]

mod ast_enum;
mod ast_struct;
mod convert;
mod file;
mod gen;

use std::path::Path;

use anyhow::Result;

fn main() -> Result<()> {
    let syn_json = Path::new(env!("CARGO_MANIFEST_DIR")).join("syn.json");
    let defs = fs::read_to_string(syn_json)?;
    let defs = serde_json::from_str(&defs)?;

    ast_struct::generate(&defs)?;
    ast_enum::generate(&defs)?;
    convert::generate(&defs)?;
    Ok(())
}
