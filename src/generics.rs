use super::*;

ast_struct! {
    /// Lifetimes and type parameters attached to a declaration of a function,
    /// enum, trait, etc.
    #[derive(Default)]
    pub struct Generics {
        // #[serde(default, skip_serializing_if = "not")]
        // pub lt_token: bool,
        #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
        pub params: Punctuated<GenericParam>,
        // #[serde(default, skip_serializing_if = "not")]
        // pub gt_token: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub where_clause: Option<WhereClause>,
    }
}

impl Generics {
    pub(crate) fn is_none(&self) -> bool {
        self.params.is_empty() && self.where_clause.is_none() // && !self.lt_token && !self.gt_token
    }
}

ast_enum_of_structs! {
    /// A generic type parameter, lifetime, or const generic: `T: Into<String>`,
    /// `'a: 'b`, `const LEN: usize`.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum GenericParam {
        /// A generic type parameter: `T: Into<String>`.
        pub Type(TypeParam {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub attrs: Vec<Attribute>,
            pub ident: Ident,
            #[serde(default, skip_serializing_if = "not")]
            pub colon_token: bool,
            #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
            pub bounds: Punctuated<TypeParamBound>,
            #[serde(default, skip_serializing_if = "not")]
            pub eq_token: bool,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub default: Option<Type>,
        }),

        /// A lifetime definition: `'a: 'b + 'c + 'd`.
        pub Lifetime(LifetimeDef {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub attrs: Vec<Attribute>,
            pub lifetime: Lifetime,
            #[serde(default, skip_serializing_if = "not")]
            pub colon_token: bool,
            #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
            pub bounds: Punctuated<Lifetime>,
        }),

        /// A const generic parameter: `const LENGTH: usize`.
        pub Const(ConstParam {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub attrs: Vec<Attribute>,
            pub ident: Ident,
            pub ty: Type,
            #[serde(default, skip_serializing_if = "not")]
            pub eq_token: bool,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub default: Option<Expr>,
        }),
    }
}

ast_struct! {
    /// A set of bound lifetimes: `for<'a, 'b, 'c>`.
    pub struct BoundLifetimes {
        pub lifetimes: Punctuated<LifetimeDef>,
    }
}

impl From<Lifetime> for LifetimeDef {
    fn from(lifetime: Lifetime) -> Self {
        Self {
            attrs: Vec::new(),
            lifetime,
            colon_token: false,
            bounds: Punctuated::new(),
        }
    }
}

impl From<Ident> for TypeParam {
    fn from(ident: Ident) -> Self {
        Self {
            attrs: vec![],
            ident,
            colon_token: false,
            bounds: Punctuated::new(),
            eq_token: false,
            default: None,
        }
    }
}

ast_enum_of_structs! {
    /// A trait or lifetime used as a bound on a type parameter.
    pub enum TypeParamBound {
        pub Trait(TraitBound),
        pub Lifetime(Lifetime),
    }
}

ast_struct! {
    /// A trait used as a bound on a type parameter.
    pub struct TraitBound {
        #[serde(default, skip_serializing_if = "not")]
        pub paren_token: bool,
        #[serde(default, skip_serializing_if = "TraitBoundModifier::is_none")]
        pub modifier: TraitBoundModifier,
        /// The `for<'a>` in `for<'a> Foo<&'a T>`
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lifetimes: Option<BoundLifetimes>,
        /// The `Foo<&'a T>` in `for<'a> Foo<&'a T>`
        pub path: Path,
    }
}

ast_enum! {
    /// A modifier on a trait bound, currently only used for the `?` in
    /// `?Sized`.
    pub enum TraitBoundModifier #manual_from_impl {
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
    pub struct WhereClause {
        pub predicates: Punctuated<WherePredicate>,
    }
}

ast_enum_of_structs! {
    /// A single predicate in a `where` clause: `T: Deserialize<'de>`.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum WherePredicate {
        /// A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.
        pub Type(PredicateType {
            /// Any lifetimes from a `for` binding
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub lifetimes: Option<BoundLifetimes>,
            /// The type being bounded
            pub bounded_ty: Type,
            /// Trait and lifetime bounds (`Clone+Send+'static`)
            pub bounds: Punctuated<TypeParamBound>,
        }),

        /// A lifetime predicate in a `where` clause: `'a: 'b + 'c`.
        pub Lifetime(PredicateLifetime {
            pub lifetime: Lifetime,
            pub bounds: Punctuated<Lifetime>,
        }),

        /// An equality predicate in a `where` clause (unsupported).
        pub Eq(PredicateEq {
            pub lhs_ty: Type,
            pub rhs_ty: Type,
        }),
    }
}

mod convert {
    use super::*;

    // Generics

    /* TODO: document
    /// # Panics
    ///
     */
    impl From<&syn::Generics> for Generics {
        fn from(other: &syn::Generics) -> Self {
            // `ident ..>` or `ident <..`
            assert_eq!(other.lt_token.is_some(), other.gt_token.is_some());
            // `ident T`
            assert!(
                other.params.is_empty() || other.lt_token.is_some(),
                "expected `<`"
            );

            Self {
                params: other.params.map_into(),
                where_clause: other.where_clause.map_into(),
            }
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

    // TypeParam

    impl From<&syn::TypeParam> for TypeParam {
        fn from(other: &syn::TypeParam) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                ident: other.ident.ref_into(),
                colon_token: other.colon_token.is_some(),
                bounds: other.bounds.map_into(),
                eq_token: other.eq_token.is_some(),
                default: other.default.map_into(),
            }
        }
    }

    impl From<&TypeParam> for syn::TypeParam {
        fn from(other: &TypeParam) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                ident: other.ident.ref_into(),
                colon_token: default_or_none(other.colon_token),
                bounds: other.bounds.map_into(),
                eq_token: default_or_none(other.eq_token),
                default: other.default.map_into(),
            }
        }
    }

    // LifetimeDef

    impl From<&syn::LifetimeDef> for LifetimeDef {
        fn from(other: &syn::LifetimeDef) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                lifetime: other.lifetime.ref_into(),
                colon_token: other.colon_token.is_some(),
                bounds: other.bounds.map_into(),
            }
        }
    }

    impl From<&LifetimeDef> for syn::LifetimeDef {
        fn from(other: &LifetimeDef) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                lifetime: other.lifetime.ref_into(),
                colon_token: default_or_none(other.colon_token),
                bounds: other.bounds.map_into(),
            }
        }
    }

    // ConstParam

    impl From<&syn::ConstParam> for ConstParam {
        fn from(other: &syn::ConstParam) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                ident: other.ident.ref_into(),
                ty: other.ty.ref_into(),
                eq_token: other.eq_token.is_some(),
                default: other.default.map_into(),
            }
        }
    }

    impl From<&ConstParam> for syn::ConstParam {
        fn from(other: &ConstParam) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                const_token: default(),
                ident: other.ident.ref_into(),
                colon_token: default(),
                ty: other.ty.ref_into(),
                eq_token: default_or_none(other.eq_token),
                default: other.default.map_into(),
            }
        }
    }

    // BoundLifetimes

    impl From<&syn::BoundLifetimes> for BoundLifetimes {
        fn from(other: &syn::BoundLifetimes) -> Self {
            Self {
                lifetimes: other.lifetimes.map_into(),
            }
        }
    }

    impl From<&BoundLifetimes> for syn::BoundLifetimes {
        fn from(other: &BoundLifetimes) -> Self {
            Self {
                for_token: default(),
                lt_token: default(),
                lifetimes: other.lifetimes.map_into(),
                gt_token: default(),
            }
        }
    }

    // TraitBound

    impl From<&syn::TraitBound> for TraitBound {
        fn from(other: &syn::TraitBound) -> Self {
            Self {
                paren_token: other.paren_token.is_some(),
                modifier: other.modifier.ref_into(),
                lifetimes: other.lifetimes.map_into(),
                path: other.path.ref_into(),
            }
        }
    }

    impl From<&TraitBound> for syn::TraitBound {
        fn from(other: &TraitBound) -> Self {
            Self {
                paren_token: default_or_none(other.paren_token),
                modifier: other.modifier.ref_into(),
                lifetimes: other.lifetimes.map_into(),
                path: other.path.ref_into(),
            }
        }
    }

    // TraitBoundModifier

    impl From<&syn::TraitBoundModifier> for TraitBoundModifier {
        fn from(other: &syn::TraitBoundModifier) -> Self {
            use super::TraitBoundModifier::*;
            use syn::TraitBoundModifier;
            match other {
                TraitBoundModifier::None => None,
                TraitBoundModifier::Maybe(_) => Maybe,
            }
        }
    }

    impl From<&TraitBoundModifier> for syn::TraitBoundModifier {
        fn from(other: &TraitBoundModifier) -> Self {
            use syn::TraitBoundModifier::*;
            match other {
                TraitBoundModifier::None => None,
                TraitBoundModifier::Maybe => Maybe(default()),
            }
        }
    }

    // WhereClause

    impl From<&syn::WhereClause> for WhereClause {
        fn from(other: &syn::WhereClause) -> Self {
            Self {
                predicates: other.predicates.map_into(),
            }
        }
    }

    impl From<&WhereClause> for syn::WhereClause {
        fn from(other: &WhereClause) -> Self {
            Self {
                where_token: default(),
                predicates: other.predicates.map_into(),
            }
        }
    }

    // PredicateType

    impl From<&syn::PredicateType> for PredicateType {
        fn from(other: &syn::PredicateType) -> Self {
            Self {
                lifetimes: other.lifetimes.map_into(),
                bounded_ty: other.bounded_ty.ref_into(),
                bounds: other.bounds.map_into(),
            }
        }
    }

    impl From<&PredicateType> for syn::PredicateType {
        fn from(other: &PredicateType) -> Self {
            Self {
                lifetimes: other.lifetimes.map_into(),
                bounded_ty: other.bounded_ty.ref_into(),
                colon_token: default(),
                bounds: other.bounds.map_into(),
            }
        }
    }

    // PredicateLifetime

    impl From<&syn::PredicateLifetime> for PredicateLifetime {
        fn from(other: &syn::PredicateLifetime) -> Self {
            Self {
                lifetime: other.lifetime.ref_into(),
                bounds: other.bounds.map_into(),
            }
        }
    }

    impl From<&PredicateLifetime> for syn::PredicateLifetime {
        fn from(other: &PredicateLifetime) -> Self {
            Self {
                lifetime: other.lifetime.ref_into(),
                colon_token: default(),
                bounds: other.bounds.map_into(),
            }
        }
    }

    // PredicateEq

    impl From<&syn::PredicateEq> for PredicateEq {
        fn from(other: &syn::PredicateEq) -> Self {
            Self {
                lhs_ty: other.lhs_ty.ref_into(),
                rhs_ty: other.rhs_ty.ref_into(),
            }
        }
    }

    impl From<&PredicateEq> for syn::PredicateEq {
        fn from(other: &PredicateEq) -> Self {
            Self {
                lhs_ty: other.lhs_ty.ref_into(),
                eq_token: default(),
                rhs_ty: other.rhs_ty.ref_into(),
            }
        }
    }
}
