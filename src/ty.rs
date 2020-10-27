use super::*;

ast_enum! {
    /// The possible types that a Rust value could have.
    pub enum Type {
        /// A fixed size array type: `[T; n]`.
        Array(TypeArray),

        /// A bare function type: `fn(usize) -> bool`.
        BareFn(TypeBareFn),

        /// A type contained within invisible delimiters.
        Group(TypeGroup),

        /// An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
        /// a lifetime.
        ImplTrait(TypeImplTrait),

        /// Indication that a type should be inferred by the compiler: `_`.
        #[serde(rename = "_")]
        Infer,

        /// A macro in the type position.
        Macro(TypeMacro),

        /// The never type: `!`.
        #[serde(rename = "!")]
        Never,

        /// A parenthesized type equivalent to the inner type.
        Paren(TypeParen),

        /// A path like `std::slice::Iter`, optionally qualified with a
        /// self-type as in `<Vec<T> as SomeTrait>::Associated`.
        Path(TypePath),

        /// A raw pointer type: `*const T` or `*mut T`.
        Ptr(TypePtr),

        /// A reference type: `&'a T` or `&'a mut T`.
        Reference(TypeReference),

        /// A dynamically sized slice type: `[T]`.
        Slice(TypeSlice),

        /// A trait object type `Bound1 + Bound2 + Bound3` where `Bound` is a
        /// trait or a lifetime.
        TraitObject(TypeTraitObject),

        /// A tuple type: `(A, B, C, String)`.
        Tuple(TypeTuple),

        /// Tokens in type position not interpreted by Syn.
        Verbatim(TokenStream),

        #[doc(hidden)]
        __Nonexhaustive,
    }
}

ast_struct! {
    /// A fixed size array type: `[T; n]`.
    pub struct TypeArray {
        pub(crate) elem: Box<Type>,
        pub(crate) len: Expr,
    }
}

ast_struct! {
    /// A bare function type: `fn(usize) -> bool`.
    pub struct TypeBareFn {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) lifetimes: Option<BoundLifetimes>,
        #[serde(rename = "unsafe")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) unsafety: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) abi: Option<Abi>,
        pub(crate) inputs: Punctuated<BareFnArg>,
        pub(crate) variadic: Option<Variadic>,
        pub(crate) output: ReturnType,
    }
}

ast_struct! {
    /// A type contained within invisible delimiters.
    pub struct TypeGroup {
        pub(crate) elem: Box<Type>,
    }
}

ast_struct! {
    /// An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
    /// a lifetime.
    pub struct TypeImplTrait {
        pub(crate) bounds: Punctuated<TypeParamBound>,
    }
}

ast_struct! {
    /// A macro in the type position.
    pub struct TypeMacro {
        #[serde(flatten)]
        pub(crate) mac: Macro,
    }
}

ast_struct! {
    /// A parenthesized type equivalent to the inner type.
    pub struct TypeParen {
        pub(crate) elem: Box<Type>,
    }
}

ast_struct! {
    /// A path like `std::slice::Iter`, optionally qualified with a
    /// self-type as in `<Vec<T> as SomeTrait>::Associated`.
    pub struct TypePath {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) qself: Option<QSelf>,
        #[serde(flatten)]
        pub(crate) path: Path,
    }
}

ast_struct! {
    /// A raw pointer type: `*const T` or `*mut T`.
    pub struct TypePtr {
        #[serde(rename = "const")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) const_token: bool,
        #[serde(rename = "mut")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) mutability: bool,
        pub(crate) elem: Box<Type>,
    }
}

ast_struct! {
    /// A reference type: `&'a T` or `&'a mut T`.
    pub struct TypeReference {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) lifetime: Option<Lifetime>,
        #[serde(rename = "mut")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) mutability: bool,
        pub(crate) elem: Box<Type>,
    }
}

ast_struct! {
    /// A dynamically sized slice type: `[T]`.
    pub struct TypeSlice {
        pub(crate) elem: Box<Type>,
    }
}

ast_struct! {
    /// A trait object type `Bound1 + Bound2 + Bound3` where `Bound` is a
    /// trait or a lifetime.
    pub struct TypeTraitObject {
        #[serde(rename = "dyn")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) dyn_token: bool,
        pub(crate) bounds: Punctuated<TypeParamBound>,
    }
}

ast_struct! {
    /// A tuple type: `(A, B, C, String)`.
    pub struct TypeTuple {
        pub(crate) elems: Punctuated<Type>,
    }
}

ast_struct! {
    /// The binary interface of a function: `extern "C"`.
    pub struct Abi {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) name: Option<LitStr>,
    }
}

ast_struct! {
    /// An argument in a function type: the `usize` in `fn(usize) -> bool`.
    pub struct BareFnArg {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) name: Option<Ident>,
        pub(crate) ty: Type,
    }
}

ast_struct! {
    /// The variadic argument of a foreign function.
    pub struct Variadic {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
    }
}

ast_struct! {
    /// Return type of a function signature.
    #[derive(Default)]
    #[serde(transparent)]
    pub struct ReturnType {
        ty: Option<Box<Type>>,
    }
}

mod convert {
    use super::*;

    // ReturnType
    syn_trait_impl!(syn::ReturnType);
    impl From<&syn::ReturnType> for ReturnType {
        fn from(other: &syn::ReturnType) -> Self {
            use syn::ReturnType;
            match other {
                ReturnType::Default => Self { ty: None },
                ReturnType::Type(_, x) => Self { ty: Some(x.map_into()) },
            }
        }
    }
    impl From<&ReturnType> for syn::ReturnType {
        fn from(other: &ReturnType) -> Self {
            use syn::ReturnType;
            match &other.ty {
                None => ReturnType::Default,
                Some(x) => ReturnType::Type(default(), x.map_into()),
            }
        }
    }
}
