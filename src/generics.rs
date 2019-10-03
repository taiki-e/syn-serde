use super::*;

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

ast_enum! {
    /// A generic type parameter, lifetime, or const generic: `T: Into<String>`,
    /// `'a: 'b`, `const LEN: usize`.
    pub enum GenericParam {
        /// A generic type parameter: `T: Into<String>`.
        Type(TypeParam),

        /// A lifetime definition: `'a: 'b + 'c + 'd`.
        Lifetime(LifetimeDef),

        /// A const generic parameter: `const LENGTH: usize`.
        Const(ConstParam),
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

ast_struct! {
    /// A const generic parameter: `const LENGTH: usize`.
    pub struct ConstParam {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) ident: Ident,
        pub(crate) ty: Type,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) eq_token: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) default: Option<Expr>,
    }
}

ast_struct! {
    /// A set of bound lifetimes: `for<'a, 'b, 'c>`.
    #[derive(Default)]
    #[serde(transparent)]
    pub struct BoundLifetimes {
        pub(crate) lifetimes: Punctuated<LifetimeDef>,
    }
}

ast_enum! {
    /// A trait or lifetime used as a bound on a type parameter.
    pub enum TypeParamBound {
        Trait(TraitBound),
        Lifetime(Lifetime),
    }
}

ast_struct! {
    /// A trait used as a bound on a type parameter.
    pub struct TraitBound {
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) paren_token: bool,
        #[serde(default, skip_serializing_if = "TraitBoundModifier::is_none")]
        pub(crate) modifier: TraitBoundModifier,
        /// The `for<'a>` in `for<'a> Foo<&'a T>`
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) lifetimes: Option<BoundLifetimes>,
        /// The `Foo<&'a T>` in `for<'a> Foo<&'a T>`
        pub(crate) path: Path,
    }
}

ast_enum! {
    /// A modifier on a trait bound, currently only used for the `?` in
    /// `?Sized`.
    pub enum TraitBoundModifier {
        None,
        Maybe,
    }
}

impl TraitBoundModifier {
    fn is_none(&self) -> bool {
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
    /// A `where` clause in a definition: `where T: Deserialize<'de>, D:
    /// 'static`.
    #[serde(transparent)]
    pub struct WhereClause {
        pub(crate) predicates: Punctuated<WherePredicate>,
    }
}

ast_enum! {
    /// A single predicate in a `where` clause: `T: Deserialize<'de>`.
    pub enum WherePredicate {
        /// A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.
        Type(PredicateType),

        /// A lifetime predicate in a `where` clause: `'a: 'b + 'c`.
        Lifetime(PredicateLifetime),

        /// An equality predicate in a `where` clause (unsupported).
        Eq(PredicateEq),
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
        pub(crate) bounds: Punctuated<TypeParamBound>,
    }
}

ast_struct! {
    /// A lifetime predicate in a `where` clause: `'a: 'b + 'c`.
    pub struct PredicateLifetime {
        pub(crate) lifetime: Lifetime,
        pub(crate) bounds: Punctuated<Lifetime>,
    }
}

ast_struct! {
    /// An equality predicate in a `where` clause (unsupported).
    pub struct PredicateEq {
        pub(crate) lhs_ty: Type,
        pub(crate) rhs_ty: Type,
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
