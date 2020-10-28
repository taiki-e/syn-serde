use super::*;

#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::{
    ast_enum::Pat,
    ast_struct::{
        FieldPat, PatBox, PatIdent, PatLit, PatMacro, PatRange, PatReference, PatRest, PatSlice,
        PatStruct, PatTuple, PatTupleStruct, PatType, PatWild,
    },
};

ast_struct! {
    /// A pattern that matches any one of a set of cases.
    pub struct PatOr {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        // TODO: can remove
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) leading_vert: bool,
        pub(crate) cases: Punctuated<Pat>,
    }
}

ast_struct! {
    /// A path pattern like `Color::Red`, optionally qualified with a
    /// self-type.
    ///
    /// Unqualified path patterns can legally refer to variants, structs,
    /// constants or associated constants. Qualified path patterns like
    /// `<A>::B::C` and `<A as Trait>::B::C` can only legally refer to
    /// associated constants.
    pub struct PatPath {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) qself: Option<QSelf>,
        #[serde(flatten)]
        pub(crate) path: Path,
    }
}
