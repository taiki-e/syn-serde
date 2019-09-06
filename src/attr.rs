use super::*;

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

ast_enum! {
    /// Distinguishes between attributes that decorate an item and attributes
    /// that are contained within an item.
    ///
    /// # Outer attributes
    ///
    /// - `#[repr(transparent)]`
    /// - `/// # Example`
    /// - `/** Please file an issue */`
    ///
    /// # Inner attributes
    ///
    /// - `#![feature(proc_macro)]`
    /// - `//! # Example`
    /// - `/*! Please file an issue */`
    pub enum AttrStyle {
        Outer,
        Inner,
    }
}

ast_enum! {
    /// Content of a compile-time structured attribute.
    ///
    /// ## Path
    ///
    /// A meta path is like the `test` in `#[test]`.
    ///
    /// ## List
    ///
    /// A meta list is like the `derive(Copy)` in `#[derive(Copy)]`.
    ///
    /// ## NameValue
    ///
    /// A name-value meta is like the `path = "..."` in `#[path =
    /// "sys/windows.rs"]`.
    pub enum Meta {
        Path(Path),

        /// A structured list within an attribute, like `derive(Copy, Clone)`.
        List(MetaList),

        /// A name-value pair within an attribute, like `feature = "nightly"`.
        NameValue(MetaNameValue),
    }
}

ast_struct! {
    /// A structured list within an attribute, like `derive(Copy, Clone)`.
    pub struct MetaList {
        pub(crate) path: Path,
        pub(crate) nested: Punctuated<NestedMeta>,
    }
}

ast_struct! {
    /// A name-value pair within an attribute, like `feature = "nightly"`.
    pub struct MetaNameValue {
        pub(crate) path: Path,
        pub(crate) lit: Lit,
    }
}

ast_enum! {
    /// Element of a compile-time attribute list.
    pub enum NestedMeta {
        /// A structured meta item, like the `Copy` in `#[derive(Copy)]` which
        /// would be a nested `Meta::Word`.
        Meta(Meta),

        /// A Rust literal, like the `"new_name"` in `#[rename("new_name")]`.
        Lit(Lit),
    }
}
