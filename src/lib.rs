//! Library to serialize and deserialize [Syn] syntax trees.
//!
//! ## Examples
//!
//! ```toml
//! [dependencies]
//! serde_syn = { version = "0.0.1", features = ["json"] }
//! syn = { version = "0.15", features = ["full"] }
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
//!                 "tts": [
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

// Serde Syn types in rustdoc of other crates get linked to here.
#![doc(html_root_url = "https://docs.rs/serde_syn/0.0.1")]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![deny(clippy::all, clippy::pedantic)]
// Ignored clippy::all lints.
#![allow(clippy::large_enum_variant)]
// Ignored clippy::pedantic lints.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::doc_markdown,
    clippy::if_not_else,
    clippy::module_name_repetitions,
    clippy::shadow_unrelated,
    clippy::similar_names,
    clippy::single_match_else
)]

#[macro_use]
mod macros;

mod attr;
pub use self::attr::{AttrStyle, Attribute, Meta, MetaList, MetaNameValue, NestedMeta};

mod data;
pub use self::data::{
    Field, Fields, FieldsNamed, FieldsUnnamed, Variant, VisRestricted, Visibility,
};

mod expr;
pub use self::expr::{
    Expr, ExprArray, ExprAssign, ExprAssignOp, ExprAsync, ExprBinary, ExprBlock, ExprBox,
    ExprBreak, ExprCall, ExprCast, ExprClosure, ExprContinue, ExprField, ExprForLoop, ExprGroup,
    ExprIf, ExprInPlace, ExprIndex, ExprLet, ExprLit, ExprLoop, ExprMacro, ExprMatch,
    ExprMethodCall, ExprParen, ExprPath, ExprRange, ExprReference, ExprRepeat, ExprReturn,
    ExprStruct, ExprTry, ExprTryBlock, ExprTuple, ExprType, ExprUnary, ExprUnsafe, ExprVerbatim,
    ExprWhile, ExprYield, Index, Member,
};

pub use self::expr::{
    Arm, Block, FieldPat, FieldValue, GenericMethodArgument, Label, Local, MethodTurbofish, Pat,
    PatBox, PatIdent, PatLit, PatMacro, PatPath, PatRange, PatRef, PatSlice, PatStruct, PatTuple,
    PatTupleStruct, PatVerbatim, RangeLimits, Stmt,
};

mod generics;
pub use self::generics::{
    BoundLifetimes, ConstParam, GenericParam, Generics, LifetimeDef, PredicateEq,
    PredicateLifetime, PredicateType, TraitBound, TraitBoundModifier, TypeParam, TypeParamBound,
    WhereClause, WherePredicate,
};

mod item;
pub use self::item::{
    ArgCaptured, ArgSelf, ArgSelfRef, FnArg, FnDecl, ForeignItem, ForeignItemFn, ForeignItemMacro,
    ForeignItemStatic, ForeignItemType, ForeignItemVerbatim, ImplItem, ImplItemConst,
    ImplItemExistential, ImplItemMacro, ImplItemMethod, ImplItemType, ImplItemVerbatim, Item,
    ItemConst, ItemEnum, ItemExistential, ItemExternCrate, ItemFn, ItemForeignMod, ItemImpl,
    ItemMacro, ItemMacro2, ItemMod, ItemStatic, ItemStruct, ItemTrait, ItemTraitAlias, ItemType,
    ItemUnion, ItemUse, ItemVerbatim, MethodSig, TraitItem, TraitItemConst, TraitItemMacro,
    TraitItemMethod, TraitItemType, TraitItemVerbatim, UseGroup, UseName, UsePath, UseRename,
    UseTree,
};

mod file;
pub use self::file::File;

mod lifetime;
pub use self::lifetime::Lifetime;

mod lit;
pub use self::lit::{
    Lit, LitBool, LitByte, LitByteStr, LitChar, LitFloat, LitInt, LitStr, LitVerbatim,
};

mod mac;
pub use self::mac::{Macro, MacroDelimiter};

mod op;
pub use self::op::{BinOp, UnOp};

mod ty;
pub use self::ty::{
    Abi, BareFnArg, BareFnArgName, ReturnType, Type, TypeArray, TypeBareFn, TypeGroup,
    TypeImplTrait, TypeMacro, TypeParen, TypePath, TypePtr, TypeReference, TypeSlice,
    TypeTraitObject, TypeTuple, TypeVerbatim,
};

mod path;
pub use self::path::{
    AngleBracketedGenericArguments, Binding, Constraint, GenericArgument,
    ParenthesizedGenericArguments, Path, PathArguments, PathSegment, QSelf,
};

mod token_stream;
pub use self::token_stream::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree,
};

#[cfg(feature = "json")]
pub mod json;

////////////////////////////////////////////////////////////////////////////////

use serde_derive::{Deserialize, Serialize};

// assertions
// `syn::perse*` functions will detect these, but there is a possibility to
// generate incorrect code by subsequent operations.
use self::data::assert_struct_semi;

type Punctuated<T> = Vec<T>;

fn default<T: Default>() -> T {
    T::default()
}

fn default_or_none<T: Default>(x: bool) -> Option<T> {
    if x {
        Some(T::default())
    } else {
        None
    }
}

fn not<T: std::ops::Not>(x: T) -> T::Output {
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
    type T: ?Sized;
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

impl<T, U, P: Default> MapInto<U, syn::punctuated::Punctuated<U, P>> for Vec<T> {
    type T = T;
    fn ref_map<'a, F>(&'a self, f: F) -> syn::punctuated::Punctuated<U, P>
    where
        F: FnMut(&'a Self::T) -> U,
    {
        self.iter().map(f).collect()
    }
}

impl<T, U, P: Default> MapInto<U, Vec<U>> for syn::punctuated::Punctuated<T, P> {
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

// Clippy false positive
// https://github.com/rust-lang/rust-clippy/issues/3410
#[allow(clippy::use_self)]
impl<T, U> MapInto<U, Box<U>> for Box<T> {
    type T = T;
    fn ref_map<'a, F>(&'a self, mut f: F) -> Box<U>
    where
        F: FnMut(&'a Self::T) -> U,
    {
        Box::new(f(&**self))
    }
}
