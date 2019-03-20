use super::*;

ast_struct! {
    /// A path at which a named item is exported: `std::collections::HashMap`.
    pub struct Path {
        #[serde(default, skip_serializing_if = "not")]
        leading_colon: bool,
        segments: Punctuated<PathSegment>,
    }
}

impl<T> From<T> for Path
where
    T: Into<PathSegment>,
{
    fn from(segment: T) -> Self {
        let mut path = Self {
            leading_colon: false,
            segments: Punctuated::new(),
        };
        path.segments.push(segment.into());
        path
    }
}

ast_struct! {
    /// A segment of a path together with any path arguments on that segment.
    pub struct PathSegment {
        ident: Ident,
        #[serde(default, skip_serializing_if = "PathArguments::is_none")]
        arguments: PathArguments,
    }
}

impl<T> From<T> for PathSegment
where
    T: Into<Ident>,
{
    fn from(ident: T) -> Self {
        Self {
            ident: ident.into(),
            arguments: PathArguments::None,
        }
    }
}

ast_enum! {
    /// Angle bracketed or parenthesized arguments of a path segment.
    ///
    /// ## Angle bracketed
    ///
    /// The `<'a, T>` in `std::slice::iter<'a, T>`.
    ///
    /// ## Parenthesized
    ///
    /// The `(A, B) -> C` in `Fn(A, B) -> C`.
    pub enum PathArguments #manual_from_impl {
        None,
        /// The `<'a, T>` in `std::slice::iter<'a, T>`.
        AngleBracketed(AngleBracketedGenericArguments),
        /// The `(A, B) -> C` in `Fn(A, B) -> C`.
        Parenthesized(ParenthesizedGenericArguments),
    }
}

impl Default for PathArguments {
    fn default() -> Self {
        PathArguments::None
    }
}

impl PathArguments {
    pub fn is_empty(&self) -> bool {
        match *self {
            PathArguments::None => true,
            PathArguments::AngleBracketed(ref bracketed) => bracketed.args.is_empty(),
            PathArguments::Parenthesized(_) => false,
        }
    }

    fn is_none(&self) -> bool {
        match *self {
            PathArguments::None => true,
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => false,
        }
    }
}

ast_enum! {
    /// An individual generic argument, like `'a`, `T`, or `Item = T`.
    pub enum GenericArgument {
        /// A lifetime argument.
        Lifetime(Lifetime),
        /// A type argument.
        Type(Type),
        /// A binding (equality constraint) on an associated type: the `Item =
        /// u8` in `Iterator<Item = u8>`.
        Binding(Binding),
        /// An associated type bound: `Iterator<Item: Display>`.
        Constraint(Constraint),
        /// A const expression. Must be inside of a block.
        ///
        /// NOTE: Identity expressions are represented as Type arguments, as
        /// they are indistinguishable syntactically.
        Const(Expr),
    }
}

ast_struct! {
    /// Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K,
    /// V>`.
    pub struct AngleBracketedGenericArguments {
        #[serde(default, skip_serializing_if = "not")]
        colon2_token: bool,
        args: Punctuated<GenericArgument>,
    }
}

ast_struct! {
    /// A binding (equality constraint) on an associated type: `Item = u8`.
    pub struct Binding {
        ident: Ident,
        ty: Type,
    }
}

ast_struct! {
    /// An associated type bound: `Iterator<Item: Display>`.
    pub struct Constraint {
        ident: Ident,
        bounds: Punctuated<TypeParamBound>,
    }
}

ast_struct! {
    /// Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) ->
    /// C`.
    pub struct ParenthesizedGenericArguments {
        /// `(A, B)`
        inputs: Punctuated<Type>,
        /// `C`
        output: ReturnType,
    }
}

ast_struct! {
    /// The explicit Self type in a qualified path: the `T` in `<T as
    /// Display>::fmt`.
    ///
    /// The actual path, including the trait and the associated item, is stored
    /// separately. The `position` field represents the index of the associated
    /// item qualified with this Self type.
    ///
    /// ```text
    /// <Vec<T> as a::b::Trait>::AssociatedItem
    ///  ^~~~~~    ~~~~~~~~~~~~~~^
    ///  ty        position = 3
    ///
    /// <Vec<T>>::AssociatedItem
    ///  ^~~~~~   ^
    ///  ty       position = 0
    /// ```
    pub struct QSelf {
        ty: Box<Type>,
        position: usize,
        #[serde(default, skip_serializing_if = "not")]
        as_token: bool,
    }
}

mod convert {
    use super::*;

    // Path

    impl From<&syn::Path> for Path {
        fn from(other: &syn::Path) -> Self {
            Self {
                leading_colon: other.leading_colon.is_some(),
                segments: other.segments.map_into(),
            }
        }
    }

    impl From<&Path> for syn::Path {
        fn from(other: &Path) -> Self {
            Self {
                leading_colon: default_or_none(other.leading_colon),
                segments: other.segments.map_into(),
            }
        }
    }

    // PathSegment

    impl From<&syn::PathSegment> for PathSegment {
        fn from(other: &syn::PathSegment) -> Self {
            Self {
                ident: other.ident.ref_into(),
                arguments: other.arguments.ref_into(),
            }
        }
    }

    impl From<&PathSegment> for syn::PathSegment {
        fn from(other: &PathSegment) -> Self {
            Self {
                ident: other.ident.ref_into(),
                arguments: other.arguments.ref_into(),
            }
        }
    }

    // PathArguments

    impl From<&syn::PathArguments> for PathArguments {
        fn from(other: &syn::PathArguments) -> Self {
            use super::PathArguments::*;
            use syn::PathArguments;
            match other {
                PathArguments::None => None,
                PathArguments::AngleBracketed(x) => AngleBracketed(x.into()),
                PathArguments::Parenthesized(x) => Parenthesized(x.into()),
            }
        }
    }

    impl From<&PathArguments> for syn::PathArguments {
        fn from(other: &PathArguments) -> Self {
            use syn::PathArguments::*;
            match other {
                PathArguments::None => None,
                PathArguments::AngleBracketed(x) => AngleBracketed(x.into()),
                PathArguments::Parenthesized(x) => Parenthesized(x.into()),
            }
        }
    }

    // AngleBracketedGenericArguments

    impl From<&syn::AngleBracketedGenericArguments> for AngleBracketedGenericArguments {
        fn from(other: &syn::AngleBracketedGenericArguments) -> Self {
            Self {
                colon2_token: other.colon2_token.is_some(),
                args: other.args.map_into(),
            }
        }
    }

    impl From<&AngleBracketedGenericArguments> for syn::AngleBracketedGenericArguments {
        fn from(other: &AngleBracketedGenericArguments) -> Self {
            Self {
                colon2_token: default_or_none(other.colon2_token),
                lt_token: default(),
                args: other.args.map_into(),
                gt_token: default(),
            }
        }
    }

    // Binding

    impl From<&syn::Binding> for Binding {
        fn from(other: &syn::Binding) -> Self {
            Self {
                ident: other.ident.ref_into(),
                ty: other.ty.ref_into(),
            }
        }
    }

    impl From<&Binding> for syn::Binding {
        fn from(other: &Binding) -> Self {
            Self {
                ident: other.ident.ref_into(),
                eq_token: default(),
                ty: other.ty.ref_into(),
            }
        }
    }

    // Constraint

    impl From<&syn::Constraint> for Constraint {
        fn from(other: &syn::Constraint) -> Self {
            Self {
                ident: other.ident.ref_into(),
                bounds: other.bounds.map_into(),
            }
        }
    }

    impl From<&Constraint> for syn::Constraint {
        fn from(other: &Constraint) -> Self {
            Self {
                ident: other.ident.ref_into(),
                colon_token: default(),
                bounds: other.bounds.map_into(),
            }
        }
    }

    // ParenthesizedGenericArguments

    impl From<&syn::ParenthesizedGenericArguments> for ParenthesizedGenericArguments {
        fn from(other: &syn::ParenthesizedGenericArguments) -> Self {
            Self {
                inputs: other.inputs.map_into(),
                output: other.output.ref_into(),
            }
        }
    }

    impl From<&ParenthesizedGenericArguments> for syn::ParenthesizedGenericArguments {
        fn from(other: &ParenthesizedGenericArguments) -> Self {
            Self {
                paren_token: default(),
                inputs: other.inputs.map_into(),
                output: other.output.ref_into(),
            }
        }
    }

    // QSelf

    impl From<&syn::QSelf> for QSelf {
        fn from(other: &syn::QSelf) -> Self {
            Self {
                ty: other.ty.map_into(),
                position: other.position,
                as_token: other.as_token.is_some(),
            }
        }
    }

    impl From<&QSelf> for syn::QSelf {
        fn from(other: &QSelf) -> Self {
            Self {
                lt_token: default(),
                ty: other.ty.map_into(),
                position: other.position,
                as_token: default_or_none(other.as_token),
                gt_token: default(),
            }
        }
    }
}
