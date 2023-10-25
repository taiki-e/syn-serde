// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::*;
#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::{
    ast_enum::{Expr, Member, RangeLimits},
    ast_struct::{
        ExprArray, ExprAssign, ExprAsync, ExprAwait, ExprBinary, ExprBlock, ExprBreak, ExprCall,
        ExprCast, ExprClosure, ExprConst, ExprContinue, ExprField, ExprForLoop, ExprGroup, ExprIf,
        ExprIndex, ExprInfer, ExprLet, ExprLit, ExprLoop, ExprMacro, ExprMatch, ExprMethodCall,
        ExprParen, ExprPath, ExprRange, ExprReference, ExprRepeat, ExprReturn, ExprStruct, ExprTry,
        ExprTryBlock, ExprTuple, ExprUnary, ExprUnsafe, ExprWhile, ExprYield, FieldValue, Index,
        Label,
    },
};

ast_struct! {
    /// An adapter for [`struct@syn::Arm`].
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

// https://github.com/dtolnay/syn/blob/2.0.15/src/expr.rs#L913
pub(crate) fn requires_terminator(expr: &Expr) -> bool {
    // see https://github.com/rust-lang/rust/blob/9a19e7604/compiler/rustc_ast/src/util/classify.rs#L7-L26
    match expr {
        Expr::If(_)
        | Expr::Match(_)
        | Expr::Block(_) | Expr::Unsafe(_) // both under ExprKind::Block in rustc
        | Expr::While(_)
        | Expr::Loop(_)
        | Expr::ForLoop(_)
        | Expr::TryBlock(_)
        | Expr::Const(_) => false,
        Expr::Array(_)
        | Expr::Assign(_)
        | Expr::Async(_)
        | Expr::Await(_)
        | Expr::Binary(_)
        | Expr::Break(_)
        | Expr::Call(_)
        | Expr::Cast(_)
        | Expr::Closure(_)
        | Expr::Continue(_)
        | Expr::Field(_)
        | Expr::Group(_)
        | Expr::Index(_)
        | Expr::Infer(_)
        | Expr::Let(_)
        | Expr::Lit(_)
        | Expr::Macro(_)
        | Expr::MethodCall(_)
        | Expr::Paren(_)
        | Expr::Path(_)
        | Expr::Range(_)
        | Expr::Reference(_)
        | Expr::Repeat(_)
        | Expr::Return(_)
        | Expr::Struct(_)
        | Expr::Try(_)
        | Expr::Tuple(_)
        | Expr::Unary(_)
        | Expr::Yield(_)
        | Expr::Verbatim(_) => true
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
