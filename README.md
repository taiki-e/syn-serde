# syn-serde

[![Build Status][azure-badge]][azure-url]
[![Crates.io][crates-version-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![License][crates-license-badge]][crates-url]
[![Minimum supported Rust version][rustc-badge]][rustc-url]

[azure-badge]: https://dev.azure.com/taiki-e/taiki-e/_apis/build/status/taiki-e.syn-serde?branchName=master
[azure-url]: https://dev.azure.com/taiki-e/taiki-e/_build/latest?definitionId=16&branchName=master
[crates-version-badge]: https://img.shields.io/crates/v/syn-serde.svg
[crates-license-badge]: https://img.shields.io/crates/l/syn-serde.svg
[crates-badge]: https://img.shields.io/crates/v/syn-serde.svg
[crates-url]: https://crates.io/crates/syn-serde/
[docs-badge]: https://docs.rs/syn-serde/badge.svg
[docs-url]: https://docs.rs/syn-serde/
[rustc-badge]: https://img.shields.io/badge/rustc-1.31+-lightgray.svg
[rustc-url]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html

Library to serialize and deserialize [Syn] syntax trees.

[**Documentation**][docs-url]

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
syn-serde = "0.1.0-alpha.1"
```

The current syn-serde requires Rust 1.31 or later.

## Examples

```toml
[dependencies]
syn-serde = { version = "0.1.0-alpha.1", features = ["json"] }
syn = { version = "1", features = ["full"] }
```

```rust
use syn_serde::json;

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

The data structures of syn-serde 0.1 is compatible with the data structures of [Syn] 1.0.

[Syn]: https://github.com/dtolnay/syn
[proc-macro2]: https://github.com/alexcrichton/proc-macro2

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
