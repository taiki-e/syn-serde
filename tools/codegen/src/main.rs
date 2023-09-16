// SPDX-License-Identifier: Apache-2.0 OR MIT

// Based on https://github.com/dtolnay/syn/tree/1.0.5/codegen.
//
// This crate generates the Syn trait in syn-serde programmatically from
// the syntax tree description.

#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(clippy::match_on_vec_items, clippy::needless_pass_by_value)]

#[macro_use]
mod file;

mod ast_enum;
mod ast_struct;
mod convert;
mod gen;

use std::path::Path;

use anyhow::Result;
use fs_err as fs;

fn main() -> Result<()> {
    // TODO: auto-update syn.json on new release?
    let syn_json = Path::new(env!("CARGO_MANIFEST_DIR")).join("syn.json");
    let defs = fs::read_to_string(syn_json)?;
    let defs = serde_json::from_str(&defs)?;

    ast_struct::generate(&defs)?;
    ast_enum::generate(&defs)?;
    convert::generate(&defs)?;
    Ok(())
}
