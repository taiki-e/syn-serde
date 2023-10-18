// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
<!-- tidy:crate-doc:start -->
Library to serialize and deserialize [Syn] syntax trees.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
syn-serde = "0.3"
```

*Compiler support: requires rustc 1.56+*

## Examples

```toml
[dependencies]
syn-serde = { version = "0.3", features = ["json"] }
syn = { version = "2", features = ["full"] }
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

### JSON file -> Rust syntax tree

The [`json2rust`] example parse a JSON file into a `syn_serde::File` and
print out a Rust syntax tree.

## Optional features

- **`json`** — Provides functions for JSON <-> Rust serializing and
  deserializing.

## Relationship to Syn

syn-serde is a fork of [Syn], and syn-serde provides a set of data structures
similar but not identical to [Syn]. All data structures provided by syn-serde
can be converted to the data structures of [Syn] and [proc-macro2].

The data structures of syn-serde 0.3 is compatible with the data structures of
[Syn] 2.x.

[Syn]: https://github.com/dtolnay/syn
[proc-macro2]: https://github.com/alexcrichton/proc-macro2
[`rust2json`]: https://github.com/taiki-e/syn-serde/tree/HEAD/examples/rust2json
[`json2rust`]: https://github.com/taiki-e/syn-serde/tree/HEAD/examples/json2rust

<!-- tidy:crate-doc:end -->
*/

#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![forbid(unsafe_code)]
#![warn(
    rust_2018_idioms,
    single_use_lifetimes,
    unreachable_pub,
    clippy::pedantic,
    // Lints that may help when writing public library.
    // missing_debug_implementations,
    // missing_docs,
    clippy::alloc_instead_of_core,
    // clippy::exhaustive_enums, // TODO
    // clippy::exhaustive_structs, // TODO
    clippy::impl_trait_in_params,
    // clippy::missing_inline_in_public_items,
    // clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
)]
#![allow(
    clippy::enum_glob_use,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::needless_doctest_main,
    clippy::struct_excessive_bools,
    clippy::used_underscore_binding,
    clippy::wildcard_imports
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod macros;

#[path = "gen/ast_struct.rs"]
mod ast_struct;

#[path = "gen/ast_enum.rs"]
mod ast_enum;

#[path = "gen/convert.rs"]
mod convert;

mod attr {
    #[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
    pub use crate::{
        ast_enum::{AttrStyle, Meta},
        ast_struct::{Attribute, MetaList, MetaNameValue},
    };
}
#[doc(hidden)]
pub use crate::attr::{AttrStyle, Attribute, Meta, MetaList, MetaNameValue};

mod data;
pub(crate) use crate::data::assert_struct_semi;
#[doc(hidden)]
pub use crate::data::{Field, Fields, FieldsNamed, FieldsUnnamed, Variant};

mod expr;
#[doc(hidden)]
pub use crate::expr::{
    Arm, Expr, ExprArray, ExprAssign, ExprAsync, ExprAwait, ExprBinary, ExprBlock, ExprBreak,
    ExprCall, ExprCast, ExprClosure, ExprConst, ExprContinue, ExprField, ExprForLoop, ExprGroup,
    ExprIf, ExprIndex, ExprInfer, ExprLet, ExprLit, ExprLoop, ExprMacro, ExprMatch, ExprMethodCall,
    ExprParen, ExprPath, ExprRange, ExprReference, ExprRepeat, ExprReturn, ExprStruct, ExprTry,
    ExprTryBlock, ExprTuple, ExprUnary, ExprUnsafe, ExprWhile, ExprYield, FieldValue, Index, Label,
    Member, RangeLimits,
};

mod file {
    #[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
    pub use crate::ast_struct::File;
}
#[doc(hidden)]
pub use crate::file::File;

mod generics;
#[doc(hidden)]
pub use crate::generics::{
    BoundLifetimes, ConstParam, GenericParam, Generics, LifetimeParam, PredicateLifetime,
    PredicateType, TraitBound, TraitBoundModifier, TypeParam, TypeParamBound, WhereClause,
    WherePredicate,
};

mod item;
#[doc(hidden)]
pub use crate::item::{
    FnArg, ForeignItem, ForeignItemFn, ForeignItemMacro, ForeignItemStatic, ForeignItemType,
    ImplItem, ImplItemConst, ImplItemFn, ImplItemMacro, ImplItemType, ImplRestriction, Item,
    ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemForeignMod, ItemImpl, ItemMacro, ItemMod,
    ItemStatic, ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, ItemUse, Receiver,
    Signature, StaticMutability, TraitItem, TraitItemConst, TraitItemFn, TraitItemMacro,
    TraitItemType, UseGroup, UseName, UsePath, UseRename, UseTree, Variadic,
};

mod lifetime {
    #[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
    pub use crate::ast_struct::Lifetime;
}
#[doc(hidden)]
pub use crate::lifetime::Lifetime;

mod lit;
#[doc(hidden)]
pub use crate::lit::{
    Lit, LitBool, LitByte, LitByteStr, LitChar, LitFloat, LitInt, LitStr, StrStyle,
};

mod mac {
    #[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
    pub use crate::{ast_enum::MacroDelimiter, ast_struct::Macro};
}
#[doc(hidden)]
pub use crate::mac::{Macro, MacroDelimiter};

mod op {
    #[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
    pub use crate::ast_enum::{BinOp, UnOp};
}
#[doc(hidden)]
pub use crate::op::{BinOp, UnOp};

mod pat;
#[doc(hidden)]
pub use crate::expr::{
    ExprConst as PatConst, ExprLit as PatLit, ExprMacro as PatMacro, ExprPath as PatPath,
    ExprRange as PatRange,
};
#[doc(hidden)]
pub use crate::pat::{
    FieldPat, Pat, PatIdent, PatOr, PatParen, PatReference, PatRest, PatSlice, PatStruct, PatTuple,
    PatTupleStruct, PatType, PatWild,
};

mod path;
#[doc(hidden)]
pub use crate::path::{
    AngleBracketedGenericArguments, AssocConst, AssocType, Constraint, GenericArgument,
    ParenthesizedGenericArguments, Path, PathArguments, PathSegment, QSelf,
};

mod restriction;
#[doc(hidden)]
pub use crate::restriction::{FieldMutability, VisRestricted, Visibility};

mod stmt;
#[doc(hidden)]
pub use crate::stmt::{Block, Local, LocalInit, Stmt, StmtMacro};

mod ty;
#[doc(hidden)]
pub use crate::ty::{
    Abi, BareFnArg, BareVariadic, ReturnType, Type, TypeArray, TypeBareFn, TypeGroup,
    TypeImplTrait, TypeMacro, TypeParen, TypePath, TypePtr, TypeReference, TypeSlice,
    TypeTraitObject, TypeTuple,
};

mod token_stream;
#[doc(hidden)]
pub use crate::token_stream::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree,
};

#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub mod json;

mod sealed {
    pub trait Sealed {}
}

// =============================================================================
// Syn trait

/// A trait for the data structures of [Syn] and [proc-macro2].
///
/// [Syn]: https://github.com/dtolnay/syn
/// [proc-macro2]: https://github.com/alexcrichton/proc-macro2
#[allow(single_use_lifetimes)] // https://github.com/rust-lang/rust/issues/55058
pub trait Syn: Sized + sealed::Sealed {
    type Adapter: Serialize + for<'de> Deserialize<'de>;

    /// Converts a `Syn` type into an adapter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "json")]
    /// # fn dox() {
    /// use syn_serde::Syn;
    ///
    /// let syn_file: syn::File = syn::parse_quote! {
    ///     fn main() {
    ///         println!("Hello, world!");
    ///     }
    /// };
    ///
    /// let serializable_file = syn_file.to_adapter();
    /// println!("{}", serde_json::to_string_pretty(&serializable_file).unwrap());
    /// # }
    /// ```
    fn to_adapter(&self) -> Self::Adapter;

    /// Converts an adapter into a `Syn` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "json")]
    /// # fn dox() -> Result<(), Box<dyn std::error::Error>> {
    /// use syn_serde::Syn;
    ///
    /// // `struct Unit;`
    /// let json = r#"{
    ///   "struct": {
    ///     "ident": "Unit",
    ///     "fields": "unit"
    ///   }
    /// }"#;
    ///
    /// let serializable_file: <syn::File as Syn>::Adapter = serde_json::from_str(json)?;
    /// let syn_file = syn::File::from_adapter(&serializable_file);
    /// # Ok(())
    /// # }
    /// ```
    fn from_adapter(adapter: &Self::Adapter) -> Self;
}

// =============================================================================

use core::ops;

use proc_macro2::Span;
use serde::{de::Deserialize, ser::Serialize};
use serde_derive::{Deserialize, Serialize};

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
    T: ops::Not,
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
        Box::new(f(self))
    }
}
