ast_enum! {
    /// A binary operator: `+`, `+=`, `&`.
    pub enum BinOp {
        /// The `+` operator (addition)
        #[serde(rename = "+")]
        Add,
        /// The `-` operator (subtraction)
        #[serde(rename = "-")]
        Sub,
        /// The `*` operator (multiplication)
        #[serde(rename = "*")]
        Mul,
        /// The `/` operator (division)
        #[serde(rename = "/")]
        Div,
        /// The `%` operator (modulus)
        #[serde(rename = "%")]
        Rem,
        /// The `&&` operator (logical and)
        #[serde(rename = "&&")]
        And,
        /// The `||` operator (logical or)
        #[serde(rename = "||")]
        Or,
        /// The `^` operator (bitwise xor)
        #[serde(rename = "^")]
        BitXor,
        /// The `&` operator (bitwise and)
        #[serde(rename = "&")]
        BitAnd,
        /// The `|` operator (bitwise or)
        #[serde(rename = "|")]
        BitOr,
        /// The `<<` operator (shift left)
        #[serde(rename = "<<")]
        Shl,
        /// The `>>` operator (shift right)
        #[serde(rename = ">>")]
        Shr,
        /// The `==` operator (equality)
        #[serde(rename = "==")]
        Eq,
        /// The `<` operator (less than)
        #[serde(rename = "<")]
        Lt,
        /// The `<=` operator (less than or equal to)
        #[serde(rename = "<=")]
        Le,
        /// The `!=` operator (not equal to)
        #[serde(rename = "!=")]
        Ne,
        /// The `>=` operator (greater than or equal to)
        #[serde(rename = ">=")]
        Ge,
        /// The `>` operator (greater than)
        #[serde(rename = ">")]
        Gt,
        /// The `+=` operator
        #[serde(rename = "+=")]
        AddEq,
        /// The `-=` operator
        #[serde(rename = "-=")]
        SubEq,
        /// The `*=` operator
        #[serde(rename = "*=")]
        MulEq,
        /// The `/=` operator
        #[serde(rename = "/=")]
        DivEq,
        /// The `%=` operator
        #[serde(rename = "%=")]
        RemEq,
        /// The `^=` operator
        #[serde(rename = "^=")]
        BitXorEq,
        /// The `&=` operator
        #[serde(rename = "&=")]
        BitAndEq,
        /// The `|=` operator
        #[serde(rename = "|=")]
        BitOrEq,
        /// The `<<=` operator
        #[serde(rename = "<<=")]
        ShlEq,
        /// The `>>=` operator
        #[serde(rename = ">>=")]
        ShrEq,
    }
}

ast_enum! {
    /// A unary operator: `*`, `!`, `-`.
    pub enum UnOp {
        /// The `*` operator for dereferencing
        #[serde(rename = "*")]
        Deref,
        /// The `!` operator for logical inversion
        #[serde(rename = "!")]
        Not,
        /// The `-` operator for negation
        #[serde(rename = "-")]
        Neg,
    }
}
