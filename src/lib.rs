//! Library to serialize and deserialize [Syn] syntax trees.
//!
//! # Examples
//!
//! ```toml
//! [dependencies]
//! syn-serde = { version = "0.2", features = ["json"] }
//! syn = { version = "1", features = ["full"] }
//! ```
//!
//! ```rust
//! # #[cfg(feature = "json")]
//! # fn dox() {
//! use syn_serde::json;
//!
//! let syn_file: syn::File = syn::parse_quote! {
//!     fn main() {
//!         println!("Hello, world!");
//!     }
//! };
//! println!("{}", json::to_string_pretty(&syn_file));
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
//! ## Rust source file -> JSON representation of the syntax tree
//!
//! The [`rust2json`] example parse a Rust source file into a `syn_serde::File`
//! and print out a JSON representation of the syntax tree.
//!
//! ## JSON file -> Rust syntax tree
//!
//! The [`json2rust`] example parse a JSON file into a `syn_serde::File` and
//! print out a Rust syntax tree.
//!
//! # Optional features
//!
//! - **`json`** — Provides functions for JSON <-> Rust serializing and
//!   deserializing.
//!
//! [Syn]: https://github.com/dtolnay/syn
//! [`rust2json`]: https://github.com/taiki-e/syn-serde/tree/HEAD/examples/rust2json
//! [`json2rust`]: https://github.com/taiki-e/syn-serde/tree/HEAD/examples/json2rust

#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![forbid(unsafe_code)]
#![warn(future_incompatible, rust_2018_idioms, unreachable_pub)]
// It cannot be included in the published code because these lints have false positives in the minimum required version.
#![cfg_attr(test, warn(single_use_lifetimes))]
#![warn(clippy::default_trait_access)]
#![allow(clippy::needless_doctest_main, clippy::if_then_panic)]
#![allow(clippy::used_underscore_binding, clippy::wildcard_imports)]
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
        ast_enum::{AttrStyle, Meta, NestedMeta},
        ast_struct::{Attribute, MetaList, MetaNameValue},
    };
}
#[doc(hidden)]
pub use crate::attr::{AttrStyle, Attribute, Meta, MetaList, MetaNameValue, NestedMeta};

mod data;
pub(crate) use crate::data::assert_struct_semi;
#[doc(hidden)]
pub use crate::data::{
    Field, Fields, FieldsNamed, FieldsUnnamed, Variant, VisRestricted, Visibility,
};

mod expr;
#[doc(hidden)]
pub use crate::expr::{
    Arm, Expr, ExprArray, ExprAssign, ExprAssignOp, ExprAsync, ExprAwait, ExprBinary, ExprBlock,
    ExprBox, ExprBreak, ExprCall, ExprCast, ExprClosure, ExprContinue, ExprField, ExprForLoop,
    ExprGroup, ExprIf, ExprIndex, ExprLet, ExprLit, ExprLoop, ExprMacro, ExprMatch, ExprMethodCall,
    ExprParen, ExprPath, ExprRange, ExprReference, ExprRepeat, ExprReturn, ExprStruct, ExprTry,
    ExprTryBlock, ExprTuple, ExprType, ExprUnary, ExprUnsafe, ExprWhile, ExprYield, FieldValue,
    GenericMethodArgument, Index, Label, Member, MethodTurbofish, RangeLimits,
};

mod generics;
#[doc(hidden)]
pub use crate::generics::{
    BoundLifetimes, ConstParam, GenericParam, Generics, LifetimeDef, PredicateEq,
    PredicateLifetime, PredicateType, TraitBound, TraitBoundModifier, TypeParam, TypeParamBound,
    WhereClause, WherePredicate,
};

mod item;
#[doc(hidden)]
pub use crate::item::{
    FnArg, ForeignItem, ForeignItemFn, ForeignItemMacro, ForeignItemStatic, ForeignItemType,
    ImplItem, ImplItemConst, ImplItemMacro, ImplItemMethod, ImplItemType, Item, ItemConst,
    ItemEnum, ItemExternCrate, ItemFn, ItemForeignMod, ItemImpl, ItemMacro, ItemMacro2, ItemMod,
    ItemStatic, ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, ItemUse, Receiver,
    Signature, TraitItem, TraitItemConst, TraitItemMacro, TraitItemMethod, TraitItemType, UseGroup,
    UseName, UsePath, UseRename, UseTree,
};

mod file {
    #[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
    pub use crate::ast_struct::File;
}
#[doc(hidden)]
pub use crate::file::File;

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

mod ty;
#[doc(hidden)]
pub use crate::ty::{
    Abi, BareFnArg, ReturnType, Type, TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeMacro,
    TypeParen, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple, Variadic,
};

mod pat;
#[doc(hidden)]
pub use crate::pat::{
    FieldPat, Pat, PatBox, PatIdent, PatLit, PatMacro, PatOr, PatPath, PatRange, PatReference,
    PatRest, PatSlice, PatStruct, PatTuple, PatTupleStruct, PatType, PatWild,
};

mod path;
#[doc(hidden)]
pub use crate::path::{
    AngleBracketedGenericArguments, Binding, Constraint, GenericArgument,
    ParenthesizedGenericArguments, Path, PathArguments, PathSegment, QSelf,
};

mod stmt {
    #[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
    pub use crate::{
        ast_enum::Stmt,
        ast_struct::{Block, Local},
    };
}
#[doc(hidden)]
pub use crate::stmt::{Block, Local, Stmt};

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

use std::ops;

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
        Box::new(f(&**self))
    }
}
