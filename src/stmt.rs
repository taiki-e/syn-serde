use super::*;

ast_struct! {
    /// A braced block containing Rust statements.
    #[serde(transparent)]
    pub struct Block {
        /// Statements in a block
        pub(crate) stmts: Vec<Stmt>,
    }
}

ast_enum! {
    /// A statement, usually ending in a semicolon.
    pub enum Stmt {
        /// A local (let) binding.
        #[serde(rename = "let")]
        Local(Local),

        /// An item definition.
        Item(Item),

        /// Expr without trailing semicolon.
        Expr(Expr),

        /// Expression with trailing semicolon.
        Semi(Expr),
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
