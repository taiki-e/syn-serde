#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::{
    ast_enum::{GenericArgument, PathArguments},
    ast_struct::{
        AngleBracketedGenericArguments, AssocConst, AssocType, Constraint,
        ParenthesizedGenericArguments, Path, PathSegment, QSelf,
    },
};

impl Default for PathArguments {
    fn default() -> Self {
        PathArguments::None
    }
}

impl PathArguments {
    pub(crate) fn is_none(&self) -> bool {
        match self {
            PathArguments::None => true,
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => false,
        }
    }
}
