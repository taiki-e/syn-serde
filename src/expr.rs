use super::*;

#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::ast_enum::{Expr, GenericMethodArgument, Member, RangeLimits};

ast_struct! {
    /// A slice literal expression: `[a, b, c, d]`.
    pub struct ExprArray {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) elems: Punctuated<Expr>,
    }
}

ast_struct! {
    /// An assignment expression: `a = compute()`.
    pub struct ExprAssign {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) left: Box<Expr>,
        pub(crate) right: Box<Expr>,
    }
}

ast_struct! {
    /// A compound assignment expression: `counter += 1`.
    pub struct ExprAssignOp {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) left: Box<Expr>,
        pub(crate) op: BinOp,
        pub(crate) right: Box<Expr>,
    }
}

ast_struct! {
    /// An async block: `async { ... }`.
    pub struct ExprAsync {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(rename = "move")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) capture: bool,
        #[serde(rename = "stmts")]
        pub(crate) block: Block,
    }
}

ast_struct! {
    /// An await expression: `fut.await`.
    pub struct ExprAwait {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) base: Box<Expr>,
    }
}

ast_struct! {
    /// A binary operation: `a + b`, `a * b`.
    pub struct ExprBinary {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) left: Box<Expr>,
        pub(crate) op: BinOp,
        pub(crate) right: Box<Expr>,
    }
}

ast_struct! {
    /// A blocked scope: `{ ... }`.
    pub struct ExprBlock {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) label: Option<Label>,
        #[serde(rename = "stmts")]
        pub(crate) block: Block,
    }
}

ast_struct! {
    /// A box expression: `box f`.
    pub struct ExprBox {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) expr: Box<Expr>,
    }
}

ast_struct! {
    /// A `break`, with an optional label to break and an optional
    /// expression.
    pub struct ExprBreak {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) label: Option<Lifetime>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) expr: Option<Box<Expr>>,
    }
}

ast_struct! {
    /// A function call expression: `invoke(a, b)`.
    pub struct ExprCall {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) func: Box<Expr>,
        pub(crate) args: Punctuated<Expr>,
    }
}

ast_struct! {
    /// A cast expression: `foo as f64`.
    pub struct ExprCast {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) expr: Box<Expr>,
        pub(crate) ty: Box<Type>,
    }
}

ast_struct! {
    /// A closure expression: `|a, b| a + b`.
    pub struct ExprClosure {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(rename = "async")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) asyncness: bool,
        #[serde(rename = "static")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) movability: bool,
        #[serde(rename = "move")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) capture: bool,
        pub(crate) inputs: Punctuated<Pat>,
        #[serde(default)]
        pub(crate) output: ReturnType,
        pub(crate) body: Box<Expr>,
    }
}

ast_struct! {
    /// A `continue`, with an optional label.
    pub struct ExprContinue {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) label: Option<Lifetime>,
    }
}

ast_struct! {
    /// Access of a named struct field (`obj.k`) or unnamed tuple struct
    /// field (`obj.0`).
    pub struct ExprField {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) base: Box<Expr>,
        #[serde(flatten)]
        pub(crate) member: Member,
    }
}

ast_struct! {
    /// A for loop: `for pat in expr { ... }`.
    pub struct ExprForLoop {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) label: Option<Label>,
        pub(crate) pat: Pat,
        pub(crate) expr: Box<Expr>,
        pub(crate) body: Block,
    }
}

ast_struct! {
    /// An expression contained within invisible delimiters.
    ///
    /// This variant is important for faithfully representing the precedence
    /// of expressions and is related to `None`-delimited spans in a
    /// `TokenStream`.
    pub struct ExprGroup {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) expr: Box<Expr>,
    }
}

ast_struct! {
    /// An `if` expression with an optional `else` block: `if expr { ... }
    /// else { ... }`.
    ///
    /// The `else` branch expression may only be an `If` or `Block`
    /// expression, not any of the other types of expression.
    pub struct ExprIf {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) cond: Box<Expr>,
        pub(crate) then_branch: Block,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) else_branch: Option<Box<Expr>>,
    }
}

ast_struct! {
    /// A square bracketed indexing expression: `vector[2]`.
    pub struct ExprIndex {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) expr: Box<Expr>,
        pub(crate) index: Box<Expr>,
    }
}

ast_struct! {
    /// A `let` guard: `let Some(x) = opt`.
    pub struct ExprLet {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) pat: Pat,
        pub(crate) expr: Box<Expr>,
    }
}

ast_struct! {
    /// A literal in place of an expression: `1`, `"foo"`.
    pub struct ExprLit {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(flatten)]
        pub(crate) lit: Lit,
    }
}

ast_struct! {
    /// Conditionless loop: `loop { ... }`.
    pub struct ExprLoop {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) label: Option<Label>,
        pub(crate) body: Block,
    }
}

ast_struct! {
    /// A macro invocation expression: `format!("{}", q)`.
    pub struct ExprMacro {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(flatten)]
        pub(crate) mac: Macro,
    }
}

ast_struct! {
    /// A `match` expression: `match n { Some(n) => {}, None => {} }`.
    pub struct ExprMatch {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) expr: Box<Expr>,
        pub(crate) arms: Vec<Arm>,
    }
}

ast_struct! {
    /// A method call expression: `x.foo::<T>(a, b)`.
    pub struct ExprMethodCall {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) receiver: Box<Expr>,
        pub(crate) method: Ident,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) turbofish: Option<MethodTurbofish>,
        pub(crate) args: Punctuated<Expr>,
    }
}

ast_struct! {
    /// A parenthesized expression: `(a + b)`.
    pub struct ExprParen {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) expr: Box<Expr>,
    }
}

ast_struct! {
    /// A path like `std::mem::replace` possibly containing generic
    /// parameters and a qualified self-type.
    ///
    /// A plain identifier like `x` is a path of length 1.
    pub struct ExprPath {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) qself: Option<QSelf>,
        #[serde(flatten)]
        pub(crate) path: Path,
    }
}

ast_struct! {
    /// A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.
    pub struct ExprRange {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) from: Option<Box<Expr>>,
        pub(crate) limits: RangeLimits,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) to: Option<Box<Expr>>,
    }
}

ast_struct! {
    /// A referencing operation: `&a` or `&mut a`.
    pub struct ExprReference {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        // #[serde(default, skip_serializing_if = "Reserved::is_default")]
        // pub(crate) raw: Reserved,
        #[serde(rename = "mut")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) mutability: bool,
        pub(crate) expr: Box<Expr>,
    }
}

ast_struct! {
    /// An array literal constructed from one repeated element: `[0u8; N]`.
    pub struct ExprRepeat {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) expr: Box<Expr>,
        pub(crate) len: Box<Expr>,
    }
}

ast_struct! {
    /// A `return`, with an optional value to be returned.
    pub struct ExprReturn {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) expr: Option<Box<Expr>>,
    }
}

ast_struct! {
    /// A struct literal expression: `Point { x: 1, y: 1 }`.
    ///
    /// The `rest` provides the value of the remaining fields as in `S { a:
    /// 1, b: 1, ..rest }`.
    pub struct ExprStruct {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) path: Path,
        pub(crate) fields: Punctuated<FieldValue>,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) dot2_token: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) rest: Option<Box<Expr>>,
    }
}

ast_struct! {
    /// A try-expression: `expr?`.
    pub struct ExprTry {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) expr: Box<Expr>,
    }
}

ast_struct! {
    /// A try block: `try { ... }`.
    pub struct ExprTryBlock {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(rename = "stmts")]
        pub(crate) block: Block,
    }
}

ast_struct! {
    /// A tuple expression: `(a, b, c, d)`.
    pub struct ExprTuple {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) elems: Punctuated<Expr>,
    }
}

ast_struct! {
    /// A type ascription expression: `foo: f64`.
    pub struct ExprType {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) expr: Box<Expr>,
        pub(crate) ty: Box<Type>,
    }
}

ast_struct! {
    /// A unary operation: `!x`, `*x`.
    pub struct ExprUnary {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) op: UnOp,
        pub(crate) expr: Box<Expr>,
    }
}

ast_struct! {
    /// An unsafe block: `unsafe { ... }`.
    pub struct ExprUnsafe {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(rename = "stmts")]
        pub(crate) block: Block,
    }
}

ast_struct! {
    /// A while loop: `while expr { ... }`.
    pub struct ExprWhile {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) label: Option<Label>,
        pub(crate) cond: Box<Expr>,
        pub(crate) body: Block,
    }
}

ast_struct! {
    /// A yield expression: `yield expr`.
    pub struct ExprYield {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) expr: Option<Box<Expr>>,
    }
}

ast_struct! {
    /// The index of an unnamed tuple struct field.
    #[serde(transparent)]
    pub struct Index {
        pub(crate) index: u32,
    }
}

ast_struct! {
    /// The `::<>` explicit type parameters passed to a method call:
    /// `parse::<u64>()`.
    pub struct MethodTurbofish {
        pub(crate) args: Punctuated<GenericMethodArgument>,
    }
}

ast_struct! {
    /// A field-value pair in a struct literal.
    pub struct FieldValue {
        /// Attributes tagged on the field.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,

        /// Name or index of the field.
        #[serde(flatten)]
        pub(crate) member: Member,

        /// The colon in `Struct { x: x }`. If written in shorthand like
        /// `Struct { x }`, there is no colon.
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) colon_token: bool,

        /// Value of the field.
        pub(crate) expr: Expr,
    }
}

ast_struct! {
    /// A lifetime labeling a `for`, `while`, or `loop`.
    #[serde(transparent)]
    pub struct Label {
        pub(crate) name: Lifetime,
    }
}

ast_struct! {
    /// One arm of a `match` expression: `0...10 => { return true; }`.
    pub struct Arm {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) pat: Pat,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) guard: Option<Box<Expr>>,
        pub(crate) body: Box<Expr>,
        // #[serde(default, skip_serializing_if = "not")]
        // pub(crate) comma: bool,
    }
}

pub(crate) fn requires_terminator(expr: &Expr) -> bool {
    // see https://github.com/rust-lang/rust/blob/eb8f2586e/src/libsyntax/parse/classify.rs#L17-L37
    match expr {
        Expr::Unsafe(..)
        | Expr::Block(..)
        | Expr::If(..)
        | Expr::Match(..)
        | Expr::While(..)
        | Expr::Loop(..)
        | Expr::ForLoop(..)
        | Expr::Async(..)
        | Expr::TryBlock(..) => false,
        _ => true,
    }
}

mod convert {
    use super::*;

    // ExprMatch
    syn_trait_impl!(syn::ExprMatch);
    fn from_syn_arms(other: &[syn::Arm]) -> Vec<Arm> {
        let last = other.len().saturating_sub(1);
        other
            .iter()
            .enumerate()
            .map(|(i, other)| {
                let body = other.body.map_into();
                if i < last && requires_terminator(&*body) {
                    assert!(other.comma.is_some(), "expected `,`");
                }

                Arm {
                    attrs: other.attrs.map_into(),
                    pat: other.pat.ref_into(),
                    guard: other.guard.ref_map(|(_, x)| x.map_into()),
                    body,
                }
            })
            .collect()
    }
    impl From<&syn::ExprMatch> for ExprMatch {
        fn from(other: &syn::ExprMatch) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.map_into(),
                arms: from_syn_arms(&other.arms),
            }
        }
    }
    impl From<&ExprMatch> for syn::ExprMatch {
        fn from(other: &ExprMatch) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                match_token: default(),
                expr: other.expr.map_into(),
                brace_token: default(),
                arms: other.arms.map_into(),
            }
        }
    }

    // Arm
    syn_trait_impl!(syn::Arm);
    impl From<&syn::Arm> for Arm {
        fn from(other: &syn::Arm) -> Self {
            let body = other.body.map_into();
            if requires_terminator(&*body) {
                assert!(other.comma.is_some(), "expected `,`");
            }

            Self {
                attrs: other.attrs.map_into(),
                pat: other.pat.ref_into(),
                guard: other.guard.ref_map(|(_, x)| x.map_into()),
                body,
            }
        }
    }
    impl From<&Arm> for syn::Arm {
        fn from(other: &Arm) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                pat: other.pat.ref_into(),
                guard: other.guard.ref_map(|x| (default(), x.map_into())),
                fat_arrow_token: default(),
                body: other.body.map_into(),
                comma: default_or_none(requires_terminator(&*other.body)),
            }
        }
    }
}
