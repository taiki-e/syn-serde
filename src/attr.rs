use super::*;

#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::{
    ast_enum::{AttrStyle, Meta, NestedMeta},
    ast_struct::{MetaList, MetaNameValue},
};

ast_struct! {
    /// An attribute like `#[repr(transparent)]`.
    ///
    /// # Syntax
    ///
    /// Rust has six types of attributes.
    ///
    /// - Outer attributes like `#[repr(transparent)]`. These appear outside or
    ///   in front of the item they describe.
    /// - Inner attributes like `#![feature(proc_macro)]`. These appear inside
    ///   of the item they describe, usually a module.
    /// - Outer doc comments like `/// # Example`.
    /// - Inner doc comments like `//! Please file an issue`.
    /// - Outer block comments `/** # Example */`.
    /// - Inner block comments `/*! Please file an issue */`.
    ///
    /// The `style` field of type `AttrStyle` distinguishes whether an attribute
    /// is outer or inner. Doc comments and block comments are promoted to
    /// attributes, as this is how they are processed by the compiler and by
    /// `macro_rules!` macros.
    ///
    /// The `path` field gives the possibly colon-delimited path against which
    /// the attribute is resolved. It is equal to `"doc"` for desugared doc
    /// comments. The `tokens` field contains the rest of the attribute body as
    /// tokens.
    ///
    /// ```text
    /// #[derive(Copy)]      #[crate::precondition x < 5]
    ///   ^^^^^^~~~~~~         ^^^^^^^^^^^^^^^^^^^ ~~~~~
    ///    path  tokens                   path         tokens
    /// ```
    pub struct Attribute {
        pub(crate) style: AttrStyle,
        pub(crate) path: Path,
        #[serde(default, skip_serializing_if = "TokenStream::is_empty")]
        pub(crate) tokens: TokenStream,
    }
}
