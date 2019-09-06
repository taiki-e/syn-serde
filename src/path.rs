use super::*;

ast_struct! {
    /// A path at which a named item is exported: `std::collections::HashMap`.
    pub struct Path {
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) leading_colon: bool,
        pub(crate) segments: Punctuated<PathSegment>,
    }
}

ast_struct! {
    /// A segment of a path together with any path arguments on that segment.
    pub struct PathSegment {
        pub(crate) ident: Ident,
        #[serde(default, skip_serializing_if = "PathArguments::is_none")]
        pub(crate) arguments: PathArguments,
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
    pub enum PathArguments {
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
    fn is_none(&self) -> bool {
        match self {
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
        pub(crate) colon2_token: bool,
        pub(crate) args: Punctuated<GenericArgument>,
    }
}

ast_struct! {
    /// A binding (equality constraint) on an associated type: `Item = u8`.
    pub struct Binding {
        pub(crate) ident: Ident,
        pub(crate) ty: Type,
    }
}

ast_struct! {
    /// An associated type bound: `Iterator<Item: Display>`.
    pub struct Constraint {
        pub(crate) ident: Ident,
        pub(crate) bounds: Punctuated<TypeParamBound>,
    }
}

ast_struct! {
    /// Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) ->
    /// C`.
    pub struct ParenthesizedGenericArguments {
        /// `(A, B)`
        pub(crate) inputs: Punctuated<Type>,
        /// `C`
        #[serde(default)]
        pub(crate) output: ReturnType,
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
        pub(crate) ty: Box<Type>,
        pub(crate) position: usize,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) as_token: bool,
    }
}
