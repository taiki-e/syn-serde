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
    /// comments. The `tts` field contains the rest of the attribute body as
    /// tokens.
    ///
    /// ```text
    /// #[derive(Copy)]      #[crate::precondition x < 5]
    ///   ^^^^^^~~~~~~         ^^^^^^^^^^^^^^^^^^^ ~~~~~
    ///    path  tts                   path         tts
    /// ```
    pub struct Attribute {
        style: AttrStyle,
        path: Path,
        #[serde(default, skip_serializing_if = "TokenStream::is_empty")]
        tts: TokenStream,
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
    pub enum AttrStyle #manual_from_impl {
        Outer,
        Inner,
    }
}

ast_enum_of_structs! {
    /// Content of a compile-time structured attribute.
    ///
    /// ## Word
    ///
    /// A meta word is like the `test` in `#[test]`.
    ///
    /// ## List
    ///
    /// A meta list is like the `derive(Copy)` in `#[derive(Copy)]`.
    ///
    /// ## NameValue
    ///
    /// A name-value meta is like the `path = "..."` in `#[path =
    /// "sys/windows.rs"]`.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum Meta {
        pub Word(Ident),
        /// A structured list within an attribute, like `derive(Copy, Clone)`.
        pub List(MetaList {
            ident: Ident,
            nested: Punctuated<NestedMeta>,
        }),
        /// A name-value pair within an attribute, like `feature = "nightly"`.
        pub NameValue(MetaNameValue {
            ident: Ident,
            lit: Lit,
        }),
    }
}

impl Meta {
    /// Returns the identifier that begins this structured meta item.
    ///
    /// For example this would return the `test` in `#[test]`, the `derive` in
    /// `#[derive(Copy)]`, and the `path` in `#[path = "sys/windows.rs"]`.
    pub fn name(&self) -> Ident {
        match *self {
            Meta::Word(ref meta) => meta.clone(),
            Meta::List(ref meta) => meta.ident.clone(),
            Meta::NameValue(ref meta) => meta.ident.clone(),
        }
    }
}

ast_enum_of_structs! {
    /// Element of a compile-time attribute list.
    pub enum NestedMeta {
        /// A structured meta item, like the `Copy` in `#[derive(Copy)]` which
        /// would be a nested `Meta::Word`.
        pub Meta(Meta),

        /// A Rust literal, like the `"new_name"` in `#[rename("new_name")]`.
        pub Literal(Lit),
    }
}

mod convert {
    use super::*;

    // Attribute

    impl From<&syn::Attribute> for Attribute {
        fn from(other: &syn::Attribute) -> Self {
            Self {
                style: other.style.ref_into(),
                path: other.path.ref_into(),
                tts: other.tts.ref_into(),
            }
        }
    }

    impl From<&Attribute> for syn::Attribute {
        fn from(other: &Attribute) -> Self {
            Self {
                pound_token: default(),
                style: other.style.ref_into(),
                bracket_token: default(),
                path: other.path.ref_into(),
                tts: other.tts.ref_into(),
            }
        }
    }

    // AttrStyle

    impl From<&syn::AttrStyle> for AttrStyle {
        fn from(other: &syn::AttrStyle) -> Self {
            use super::AttrStyle::*;
            use syn::AttrStyle;
            match other {
                AttrStyle::Outer => Outer,
                AttrStyle::Inner(_) => Inner,
            }
        }
    }

    impl From<&AttrStyle> for syn::AttrStyle {
        fn from(other: &AttrStyle) -> Self {
            use syn::AttrStyle::*;
            match other {
                AttrStyle::Outer => Outer,
                AttrStyle::Inner => Inner(default()),
            }
        }
    }

    // MetaList

    impl From<&syn::MetaList> for MetaList {
        fn from(other: &syn::MetaList) -> Self {
            Self {
                ident: other.ident.ref_into(),
                nested: other.nested.map_into(),
            }
        }
    }

    impl From<&MetaList> for syn::MetaList {
        fn from(other: &MetaList) -> Self {
            Self {
                ident: other.ident.ref_into(),
                paren_token: default(),
                nested: other.nested.map_into(),
            }
        }
    }

    // MetaNameValue

    impl From<&syn::MetaNameValue> for MetaNameValue {
        fn from(other: &syn::MetaNameValue) -> Self {
            Self {
                ident: other.ident.ref_into(),
                lit: other.lit.ref_into(),
            }
        }
    }

    impl From<&MetaNameValue> for syn::MetaNameValue {
        fn from(other: &MetaNameValue) -> Self {
            Self {
                ident: other.ident.ref_into(),
                eq_token: default(),
                lit: other.lit.ref_into(),
            }
        }
    }
}
