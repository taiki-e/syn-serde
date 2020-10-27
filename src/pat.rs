use super::*;

#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::ast_enum::Pat;

ast_struct! {
    /// A box pattern: `box v`.
    pub struct PatBox {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) pat: Box<Pat>,
    }
}

ast_struct! {
    /// A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.
    pub struct PatIdent {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(rename = "ref")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) by_ref: bool,
        #[serde(rename = "mut")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) mutability: bool,
        pub(crate) ident: Ident,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) subpat: Option<Box<Pat>>,
    }
}

ast_struct! {
    /// A literal pattern: `0`.
    ///
    /// This holds an `Expr` rather than a `Lit` because negative numbers
    /// are represented as an `Expr::Unary`.
    pub struct PatLit {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) expr: Box<Expr>,
    }
}

ast_struct! {
    /// A macro in pattern position.
    pub struct PatMacro {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(flatten)]
        pub(crate) mac: Macro,
    }
}

ast_struct! {
    /// A pattern that matches any one of a set of cases.
    pub struct PatOr {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
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

ast_struct! {
    /// A range pattern: `1..=2`.
    pub struct PatRange {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) lo: Box<Expr>,
        pub(crate) limits: RangeLimits,
        pub(crate) hi: Box<Expr>,
    }
}

ast_struct! {
    /// A reference pattern: `&mut var`.
    pub struct PatReference {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(rename = "mut")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) mutability: bool,
        pub(crate) pat: Box<Pat>,
    }
}

ast_struct! {
    /// The dots in a tuple or slice pattern: `[0, 1, ..]`
    pub struct PatRest {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
    }
}

ast_struct! {
    /// A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.
    pub struct PatSlice {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) elems: Punctuated<Pat>,
    }
}

ast_struct! {
    /// A struct or struct variant pattern: `Variant { x, y, .. }`.
    pub struct PatStruct {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) path: Path,
        pub(crate) fields: Punctuated<FieldPat>,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) dot2_token: bool,
    }
}

ast_struct! {
    /// A tuple pattern: `(a, b)`.
    pub struct PatTuple {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) elems: Punctuated<Pat>,
    }
}

ast_struct! {
    /// A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.
    pub struct PatTupleStruct {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) path: Path,
        pub(crate) pat: PatTuple,
    }
}

ast_struct! {
    /// A type ascription pattern: `foo: f64`.
    pub struct PatType {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) pat: Box<Pat>,
        pub(crate) ty: Box<Type>,
    }
}

ast_struct! {
    /// A pattern that matches any value: `_`.
    pub struct PatWild {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
    }
}

ast_struct! {
    /// A single field in a struct pattern.
    ///
    /// Patterns like the fields of Foo `{ x, ref y, ref mut z }` are treated
    /// the same as `x: x, y: ref y, z: ref mut z` but there is no colon token.
    pub struct FieldPat {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(flatten)]
        pub(crate) member: Member,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) colon_token: bool,
        pub(crate) pat: Box<Pat>,
    }
}
