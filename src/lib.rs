//! Library to serialize and deserialize [Syn] syntax trees.
//!
//! ## Examples
//!
//! ```toml
//! [dependencies]
//! serde-syn = { version = "0.1.0-alpha.1", features = ["json"] }
//! syn = { version = "1", features = ["full"] }
//! ```
//!
//! ```rust
//! # #[cfg(feature = "json")]
//! # fn foo() -> serde_json::Result<()> {
//! use serde_syn::json;
//!
//! let syn_file: syn::File = syn::parse_quote! {
//!     fn main() {
//!         println!("Hello, world!");
//!     }
//! };
//! println!("{}", json::to_string_pretty(&syn_file)?);
//! # Ok(())
//! # }
//! ```
//!
//! This prints the following JSON:
//!
//! ```json
//! {
//!   "items": [
//!     {
//!       "fn": {
//!         "ident": "main",
//!         "inputs": [],
//!         "output": null,
//!         "stmts": [
//!           {
//!             "semi": {
//!               "macro": {
//!                 "path": {
//!                   "segments": [
//!                     {
//!                       "ident": "println"
//!                     }
//!                   ]
//!                 },
//!                 "delimiter": "paren",
//!                 "tokens": [
//!                   {
//!                     "lit": "\"Hello, world!\""
//!                   }
//!                 ]
//!               }
//!             }
//!           }
//!         ]
//!       }
//!     }
//!   ]
//! }
//! ```
//!
//! ### Rust source file -> JSON representation of the syntax tree
//!
//! The [`rust2json`] example parse a Rust source file into a `serde_syn::File`
//! and print out a JSON representation of the syntax tree.
//!
//! ### JSON file -> Rust syntax tree
//!
//! The [`json2rust`] example parse a JSON file into a `serde_syn::File` and
//! print out a Rust syntax tree.
//!
//! ## Optional features
//!
//! - **`json`** — Provides functions for JSON <-> Rust serializing and
//!   deserializing.
//!
//! [Syn]: https://github.com/dtolnay/syn
//! [`rust2json`]: https://github.com/taiki-e/serde-syn/tree/master/examples/rust2json
//! [`json2rust`]: https://github.com/taiki-e/serde-syn/tree/master/examples/json2rust

#![doc(html_root_url = "https://docs.rs/serde-syn/0.1.0")]
#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code)
    )
))]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms, unreachable_pub)]
// It cannot be included in the published code because these lints have false positives in the minimum required version.
#![cfg_attr(test, warn(single_use_lifetimes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::large_enum_variant,
    clippy::module_name_repetitions,
    clippy::shadow_unrelated,
    clippy::use_self,
    clippy::used_underscore_binding
)]

#[macro_use]
mod macros;

mod gen;

mod attr;
#[doc(hidden)]
pub use crate::attr::*;

mod data;
#[doc(hidden)]
pub use crate::data::*;

mod expr;
#[doc(hidden)]
pub use crate::expr::*;

mod generics;
#[doc(hidden)]
pub use crate::generics::*;

mod item;
#[doc(hidden)]
pub use crate::item::*;

mod file;
#[doc(hidden)]
pub use crate::file::File;

mod lifetime;
#[doc(hidden)]
pub use crate::lifetime::Lifetime;

mod lit;
#[doc(hidden)]
pub use crate::lit::*;

mod mac;
#[doc(hidden)]
pub use crate::mac::{Macro, MacroDelimiter};

mod op;
#[doc(hidden)]
pub use crate::op::{BinOp, UnOp};

mod ty;
#[doc(hidden)]
pub use crate::ty::*;

mod pat;
#[doc(hidden)]
pub use crate::pat::*;

mod path;
#[doc(hidden)]
pub use crate::path::*;

mod stmt;
#[doc(hidden)]
pub use crate::stmt::{Block, Local, Stmt};

mod token_stream;
#[doc(hidden)]
pub use crate::token_stream::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree,
};

#[cfg(feature = "json")]
pub mod json;

// =============================================================================
// Syn trait

mod private {
    pub trait Sealed {}
}

#[allow(single_use_lifetimes)] // https://github.com/rust-lang/rust/issues/55058
pub trait Syn: Sized + private::Sealed {
    type Adapter: Serialize + for<'de> Deserialize<'de>;

    #[doc(hidden)]
    fn to_adapter(&self) -> Self::Adapter;
    #[doc(hidden)]
    fn from_adapter(adapter: &Self::Adapter) -> Self;
}

// =============================================================================

use proc_macro2::Span;
use serde::{Deserialize, Serialize};

type Punctuated<T> = Vec<T>;

fn default<T>() -> T
where
    T: Default,
{
    T::default()
}

fn default_or_none<T>(x: bool) -> Option<T>
where
    T: Default,
{
    if x {
        Some(T::default())
    } else {
        None
    }
}

fn not<T>(x: T) -> T::Output
where
    T: std::ops::Not,
{
    !x
}

// https://github.com/rust-lang/rust/issues/51443
trait RefInto<U>: Sized {
    fn ref_into<'a>(&'a self) -> U
    where
        &'a Self: Into<U>,
    {
        self.into()
    }
}

impl<T, U> RefInto<U> for T {}

trait MapInto<U, M> {
    type T;

    fn ref_map<'a, F>(&'a self, f: F) -> M
    where
        Self::T: 'a,
        F: FnMut(&'a Self::T) -> U;

    fn map_into<'a>(&'a self) -> M
    where
        Self::T: 'a,
        &'a Self::T: Into<U>,
    {
        self.ref_map(Into::into)
    }
}

impl<T, U> MapInto<U, Vec<U>> for Vec<T> {
    type T = T;

    fn ref_map<'a, F>(&'a self, f: F) -> Vec<U>
    where
        F: FnMut(&'a Self::T) -> U,
    {
        self.iter().map(f).collect()
    }
}

impl<T, U, P> MapInto<U, syn::punctuated::Punctuated<U, P>> for Vec<T>
where
    P: Default,
{
    type T = T;

    fn ref_map<'a, F>(&'a self, f: F) -> syn::punctuated::Punctuated<U, P>
    where
        F: FnMut(&'a Self::T) -> U,
    {
        self.iter().map(f).collect()
    }
}

impl<T, U, P> MapInto<U, Vec<U>> for syn::punctuated::Punctuated<T, P>
where
    P: Default,
{
    type T = T;

    fn ref_map<'a, F>(&'a self, f: F) -> Vec<U>
    where
        F: FnMut(&'a Self::T) -> U,
    {
        self.iter().map(f).collect()
    }
}

impl<T, U> MapInto<U, Option<U>> for Option<T> {
    type T = T;

    fn ref_map<'a, F>(&'a self, f: F) -> Option<U>
    where
        F: FnMut(&'a Self::T) -> U,
    {
        self.as_ref().map(f)
    }
}

impl<T, U> MapInto<U, Box<U>> for Box<T> {
    type T = T;

    fn ref_map<'a, F>(&'a self, mut f: F) -> Box<U>
    where
        F: FnMut(&'a Self::T) -> U,
    {
        Box::new(f(&**self))
    }
}
