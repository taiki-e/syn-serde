use super::*;
use std::slice::{Iter, IterMut};

ast_struct! {
    /// An enum variant.
    pub struct Variant {
        /// Attributes tagged on the variant.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        attrs: Vec<Attribute>,

        /// Name of the variant.
        ident: Ident,

        /// Content stored in the variant.
        fields: Fields,

        /// Explicit discriminant: `Variant = 1`
        #[serde(default, skip_serializing_if = "Option::is_none")]
        discriminant: Option<Expr>,
    }
}

ast_enum_of_structs! {
    /// Data stored within an enum variant or struct.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum Fields #manual_from_impl {
        /// Named fields of a struct or struct variant such as `Point { x: f64,
        /// y: f64 }`.
        pub Named(FieldsNamed #transparent {
            named: Punctuated<Field>,
        }),

        /// Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.
        pub Unnamed(FieldsUnnamed #transparent {
            unnamed: Punctuated<Field>,
        }),

        /// Unit struct or unit variant such as `None`.
        pub Unit,
    }
}

impl Fields {
    /// Get an iterator over the borrowed [`Field`] items in this object. This
    /// iterator can be used to iterate over a named or unnamed struct or
    /// variant's fields uniformly.
    ///
    /// [`Field`]: struct.Field.html
    pub fn iter(&self) -> Iter<'_, Field> {
        match *self {
            Fields::Unit => [].iter(),
            Fields::Named(ref f) => f.named.iter(),
            Fields::Unnamed(ref f) => f.unnamed.iter(),
        }
    }

    /// Get an iterator over the mutably borrowed [`Field`] items in this
    /// object. This iterator can be used to iterate over a named or unnamed
    /// struct or variant's fields uniformly.
    ///
    /// [`Field`]: struct.Field.html
    pub fn iter_mut(&mut self) -> IterMut<'_, Field> {
        match *self {
            Fields::Unit => [].iter_mut(),
            Fields::Named(ref mut f) => f.named.iter_mut(),
            Fields::Unnamed(ref mut f) => f.unnamed.iter_mut(),
        }
    }

    pub(crate) fn is_named(&self) -> bool {
        match self {
            Fields::Named(_) => true,
            Fields::Unnamed(_) | Fields::Unit => false,
        }
    }
}

pub(crate) fn assert_struct_semi(fields: &Fields, semi_token: bool) {
    match fields {
        // struct foo {};
        Fields::Named(_) => assert!(!semi_token, "unexpected token: `;`"),
        // struct foo ()
        Fields::Unnamed(_) => assert!(
            semi_token,
            "unexpected end of input, expected `where` or `;`"
        ),
        // struct foo
        Fields::Unit => assert!(
            semi_token,
            "unexpected end of input, expected one of: `where`, parentheses, curly braces, `;`"
        ),
    }
}

impl<'a> IntoIterator for &'a Fields {
    type Item = &'a Field;
    type IntoIter = Iter<'a, Field>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Fields {
    type Item = &'a mut Field;
    type IntoIter = IterMut<'a, Field>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

ast_struct! {
    /// A field of a struct or enum variant.
    pub struct Field {
        /// Attributes tagged on the field.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        attrs: Vec<Attribute>,

        /// Visibility of the field.
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        vis: Visibility,

        /// Name of the field, if any.
        ///
        /// Fields of tuple structs have no names.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ident: Option<Ident>,

        #[serde(default, skip_serializing_if = "not")]
        colon_token: bool,

        /// Type of the field.
        ty: Type,
    }
}

ast_enum_of_structs! {
    /// The visibility level of an item: inherited or `pub` or
    /// `pub(restricted)`.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum Visibility #manual_from_impl {
        /// A public visibility level: `pub`.
        #[serde(rename = "pub")]
        pub Public,

        /// A crate-level visibility: `crate`.
        pub Crate,

        /// A visibility level restricted to some path: `pub(self)` or
        /// `pub(super)` or `pub(crate)` or `pub(in some::module)`.
        pub Restricted(VisRestricted {
            #[serde(default, skip_serializing_if = "not")]
            in_token: bool,
            path: Box<Path>,
        }),

        /// An inherited visibility, which usually means private.
        pub Inherited,
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

mod convert {
    use super::*;

    // Variant

    impl From<&syn::Variant> for Variant {
        fn from(other: &syn::Variant) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                ident: other.ident.ref_into(),
                fields: other.fields.ref_into(),
                discriminant: other.discriminant.ref_map(|(_, x)| x.ref_into()),
            }
        }
    }

    impl From<&Variant> for syn::Variant {
        fn from(other: &Variant) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                ident: other.ident.ref_into(),
                fields: other.fields.ref_into(),
                discriminant: other.discriminant.ref_map(|x| (default(), x.ref_into())),
            }
        }
    }

    // Fields

    impl From<&syn::Fields> for Fields {
        fn from(other: &syn::Fields) -> Self {
            use super::Fields::*;
            use syn::Fields;
            match other {
                Fields::Named(x) => Named(x.ref_into()),
                Fields::Unnamed(x) => Unnamed(x.ref_into()),
                Fields::Unit => Unit,
            }
        }
    }

    impl From<&Fields> for syn::Fields {
        fn from(other: &Fields) -> Self {
            use syn::Fields::*;
            match other {
                Fields::Named(x) => Named(x.ref_into()),
                Fields::Unnamed(x) => Unnamed(x.ref_into()),
                Fields::Unit => Unit,
            }
        }
    }

    // FieldsNamed

    impl From<&syn::FieldsNamed> for FieldsNamed {
        fn from(other: &syn::FieldsNamed) -> Self {
            Self {
                named: other.named.map_into(),
            }
        }
    }

    impl From<&FieldsNamed> for syn::FieldsNamed {
        fn from(other: &FieldsNamed) -> Self {
            Self {
                brace_token: default(),
                named: other.named.map_into(),
            }
        }
    }

    // FieldsUnnamed

    impl From<&syn::FieldsUnnamed> for FieldsUnnamed {
        fn from(other: &syn::FieldsUnnamed) -> Self {
            Self {
                unnamed: other.unnamed.map_into(),
            }
        }
    }

    impl From<&FieldsUnnamed> for syn::FieldsUnnamed {
        fn from(other: &FieldsUnnamed) -> Self {
            Self {
                paren_token: default(),
                unnamed: other.unnamed.map_into(),
            }
        }
    }

    // Field

    impl From<&syn::Field> for Field {
        fn from(other: &syn::Field) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.map_into(),
                colon_token: other.colon_token.is_some(),
                ty: other.ty.ref_into(),
            }
        }
    }

    impl From<&Field> for syn::Field {
        fn from(other: &Field) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.map_into(),
                colon_token: default_or_none(other.colon_token),
                ty: other.ty.ref_into(),
            }
        }
    }

    // Visibility

    impl From<&syn::Visibility> for Visibility {
        fn from(other: &syn::Visibility) -> Self {
            use super::Visibility::*;
            use syn::Visibility;
            match other {
                Visibility::Public(_) => Public,
                Visibility::Crate(_) => Crate,
                Visibility::Restricted(x) => Restricted(x.ref_into()),
                Visibility::Inherited => Inherited,
            }
        }
    }

    impl From<&Visibility> for syn::Visibility {
        fn from(other: &Visibility) -> Self {
            use syn::Visibility::*;
            match other {
                Visibility::Public => Public(syn::VisPublic {
                    pub_token: default(),
                }),
                Visibility::Crate => Crate(syn::VisCrate {
                    crate_token: default(),
                }),
                Visibility::Restricted(x) => Restricted(x.into()),
                Visibility::Inherited => Inherited,
            }
        }
    }

    // VisRestricted

    impl From<&syn::VisRestricted> for VisRestricted {
        fn from(other: &syn::VisRestricted) -> Self {
            Self {
                in_token: other.in_token.is_some(),
                path: other.path.map_into(),
            }
        }
    }

    impl From<&VisRestricted> for syn::VisRestricted {
        fn from(other: &VisRestricted) -> Self {
            Self {
                pub_token: default(),
                paren_token: default(),
                in_token: default_or_none(other.in_token),
                path: other.path.map_into(),
            }
        }
    }
}
