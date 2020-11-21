# syn-serde

[![crates.io](https://img.shields.io/crates/v/syn-serde.svg?style=flat-square&logo=rust)](https://crates.io/crates/syn-serde)
[![docs.rs](https://img.shields.io/badge/docs.rs-syn--serde-blue?style=flat-square)](https://docs.rs/syn-serde)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg?style=flat-square)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.31+-blue.svg?style=flat-square)](https://www.rust-lang.org)
[![build status](https://img.shields.io/github/workflow/status/taiki-e/syn-serde/CI/master?style=flat-square)](https://github.com/taiki-e/syn-serde/actions?query=workflow%3ACI+branch%3Amaster)

Library to serialize and deserialize [Syn] syntax trees.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
syn-serde = "0.2"
```

*Compiler support: requires rustc 1.31+*

## Examples

```toml
[dependencies]
syn-serde = { version = "0.2", features = ["json"] }
syn = { version = "1", features = ["full"] }
```

```rust
use syn_serde::json;

let syn_file: syn::File = syn::parse_quote! {
    fn main() {
        println!("Hello, world!");
    }
};

println!("{}", json::to_string_pretty(&syn_file));
```

This prints the following JSON:

```json
{
  "items": [
    {
      "fn": {
        "ident": "main",
        "inputs": [],
        "output": null,
        "stmts": [
          {
            "semi": {
              "macro": {
                "path": {
                  "segments": [
                    {
                      "ident": "println"
                    }
                  ]
                },
                "delimiter": "paren",
                "tokens": [
                  {
                    "lit": "\"Hello, world!\""
                  }
                ]
              }
            }
          }
        ]
      }
    }
  ]
}
```

### Rust source file -> JSON representation of the syntax tree

The [`rust2json`] example parse a Rust source file into a `syn_serde::File`
and print out a JSON representation of the syntax tree.

[`rust2json`]: examples/rust2json

### JSON file -> Rust syntax tree

The [`json2rust`] example parse a JSON file into a `syn_serde::File` and
print out a Rust syntax tree.

[`json2rust`]: examples/json2rust

## Optional features

* **`json`** — Provides functions for JSON <-> Rust serializing and
  deserializing.

## Relationship to Syn

syn-serde is a fork of [Syn], and syn-serde provides a set of data structures
similar but not identical to [Syn]. All data structures provided by syn-serde
can be converted to the data structures of [Syn] and [proc-macro2].

The data structures of syn-serde 0.2 is compatible with the data structures of [Syn] 1.0.

[Syn]: https://github.com/dtolnay/syn
[proc-macro2]: https://github.com/alexcrichton/proc-macro2

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
