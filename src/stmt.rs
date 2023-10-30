// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::*;
pub use crate::ast_struct::{Block, Local, LocalInit, StmtMacro};

ast_enum! {
    /// An adapter for [`enum@syn::Stmt`].
    pub enum Stmt {
        #[serde(rename = "let")]
        Local(Local),
        Item(Item),
        // TODO: Should be Expr { expr: Exor, semi: bool }?
        Expr(Expr, bool),
        Macro(StmtMacro),
    }
}
