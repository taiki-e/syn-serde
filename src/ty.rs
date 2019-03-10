use super::*;

ast_enum_of_structs! {
    /// The possible types that a Rust value could have.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum Type #manual_from_impl {
        /// A dynamically sized slice type: `[T]`.
        pub Slice(TypeSlice {
            pub elem: Box<Type>,
        }),

        /// A fixed size array type: `[T; n]`.
        pub Array(TypeArray {
            pub elem: Box<Type>,
            pub len: Expr,
        }),

        /// A raw pointer type: `*const T` or `*mut T`.
        pub Ptr(TypePtr {
            #[serde(rename = "const")]
            #[serde(default, skip_serializing_if = "not")]
            pub const_token: bool,
            #[serde(rename = "mut")]
            #[serde(default, skip_serializing_if = "not")]
            pub mutability: bool,
            pub elem: Box<Type>,
        }),

        /// A reference type: `&'a T` or `&'a mut T`.
        pub Reference(TypeReference {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub lifetime: Option<Lifetime>,
            #[serde(rename = "mut")]
            #[serde(default, skip_serializing_if = "not")]
            pub mutability: bool,
            pub elem: Box<Type>,
        }),

        /// A bare function type: `fn(usize) -> bool`.
        pub BareFn(TypeBareFn {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub lifetimes: Option<BoundLifetimes>,
            #[serde(rename = "unsafe")]
            #[serde(default, skip_serializing_if = "not")]
            pub unsafety: bool,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub abi: Option<Abi>,
            pub inputs: Punctuated<BareFnArg>,
            #[serde(default, skip_serializing_if = "not")]
            pub variadic: bool,
            pub output: ReturnType,
        }),

        /// The never type: `!`.
        #[serde(rename = "!")]
        pub Never,

        /// A tuple type: `(A, B, C, String)`.
        pub Tuple(TypeTuple {
            pub elems: Punctuated<Type>,
        }),

        /// A path like `std::slice::Iter`, optionally qualified with a
        /// self-type as in `<Vec<T> as SomeTrait>::Associated`.
        ///
        /// Type arguments are stored in the Path itself.
        pub Path(TypePath {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub qself: Option<QSelf>,
            #[serde(flatten)]
            pub path: Path,
        }),

        /// A trait object type `Bound1 + Bound2 + Bound3` where `Bound` is a
        /// trait or a lifetime.
        pub TraitObject(TypeTraitObject {
            pub bounds: Punctuated<TypeParamBound>,
        }),

        /// An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
        /// a lifetime.
        pub ImplTrait(TypeImplTrait {
            pub bounds: Punctuated<TypeParamBound>,
        }),

        /// A parenthesized type equivalent to the inner type.
        pub Paren(TypeParen {
            pub elem: Box<Type>,
        }),

        /// A type contained within invisible delimiters.
        pub Group(TypeGroup {
            pub elem: Box<Type>,
        }),

        /// Indication that a type should be inferred by the compiler: `_`.
        #[serde(rename = "_")]
        pub Infer,

        /// A macro in the type position.
        pub Macro(TypeMacro {
            #[serde(flatten)]
            pub mac: Macro,
        }),

        /// Tokens in type position not interpreted by Syn.
        pub Verbatim(TypeVerbatim {
            pub tts: TokenStream,
        }),
    }
}

ast_struct! {
    /// The binary interface of a function: `extern "C"`.
    pub struct Abi {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<LitStr>,
    }
}

ast_struct! {
    /// An argument in a function type: the `usize` in `fn(usize) -> bool`.
    pub struct BareFnArg {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<BareFnArgName>,
        pub ty: Type,
    }
}

ast_enum! {
    /// Name of an argument in a function type: the `n` in `fn(n: usize)`.
    pub enum BareFnArgName #manual_from_impl {
        /// Argument given a name.
        Named(Ident),
        /// Argument not given a name, matched with `_`.
        #[serde(rename = "_")]
        Wild,
    }
}

ast_struct! {
    /// Return type of a function signature.
    #[derive(Default)]
    #[serde(transparent)]
    pub struct ReturnType {
        pub ty: Option<Box<Type>>,
    }
}

impl ReturnType {
    pub(crate) fn is_none(&self) -> bool {
        self.ty.is_none()
    }
}

mod convert {
    use super::*;

    // Type

    impl From<&syn::Type> for Type {
        fn from(other: &syn::Type) -> Self {
            use super::Type::*;
            use syn::Type;
            match other {
                Type::Slice(x) => Slice(x.into()),
                Type::Array(x) => Array(x.into()),
                Type::Ptr(x) => Ptr(x.into()),
                Type::Reference(x) => Reference(x.into()),
                Type::BareFn(x) => BareFn(x.into()),
                Type::Never(_) => Never,
                Type::Tuple(x) => Tuple(x.into()),
                Type::Path(x) => Path(x.into()),
                Type::TraitObject(x) => TraitObject(x.into()),
                Type::ImplTrait(x) => ImplTrait(x.into()),
                Type::Paren(x) => Paren(x.into()),
                Type::Group(x) => Group(x.into()),
                Type::Infer(_) => Infer,
                Type::Macro(x) => Macro(x.into()),
                Type::Verbatim(x) => Verbatim(x.into()),
            }
        }
    }

    impl From<&Type> for syn::Type {
        fn from(other: &Type) -> Self {
            use syn::Type::*;
            match other {
                Type::Slice(x) => Slice(x.into()),
                Type::Array(x) => Array(x.into()),
                Type::Ptr(x) => Ptr(x.into()),
                Type::Reference(x) => Reference(x.into()),
                Type::BareFn(x) => BareFn(x.into()),
                Type::Never => Never(syn::TypeNever {
                    bang_token: default(),
                }),
                Type::Tuple(x) => Tuple(x.into()),
                Type::Path(x) => Path(x.into()),
                Type::TraitObject(x) => TraitObject(x.into()),
                Type::ImplTrait(x) => ImplTrait(x.into()),
                Type::Paren(x) => Paren(x.into()),
                Type::Group(x) => Group(x.into()),
                Type::Infer => Infer(syn::TypeInfer {
                    underscore_token: default(),
                }),
                Type::Macro(x) => Macro(x.into()),
                Type::Verbatim(x) => Verbatim(x.into()),
            }
        }
    }

    // TypeSlice

    impl From<&syn::TypeSlice> for TypeSlice {
        fn from(other: &syn::TypeSlice) -> Self {
            Self {
                elem: other.elem.map_into(),
            }
        }
    }

    impl From<&TypeSlice> for syn::TypeSlice {
        fn from(other: &TypeSlice) -> Self {
            Self {
                bracket_token: default(),
                elem: other.elem.map_into(),
            }
        }
    }

    // TypeArray

    impl From<&syn::TypeArray> for TypeArray {
        fn from(other: &syn::TypeArray) -> Self {
            Self {
                elem: other.elem.map_into(),
                len: other.len.ref_into(),
            }
        }
    }

    impl From<&TypeArray> for syn::TypeArray {
        fn from(other: &TypeArray) -> Self {
            Self {
                bracket_token: default(),
                elem: other.elem.map_into(),
                semi_token: default(),
                len: other.len.ref_into(),
            }
        }
    }

    // TypePtr

    impl From<&syn::TypePtr> for TypePtr {
        fn from(other: &syn::TypePtr) -> Self {
            Self {
                const_token: other.const_token.is_some(),
                mutability: other.mutability.is_some(),
                elem: other.elem.map_into(),
            }
        }
    }

    impl From<&TypePtr> for syn::TypePtr {
        fn from(other: &TypePtr) -> Self {
            Self {
                star_token: default(),
                const_token: default_or_none(other.const_token),
                mutability: default_or_none(other.mutability),
                elem: other.elem.map_into(),
            }
        }
    }

    // TypeReference

    impl From<&syn::TypeReference> for TypeReference {
        fn from(other: &syn::TypeReference) -> Self {
            Self {
                lifetime: other.lifetime.map_into(),
                mutability: other.mutability.is_some(),
                elem: other.elem.map_into(),
            }
        }
    }

    impl From<&TypeReference> for syn::TypeReference {
        fn from(other: &TypeReference) -> Self {
            Self {
                and_token: default(),
                lifetime: other.lifetime.map_into(),
                mutability: default_or_none(other.mutability),
                elem: other.elem.map_into(),
            }
        }
    }

    // TypeBareFn

    impl From<&syn::TypeBareFn> for TypeBareFn {
        fn from(other: &syn::TypeBareFn) -> Self {
            Self {
                lifetimes: other.lifetimes.map_into(),
                unsafety: other.unsafety.is_some(),
                abi: other.abi.map_into(),
                inputs: other.inputs.map_into(),
                variadic: other.unsafety.is_some(),
                output: other.output.ref_into(),
            }
        }
    }

    impl From<&TypeBareFn> for syn::TypeBareFn {
        fn from(other: &TypeBareFn) -> Self {
            Self {
                lifetimes: other.lifetimes.map_into(),
                unsafety: default_or_none(other.unsafety),
                abi: other.abi.map_into(),
                fn_token: default(),
                paren_token: default(),
                inputs: other.inputs.map_into(),
                variadic: default_or_none(other.variadic),
                output: other.output.ref_into(),
            }
        }
    }

    // TypeTuple

    impl From<&syn::TypeTuple> for TypeTuple {
        fn from(other: &syn::TypeTuple) -> Self {
            Self {
                elems: other.elems.map_into(),
            }
        }
    }

    impl From<&TypeTuple> for syn::TypeTuple {
        fn from(other: &TypeTuple) -> Self {
            Self {
                paren_token: default(),
                elems: other.elems.map_into(),
            }
        }
    }

    // TypePath

    impl From<&syn::TypePath> for TypePath {
        fn from(other: &syn::TypePath) -> Self {
            Self {
                qself: other.qself.map_into(),
                path: other.path.ref_into(),
            }
        }
    }

    impl From<&TypePath> for syn::TypePath {
        fn from(other: &TypePath) -> Self {
            Self {
                qself: other.qself.map_into(),
                path: other.path.ref_into(),
            }
        }
    }

    // TypeTraitObject

    impl From<&syn::TypeTraitObject> for TypeTraitObject {
        fn from(other: &syn::TypeTraitObject) -> Self {
            Self {
                bounds: other.bounds.map_into(),
            }
        }
    }

    impl From<&TypeTraitObject> for syn::TypeTraitObject {
        fn from(other: &TypeTraitObject) -> Self {
            Self {
                dyn_token: default(),
                bounds: other.bounds.map_into(),
            }
        }
    }

    // TypeImplTrait

    impl From<&syn::TypeImplTrait> for TypeImplTrait {
        fn from(other: &syn::TypeImplTrait) -> Self {
            Self {
                bounds: other.bounds.map_into(),
            }
        }
    }

    impl From<&TypeImplTrait> for syn::TypeImplTrait {
        fn from(other: &TypeImplTrait) -> Self {
            Self {
                impl_token: default(),
                bounds: other.bounds.map_into(),
            }
        }
    }

    // TypeParen

    impl From<&syn::TypeParen> for TypeParen {
        fn from(other: &syn::TypeParen) -> Self {
            Self {
                elem: other.elem.map_into(),
            }
        }
    }

    impl From<&TypeParen> for syn::TypeParen {
        fn from(other: &TypeParen) -> Self {
            Self {
                paren_token: default(),
                elem: other.elem.map_into(),
            }
        }
    }

    // TypeGroup

    impl From<&syn::TypeGroup> for TypeGroup {
        fn from(other: &syn::TypeGroup) -> Self {
            Self {
                elem: other.elem.map_into(),
            }
        }
    }

    impl From<&TypeGroup> for syn::TypeGroup {
        fn from(other: &TypeGroup) -> Self {
            Self {
                group_token: default(),
                elem: other.elem.map_into(),
            }
        }
    }

    // TypeMacro

    impl From<&syn::TypeMacro> for TypeMacro {
        fn from(other: &syn::TypeMacro) -> Self {
            Self {
                mac: other.mac.ref_into(),
            }
        }
    }

    impl From<&TypeMacro> for syn::TypeMacro {
        fn from(other: &TypeMacro) -> Self {
            Self {
                mac: other.mac.ref_into(),
            }
        }
    }

    // TypeVerbatim

    impl From<&syn::TypeVerbatim> for TypeVerbatim {
        fn from(other: &syn::TypeVerbatim) -> Self {
            Self {
                tts: other.tts.ref_into(),
            }
        }
    }

    impl From<&TypeVerbatim> for syn::TypeVerbatim {
        fn from(other: &TypeVerbatim) -> Self {
            Self {
                tts: other.tts.ref_into(),
            }
        }
    }

    // Abi

    impl From<&syn::Abi> for Abi {
        fn from(other: &syn::Abi) -> Self {
            Self {
                name: other.name.map_into(),
            }
        }
    }

    impl From<&Abi> for syn::Abi {
        fn from(other: &Abi) -> Self {
            Self {
                extern_token: default(),
                name: other.name.map_into(),
            }
        }
    }

    // BareFnArg

    impl From<&syn::BareFnArg> for BareFnArg {
        fn from(other: &syn::BareFnArg) -> Self {
            Self {
                name: other.name.ref_map(|(x, _)| x.ref_into()),
                ty: other.ty.ref_into(),
            }
        }
    }

    impl From<&BareFnArg> for syn::BareFnArg {
        fn from(other: &BareFnArg) -> Self {
            Self {
                name: other.name.ref_map(|x| (x.ref_into(), default())),
                ty: other.ty.ref_into(),
            }
        }
    }

    // BareFnArgName

    impl From<&syn::BareFnArgName> for BareFnArgName {
        fn from(other: &syn::BareFnArgName) -> Self {
            use super::BareFnArgName::*;
            use syn::BareFnArgName;
            match other {
                BareFnArgName::Named(x) => Named(x.into()),
                BareFnArgName::Wild(_) => Wild,
            }
        }
    }

    impl From<&BareFnArgName> for syn::BareFnArgName {
        fn from(other: &BareFnArgName) -> Self {
            use syn::BareFnArgName::*;
            match other {
                BareFnArgName::Named(x) => Named(x.into()),
                BareFnArgName::Wild => Wild(default()),
            }
        }
    }

    // ReturnType

    impl From<&syn::ReturnType> for ReturnType {
        fn from(other: &syn::ReturnType) -> Self {
            use syn::ReturnType;
            match other {
                ReturnType::Default => Self { ty: None },
                ReturnType::Type(_, x) => Self {
                    ty: Some(x.map_into()),
                },
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
