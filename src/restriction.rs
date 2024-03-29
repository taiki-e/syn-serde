// SPDX-License-Identifier: Apache-2.0 OR MIT

pub use crate::{
    ast_enum::{FieldMutability, Visibility},
    ast_struct::VisRestricted,
};

impl Visibility {
    pub(crate) fn is_inherited(&self) -> bool {
        matches!(self, Self::Inherited)
    }
}
impl Default for Visibility {
    fn default() -> Self {
        Self::Inherited
    }
}

impl FieldMutability {
    pub(crate) fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}
impl Default for FieldMutability {
    fn default() -> Self {
        Self::None
    }
}
