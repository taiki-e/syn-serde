// SPDX-License-Identifier: Apache-2.0 OR MIT

use alloc::vec::Vec;

use super::*;
pub use crate::{
    ast_enum::Pat,
    ast_struct::{
        FieldPat, PatIdent, PatParen, PatReference, PatRest, PatSlice, PatStruct, PatTuple,
        PatTupleStruct, PatType, PatWild,
    },
};

ast_struct! {
    /// An adapter for [`struct@syn::PatOr`].
    pub struct PatOr {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        // TODO: can remove
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) leading_vert: bool,
        pub(crate) cases: Punctuated<Pat>,
    }
}
