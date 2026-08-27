// SPDX-License-Identifier: Apache-2.0 OR MIT

pub use crate::{ast_enum::Visibility, ast_struct::VisRestricted};

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
