use super::*;
#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::{
    ast_enum::{GenericParam, TraitBoundModifier, TypeParamBound, WherePredicate},
    ast_struct::{
        BoundLifetimes, ConstParam, LifetimeParam, PredicateLifetime, TraitBound, TypeParam,
        WhereClause,
    },
};

ast_struct! {
    /// An adapter for [`struct@syn::Generics`].
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
    /// An adapter for [`struct@syn::PredicateType`].
    pub struct PredicateType {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) lifetimes: Option<BoundLifetimes>,
        pub(crate) bounded_ty: Type,
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
