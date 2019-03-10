ast_enum! {
    /// A binary operator: `+`, `+=`, `&`.
    #[cfg_attr(feature = "clone-impls", derive(Copy))]
    pub enum BinOp #manual_from_impl {
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
    #[cfg_attr(feature = "clone-impls", derive(Copy))]
    pub enum UnOp #manual_from_impl {
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

mod convert {
    use super::super::*;
    use super::*;

    // BinOp

    impl From<&syn::BinOp> for BinOp {
        fn from(other: &syn::BinOp) -> Self {
            use super::BinOp::*;
            use syn::BinOp;
            match other {
                BinOp::Add(_) => Add,
                BinOp::Sub(_) => Sub,
                BinOp::Mul(_) => Mul,
                BinOp::Div(_) => Div,
                BinOp::Rem(_) => Rem,
                BinOp::And(_) => And,
                BinOp::Or(_) => Or,
                BinOp::BitXor(_) => BitXor,
                BinOp::BitAnd(_) => BitAnd,
                BinOp::BitOr(_) => BitOr,
                BinOp::Shl(_) => Shl,
                BinOp::Shr(_) => Shr,
                BinOp::Eq(_) => Eq,
                BinOp::Lt(_) => Lt,
                BinOp::Le(_) => Le,
                BinOp::Ne(_) => Ne,
                BinOp::Ge(_) => Ge,
                BinOp::Gt(_) => Gt,
                BinOp::AddEq(_) => AddEq,
                BinOp::SubEq(_) => SubEq,
                BinOp::MulEq(_) => MulEq,
                BinOp::DivEq(_) => DivEq,
                BinOp::RemEq(_) => RemEq,
                BinOp::BitXorEq(_) => BitXorEq,
                BinOp::BitAndEq(_) => BitAndEq,
                BinOp::BitOrEq(_) => BitOrEq,
                BinOp::ShlEq(_) => ShlEq,
                BinOp::ShrEq(_) => ShrEq,
            }
        }
    }

    impl From<&BinOp> for syn::BinOp {
        fn from(other: &BinOp) -> Self {
            use syn::BinOp::*;
            match other {
                BinOp::Add => Add(default()),
                BinOp::Sub => Sub(default()),
                BinOp::Mul => Mul(default()),
                BinOp::Div => Div(default()),
                BinOp::Rem => Rem(default()),
                BinOp::And => And(default()),
                BinOp::Or => Or(default()),
                BinOp::BitXor => BitXor(default()),
                BinOp::BitAnd => BitAnd(default()),
                BinOp::BitOr => BitOr(default()),
                BinOp::Shl => Shl(default()),
                BinOp::Shr => Shr(default()),
                BinOp::Eq => Eq(default()),
                BinOp::Lt => Lt(default()),
                BinOp::Le => Le(default()),
                BinOp::Ne => Ne(default()),
                BinOp::Ge => Ge(default()),
                BinOp::Gt => Gt(default()),
                BinOp::AddEq => AddEq(default()),
                BinOp::SubEq => SubEq(default()),
                BinOp::MulEq => MulEq(default()),
                BinOp::DivEq => DivEq(default()),
                BinOp::RemEq => RemEq(default()),
                BinOp::BitXorEq => BitXorEq(default()),
                BinOp::BitAndEq => BitAndEq(default()),
                BinOp::BitOrEq => BitOrEq(default()),
                BinOp::ShlEq => ShlEq(default()),
                BinOp::ShrEq => ShrEq(default()),
            }
        }
    }

    // UnOp

    impl From<&syn::UnOp> for UnOp {
        fn from(other: &syn::UnOp) -> Self {
            use super::UnOp::*;
            use syn::UnOp;
            match other {
                UnOp::Deref(_) => Deref,
                UnOp::Not(_) => Not,
                UnOp::Neg(_) => Neg,
            }
        }
    }

    impl From<&UnOp> for syn::UnOp {
        fn from(other: &UnOp) -> Self {
            use syn::UnOp::*;
            match other {
                UnOp::Deref => Deref(default()),
                UnOp::Not => Not(default()),
                UnOp::Neg => Neg(default()),
            }
        }
    }
}
