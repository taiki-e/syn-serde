use super::*;

#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::{
    ast_enum::{GenericParam, TraitBoundModifier, TypeParamBound, WherePredicate},
    ast_struct::{
        BoundLifetimes, ConstParam, PredicateEq, PredicateLifetime, TraitBound, WhereClause,
    },
};

ast_struct! {
    /// Lifetimes and type parameters attached to a declaration of a function,
    /// enum, trait, etc.
    #[derive(Default)]
    pub struct Generics {
        // #[serde(default, skip_serializing_if = "not")]
        // pub(crate) lt_token: bool,
        #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
        pub(crate) params: Punctuated<GenericParam>,
        // #[serde(default, skip_serializing_if = "not")]
        // pub(crate) gt_token: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) where_clause: Option<WhereClause>,
    }
}

impl Generics {
    pub(crate) fn is_none(&self) -> bool {
        self.params.is_empty() && self.where_clause.is_none() // && !self.lt_token && !self.gt_token
    }
}

ast_struct! {
    /// A generic type parameter: `T: Into<String>`.
    pub struct TypeParam {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) ident: Ident,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) colon_token: bool,
        #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
        pub(crate) bounds: Punctuated<TypeParamBound>,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) eq_token: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) default: Option<Type>,
    }
}

ast_struct! {
    /// A lifetime definition: `'a: 'b + 'c + 'd`.
    pub struct LifetimeDef {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) lifetime: Lifetime,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) colon_token: bool,
        #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
        pub(crate) bounds: Punctuated<Lifetime>,
    }
}

impl TraitBoundModifier {
    pub(crate) fn is_none(&self) -> bool {
        match self {
            TraitBoundModifier::None => true,
            TraitBoundModifier::Maybe => false,
        }
    }
}

impl Default for TraitBoundModifier {
    fn default() -> Self {
        TraitBoundModifier::None
    }
}

ast_struct! {
    /// A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.
    pub struct PredicateType {
        /// Any lifetimes from a `for` binding
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) lifetimes: Option<BoundLifetimes>,
        /// The type being bounded
        pub(crate) bounded_ty: Type,
        /// Trait and lifetime bounds (`Clone+Send+'static`)
        // TODO: should allow default?
        // #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
        pub(crate) bounds: Punctuated<TypeParamBound>,
    }
}

mod convert {
    use super::*;

    // Generics
    syn_trait_impl!(syn::Generics);
    impl From<&syn::Generics> for Generics {
        fn from(other: &syn::Generics) -> Self {
            // `ident ..>` or `ident <..`
            assert_eq!(other.lt_token.is_some(), other.gt_token.is_some());
            // `ident T`
            assert!(other.params.is_empty() || other.lt_token.is_some(), "expected `<`");

            Self { params: other.params.map_into(), where_clause: other.where_clause.map_into() }
        }
    }
    impl From<&Generics> for syn::Generics {
        fn from(other: &Generics) -> Self {
            Self {
                lt_token: default_or_none(!other.params.is_empty()),
                params: other.params.map_into(),
                gt_token: default_or_none(!other.params.is_empty()),
                where_clause: other.where_clause.map_into(),
            }
        }
    }
}
