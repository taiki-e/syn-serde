use super::*;

#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::ast_enum::Stmt;

ast_struct! {
    /// A braced block containing Rust statements.
    #[serde(transparent)]
    pub struct Block {
        /// Statements in a block
        pub(crate) stmts: Vec<Stmt>,
    }
}

ast_struct! {
    /// A local `let` binding: `let x: u64 = s.parse()?`.
    pub struct Local {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) pat: Pat,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) init: Option<Box<Expr>>,
    }
}
