// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::*;
#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::{
    ast_enum::Fields,
    ast_struct::{FieldsNamed, FieldsUnnamed, Variant, VisRestricted},
};

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
            assert!(semi_token, "unexpected end of input, expected `where` or `;`");
        }
        // struct foo
        Fields::Unit => assert!(
            semi_token,
            "unexpected end of input, expected one of: `where`, parentheses, curly braces, `;`"
        ),
    }
}

ast_struct! {
    /// An adapter for [`struct@syn::Field`].
    pub struct Field {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        #[serde(rename = "mut")]
        #[serde(default, skip_serializing_if = "FieldMutability::is_none")]
        pub(crate) mutability: FieldMutability,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) ident: Option<Ident>,
        // TODO: can remove?
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) colon_token: bool,
        pub(crate) ty: Type,
    }
}
