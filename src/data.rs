use super::*;

#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::ast_enum::Fields;

ast_struct! {
    /// An enum variant.
    pub struct Variant {
        /// Attributes tagged on the variant.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,

        /// Name of the variant.
        pub(crate) ident: Ident,

        /// Content stored in the variant.
        pub(crate) fields: Fields,

        /// Explicit discriminant: `Variant = 1`
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) discriminant: Option<Expr>,
    }
}

ast_struct! {
    /// Named fields of a struct or struct variant such as `Point { x: f64,
    /// y: f64 }`.
    #[serde(transparent)]
    pub struct FieldsNamed {
        pub(crate) named: Punctuated<Field>,
    }
}

ast_struct! {
    /// Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.
    #[serde(transparent)]
    pub struct FieldsUnnamed {
        pub(crate) unnamed: Punctuated<Field>,
    }
}

impl Fields {
    pub(crate) fn is_named(&self) -> bool {
        match self {
            Fields::Named(_) => true,
            Fields::Unnamed(_) | Fields::Unit => false,
        }
    }
}

// assertions
// `syn::perse*` functions will detect these, but there is a possibility to
// generate incorrect code by subsequent operations.
pub(crate) fn assert_struct_semi(fields: &Fields, semi_token: bool) {
    match fields {
        // struct foo {};
        Fields::Named(_) => assert!(!semi_token, "unexpected token: `;`"),
        // struct foo ()
        Fields::Unnamed(_) => {
            assert!(semi_token, "unexpected end of input, expected `where` or `;`")
        }
        // struct foo
        Fields::Unit => assert!(
            semi_token,
            "unexpected end of input, expected one of: `where`, parentheses, curly braces, `;`"
        ),
    }
}

ast_struct! {
    /// A field of a struct or enum variant.
    pub struct Field {
        /// Attributes tagged on the field.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,

        /// Visibility of the field.
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,

        /// Name of the field, if any.
        ///
        /// Fields of tuple structs have no names.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) ident: Option<Ident>,

        #[serde(default, skip_serializing_if = "not")]
        pub(crate) colon_token: bool,

        /// Type of the field.
        pub(crate) ty: Type,
    }
}

ast_enum! {
    /// The visibility level of an item: inherited or `pub` or
    /// `pub(restricted)`.
    pub enum Visibility {
        /// A public visibility level: `pub`.
        #[serde(rename = "pub")]
        Public,

        /// A crate-level visibility: `crate`.
        Crate,

        /// A visibility level restricted to some path: `pub(self)` or
        /// `pub(super)` or `pub(crate)` or `pub(in some::module)`.
        Restricted(VisRestricted),

        /// An inherited visibility, which usually means private.
        Inherited,
    }
}

impl Visibility {
    pub(crate) fn is_inherited(&self) -> bool {
        match self {
            Visibility::Inherited => true,
            _ => false,
        }
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Inherited
    }
}

ast_struct! {
    /// A visibility level restricted to some path: `pub(self)` or
    /// `pub(super)` or `pub(crate)` or `pub(in some::module)`.
    pub struct VisRestricted {
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) in_token: bool,
        pub(crate) path: Box<Path>,
    }
}
