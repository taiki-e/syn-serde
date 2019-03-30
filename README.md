# Serde Syn

[![Build Status](https://travis-ci.com/taiki-e/serde-syn.svg?branch=master)](https://travis-ci.com/taiki-e/serde-syn)
[![version](https://img.shields.io/crates/v/serde_syn.svg)](https://crates.io/crates/serde_syn/)
[![documentation](https://docs.rs/serde_syn/badge.svg)](https://docs.rs/serde_syn/)
[![license](https://img.shields.io/crates/l/serde_syn.svg)](https://crates.io/crates/serde_syn/)
[![Rustc Version](https://img.shields.io/badge/rustc-1.31+-lightgray.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)

Library to serialize and deserialize [Syn] syntax trees.

The API of version 0.0.* is experimental and compatibility between patch versions is not guaranteed.

[**Documentation**](https://docs.rs/serde_syn/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
serde_syn = "0.0.1"
```

The current version of serde_syn requires Rust 1.31 or later.

## Examples

```toml
[dependencies]
serde_syn = { version = "0.0.1", features = ["json"] }
syn = { version = "0.15", features = ["full"] }
```

```rust
use serde_syn::json;

let syn_file: syn::File = syn::parse_quote! {
    fn main() {
        println!("Hello, world!");
    }
};

println!("{}", json::to_string_pretty(&syn_file)?);
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
                "tts": [
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

The [`rust2json`] example parse a Rust source file into a `serde_syn::File`
and print out a JSON representation of the syntax tree.

[`rust2json`]: examples/rust2json

### JSON file -> Rust syntax tree

The [`json2rust`] example parse a JSON file into a `serde_syn::File` and
print out a Rust syntax tree.

[`json2rust`]: examples/json2rust

## Optional features

- **`json`** — Provides functions for JSON <-> Rust serializing and
  deserializing.

## Relationship to Syn

Serde Syn is a fork of [Syn], and Serde Syn provides a set of data structures
similar but not identical to [Syn]. All data structures provided by Serde Syn
can be converted to the data structures of [Syn] and [proc-macro2].

The data structures of Serde Syn 0.1 is compatible with the data structures of [Syn] 0.15.

[Syn]: https://github.com/dtolnay/syn
[proc-macro2]: https://github.com/alexcrichton/proc-macro2

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
