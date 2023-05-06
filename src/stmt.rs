use super::*;
#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
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
