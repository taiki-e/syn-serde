use super::*;
#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::{
    ast_enum::{Expr, GenericMethodArgument, Member, RangeLimits},
    ast_struct::{
        ExprArray, ExprAssign, ExprAssignOp, ExprAsync, ExprAwait, ExprBinary, ExprBlock, ExprBox,
        ExprBreak, ExprCall, ExprCast, ExprClosure, ExprContinue, ExprField, ExprForLoop,
        ExprGroup, ExprIf, ExprIndex, ExprLet, ExprLit, ExprLoop, ExprMacro, ExprMatch,
        ExprMethodCall, ExprParen, ExprPath, ExprRange, ExprReference, ExprRepeat, ExprReturn,
        ExprStruct, ExprTry, ExprTryBlock, ExprTuple, ExprType, ExprUnary, ExprUnsafe, ExprWhile,
        ExprYield, FieldValue, Index, Label, MethodTurbofish,
    },
};

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
                if i < last && requires_terminator(&body) {
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
            if requires_terminator(&body) {
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
                comma: default_or_none(requires_terminator(&other.body)),
            }
        }
    }
}
