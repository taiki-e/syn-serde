use super::*;

ast_enum_of_structs! {
    /// Things that can appear directly inside of a module or scope.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum Item {
        /// An `extern crate` item: `extern crate serde`.
        pub ExternCrate(ItemExternCrate {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            ident: Ident,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            rename: Option<Ident>,
        }),

        /// A use declaration: `use std::collections::HashMap`.
        pub Use(ItemUse {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            #[serde(default, skip_serializing_if = "not")]
            leading_colon: bool,
            tree: UseTree,
        }),

        /// A static item: `static BIKE: Shed = Shed(42)`.
        pub Static(ItemStatic {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            #[serde(rename = "mut")]
            #[serde(default, skip_serializing_if = "not")]
            mutability: bool,
            ident: Ident,
            ty: Box<Type>,
            expr: Box<Expr>,
        }),

        /// A constant item: `const MAX: u16 = 65535`.
        pub Const(ItemConst {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            ident: Ident,
            ty: Box<Type>,
            expr: Box<Expr>,
        }),

        /// A free-standing function: `fn process(n: usize) -> Result<()> { ...
        /// }`.
        pub Fn(ItemFn {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            #[serde(rename = "const")]
            #[serde(default, skip_serializing_if = "not")]
            constness: bool,
            #[serde(rename = "unsafe")]
            #[serde(default, skip_serializing_if = "not")]
            unsafety: bool,
            #[serde(rename = "async")]
            #[serde(default, skip_serializing_if = "not")]
            asyncness: bool,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            abi: Option<Abi>,
            ident: Ident,
            #[serde(flatten)]
            decl: Box<FnDecl>,
            #[serde(flatten)]
            block: Box<Block>,
        }),

        /// A module or module declaration: `mod m` or `mod m { ... }`.
        pub Mod(ItemMod {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            ident: Ident,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            content: Option<Vec<Item>>,
            #[serde(default, skip_serializing_if = "not")]
            semi: bool,
        }),

        /// A block of foreign items: `extern "C" { ... }`.
        pub ForeignMod(ItemForeignMod {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            abi: Abi,
            items: Vec<ForeignItem>,
        }),

        /// A type alias: `type Result<T> = std::result::Result<T, MyError>`.
        pub Type(ItemType {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            ident: Ident,
            #[serde(default, skip_serializing_if = "Generics::is_none")]
            generics: Generics,
            ty: Box<Type>,
        }),

        /// An existential type: `existential type Iter: Iterator<Item = u8>`.
        pub Existential(ItemExistential {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            ident: Ident,
            #[serde(default, skip_serializing_if = "Generics::is_none")]
            generics: Generics,
            #[serde(default, skip_serializing_if = "not")]
            colon_token: bool,
            #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
            bounds: Punctuated<TypeParamBound>,
        }),

        /// A struct definition: `struct Foo<A> { x: A }`.
        pub Struct(ItemStruct {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            ident: Ident,
            #[serde(default, skip_serializing_if = "Generics::is_none")]
            generics: Generics,
            fields: Fields,
            // #[serde(default, skip_serializing_if = "not")]
            // semi_token: bool,
        }),

        /// An enum definition: `enum Foo<A, B> { C<A>, D<B> }`.
        pub Enum(ItemEnum {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            ident: Ident,
            #[serde(default, skip_serializing_if = "Generics::is_none")]
            generics: Generics,
            variants: Punctuated<Variant>,
        }),

        /// A union definition: `union Foo<A, B> { x: A, y: B }`.
        pub Union(ItemUnion {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            ident: Ident,
            #[serde(default, skip_serializing_if = "Generics::is_none")]
            generics: Generics,
            fields: FieldsNamed,
        }),

        /// A trait definition: `pub trait Iterator { ... }`.
        pub Trait(ItemTrait {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            #[serde(rename = "unsafe")]
            #[serde(default, skip_serializing_if = "not")]
            unsafety: bool,
            #[serde(rename = "auto")]
            #[serde(default, skip_serializing_if = "not")]
            auto_token: bool,
            ident: Ident,
            #[serde(default, skip_serializing_if = "Generics::is_none")]
            generics: Generics,
            #[serde(default, skip_serializing_if = "not")]
            colon_token: bool,
            #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
            supertraits: Punctuated<TypeParamBound>,
            items: Vec<TraitItem>,
        }),

        /// A trait alias: `pub trait SharableIterator = Iterator + Sync`.
        pub TraitAlias(ItemTraitAlias {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            ident: Ident,
            #[serde(default, skip_serializing_if = "Generics::is_none")]
            generics: Generics,
            bounds: Punctuated<TypeParamBound>,
        }),

        /// An impl block providing trait or associated items: `impl<A> Trait
        /// for Data<A> { ... }`.
        pub Impl(ItemImpl {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "not")]
            defaultness: bool,
            #[serde(default, skip_serializing_if = "not")]
            unsafety: bool,
            #[serde(default, skip_serializing_if = "Generics::is_none")]
            generics: Generics,
            /// Trait this impl implements.
            #[serde(rename = "trait")]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            trait_: Option<(bool, Path)>,
            /// The Self type of the impl.
            self_ty: Box<Type>,
            items: Vec<ImplItem>,
        }),

        /// A macro invocation, which includes `macro_rules!` definitions.
        pub Macro(ItemMacro {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            /// The `example` in `macro_rules! example { ... }`.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            ident: Option<Ident>,
            #[serde(flatten)]
            mac: Macro,
            #[serde(default, skip_serializing_if = "not")]
            semi_token: bool,
        }),

        /// A 2.0-style declarative macro introduced by the `macro` keyword.
        pub Macro2(ItemMacro2 {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            ident: Ident,
            args: TokenStream,
            body: TokenStream,
        }),

        /// Tokens forming an item not interpreted by Syn.
        pub Verbatim(ItemVerbatim {
            tts: TokenStream,
        }),
    }
}

ast_enum_of_structs! {
    /// A suffix of an import tree in a `use` item: `Type as Renamed` or `*`.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum UseTree #manual_from_impl {
        /// A path prefix of imports in a `use` item: `std::...`.
        pub Path(UsePath {
            ident: Ident,
            tree: Box<UseTree>,
        }),

        /// An identifier imported by a `use` item: `HashMap`.
        pub Name(UseName {
            ident: Ident,
        }),

        /// An renamed identifier imported by a `use` item: `HashMap as Map`.
        pub Rename(UseRename {
            ident: Ident,
            rename: Ident,
        }),

        /// A glob import in a `use` item: `*`.
        #[serde(rename = "*")]
        pub Glob,

        /// A braced group of imports in a `use` item: `{A, B, C}`.
        pub Group(UseGroup #transparent {
            items: Punctuated<UseTree>,
        }),
    }
}

ast_enum_of_structs! {
    /// An item within an `extern` block.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum ForeignItem {
        /// A foreign function in an `extern` block.
        pub Fn(ForeignItemFn {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            ident: Ident,
            #[serde(flatten)]
            decl: Box<FnDecl>,
        }),

        /// A foreign static item in an `extern` block: `static ext: u8`.
        pub Static(ForeignItemStatic {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            #[serde(rename = "mut")]
            #[serde(default, skip_serializing_if = "not")]
            mutability: bool,
            ident: Ident,
            ty: Box<Type>,
        }),

        /// A foreign type in an `extern` block: `type void`.
        pub Type(ForeignItemType {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            ident: Ident,
        }),

        /// A macro invocation within an extern block.
        pub Macro(ForeignItemMacro {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(flatten)]
            mac: Macro,
            #[serde(default, skip_serializing_if = "not")]
            semi_token: bool,
        }),

        /// Tokens in an `extern` block not interpreted by Syn.
        pub Verbatim(ForeignItemVerbatim {
            tts: TokenStream,
        }),
    }
}

ast_enum_of_structs! {
    /// An item declaration within the definition of a trait.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum TraitItem {
        /// An associated constant within the definition of a trait.
        pub Const(TraitItemConst {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            ident: Ident,
            ty: Type,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            default: Option<Expr>,
        }),

        /// A trait method within the definition of a trait.
        pub Method(TraitItemMethod {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(flatten)]
            sig: MethodSig,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            default: Option<Block>,
            // #[serde(default, skip_serializing_if = "not")]
            // semi_token: bool,
        }),

        /// An associated type within the definition of a trait.
        pub Type(TraitItemType {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            ident: Ident,
            #[serde(default, skip_serializing_if = "Generics::is_none")]
            generics: Generics,
            #[serde(default, skip_serializing_if = "not")]
            colon_token: bool,
            #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
            bounds: Punctuated<TypeParamBound>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            default: Option<Type>,
        }),

        /// A macro invocation within the definition of a trait.
        pub Macro(TraitItemMacro {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(flatten)]
            mac: Macro,
            #[serde(default, skip_serializing_if = "not")]
            semi_token: bool,
        }),

        /// Tokens within the definition of a trait not interpreted by Syn.
        pub Verbatim(TraitItemVerbatim {
            tts: TokenStream,
        }),
    }
}

ast_enum_of_structs! {
    /// An item within an impl block.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum ImplItem {
        /// An associated constant within an impl block.
        pub Const(ImplItemConst {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            #[serde(rename = "default")]
            #[serde(default, skip_serializing_if = "not")]
            defaultness: bool,
            ident: Ident,
            ty: Type,
            expr: Expr,
        }),

        /// A method within an impl block.
        pub Method(ImplItemMethod {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            #[serde(rename = "default")]
            #[serde(default, skip_serializing_if = "not")]
            defaultness: bool,
            #[serde(flatten)]
            sig: MethodSig,
            #[serde(flatten)]
            block: Block,
        }),

        /// An associated type within an impl block.
        pub Type(ImplItemType {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
            vis: Visibility,
            #[serde(rename = "default")]
            #[serde(default, skip_serializing_if = "not")]
            defaultness: bool,
            ident: Ident,
            #[serde(default, skip_serializing_if = "Generics::is_none")]
            generics: Generics,
            ty: Type,
        }),

        /// An existential type within an impl block.
        pub Existential(ImplItemExistential {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            ident: Ident,
            #[serde(default, skip_serializing_if = "Generics::is_none")]
            generics: Generics,
            #[serde(default, skip_serializing_if = "not")]
            colon_token: bool,
            #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
            bounds: Punctuated<TypeParamBound>,
        }),

        /// A macro invocation within an impl block.
        pub Macro(ImplItemMacro {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(flatten)]
            mac: Macro,
            #[serde(default, skip_serializing_if = "not")]
            semi_token: bool,
        }),

        /// Tokens within an impl block not interpreted by Syn.
        pub Verbatim(ImplItemVerbatim {
            tts: TokenStream,
        }),
    }
}

ast_struct! {
    /// A method's signature in a trait or implementation: `unsafe fn
    /// initialize(&self)`.
    pub struct MethodSig {
        #[serde(rename = "const")]
        #[serde(default, skip_serializing_if = "not")]
        constness: bool,
        #[serde(rename = "unsafe")]
        #[serde(default, skip_serializing_if = "not")]
        unsafety: bool,
        #[serde(rename = "async")]
        #[serde(default, skip_serializing_if = "not")]
        asyncness: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        abi: Option<Abi>,
        ident: Ident,
        #[serde(flatten)]
        decl: FnDecl,
    }
}

ast_struct! {
    /// Header of a function declaration, without including the body.
    pub struct FnDecl {
        #[serde(default, skip_serializing_if = "Generics::is_none")]
        generics: Generics,
        inputs: Punctuated<FnArg>,
        #[serde(default, skip_serializing_if = "not")]
        variadic: bool,
        output: ReturnType,
    }
}

ast_enum_of_structs! {
    /// An argument in a function signature: the `n: usize` in `fn f(n: usize)`.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum FnArg {
        /// Self captured by reference in a function signature: `&self` or `&mut
        /// self`.
        pub SelfRef(ArgSelfRef {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            lifetime: Option<Lifetime>,
            #[serde(rename = "mut")]
            #[serde(default, skip_serializing_if = "not")]
            mutability: bool,
        }),

        /// Self captured by value in a function signature: `self` or `mut
        /// self`.
        pub SelfValue(ArgSelf {
            #[serde(rename = "mut")]
            #[serde(default, skip_serializing_if = "not")]
            mutability: bool,
        }),

        /// An explicitly typed pattern captured by a function signature.
        pub Captured(ArgCaptured {
            pat: Pat,
            ty: Type,
        }),

        /// A pattern whose type is inferred captured by a function signature.
        pub Inferred(Pat),
        /// A type not bound to any pattern in a function signature.
        pub Ignored(Type),
    }
}

mod convert {
    use super::*;

    // ItemExternCrate

    impl From<&syn::ItemExternCrate> for ItemExternCrate {
        fn from(other: &syn::ItemExternCrate) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
                rename: other.rename.ref_map(|(_, x)| x.ref_into()),
            }
        }
    }

    impl From<&ItemExternCrate> for syn::ItemExternCrate {
        fn from(other: &ItemExternCrate) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                extern_token: default(),
                crate_token: default(),
                ident: other.ident.ref_into(),
                rename: other.rename.ref_map(|x| (default(), x.ref_into())),
                semi_token: default(),
            }
        }
    }

    // ItemUse

    impl From<&syn::ItemUse> for ItemUse {
        fn from(other: &syn::ItemUse) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                leading_colon: other.leading_colon.is_some(),
                tree: other.tree.ref_into(),
            }
        }
    }

    impl From<&ItemUse> for syn::ItemUse {
        fn from(other: &ItemUse) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                use_token: default(),
                leading_colon: default_or_none(other.leading_colon),
                tree: other.tree.ref_into(),
                semi_token: default(),
            }
        }
    }

    // ItemStatic

    impl From<&syn::ItemStatic> for ItemStatic {
        fn from(other: &syn::ItemStatic) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                mutability: other.mutability.is_some(),
                ident: other.ident.ref_into(),
                ty: other.ty.map_into(),
                expr: other.expr.map_into(),
            }
        }
    }

    impl From<&ItemStatic> for syn::ItemStatic {
        fn from(other: &ItemStatic) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                static_token: default(),
                mutability: default_or_none(other.mutability),
                ident: other.ident.ref_into(),
                colon_token: default(),
                ty: other.ty.map_into(),
                eq_token: default(),
                expr: other.expr.map_into(),
                semi_token: default(),
            }
        }
    }

    // ItemConst

    impl From<&syn::ItemConst> for ItemConst {
        fn from(other: &syn::ItemConst) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
                ty: other.ty.map_into(),
                expr: other.expr.map_into(),
            }
        }
    }

    impl From<&ItemConst> for syn::ItemConst {
        fn from(other: &ItemConst) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                const_token: default(),
                ident: other.ident.ref_into(),
                colon_token: default(),
                ty: other.ty.map_into(),
                eq_token: default(),
                expr: other.expr.map_into(),
                semi_token: default(),
            }
        }
    }

    // ItemFn

    impl From<&syn::ItemFn> for ItemFn {
        fn from(other: &syn::ItemFn) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                constness: other.constness.is_some(),
                unsafety: other.unsafety.is_some(),
                asyncness: other.asyncness.is_some(),
                abi: other.abi.map_into(),
                ident: other.ident.ref_into(),
                decl: other.decl.map_into(),
                block: other.block.map_into(),
            }
        }
    }

    impl From<&ItemFn> for syn::ItemFn {
        fn from(other: &ItemFn) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                constness: default_or_none(other.constness),
                unsafety: default_or_none(other.unsafety),
                asyncness: default_or_none(other.asyncness),
                abi: other.abi.map_into(),
                ident: other.ident.ref_into(),
                decl: other.decl.map_into(),
                block: other.block.map_into(),
            }
        }
    }

    // ItemMod

    impl From<&syn::ItemMod> for ItemMod {
        fn from(other: &syn::ItemMod) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
                content: other.content.ref_map(|(_, x)| x.map_into()),
                semi: other.semi.is_some(),
            }
        }
    }

    impl From<&ItemMod> for syn::ItemMod {
        fn from(other: &ItemMod) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                mod_token: default(),
                ident: other.ident.ref_into(),
                content: other.content.ref_map(|x| (default(), x.map_into())),
                semi: default_or_none(other.semi),
            }
        }
    }

    // ItemForeignMod

    impl From<&syn::ItemForeignMod> for ItemForeignMod {
        fn from(other: &syn::ItemForeignMod) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                abi: other.abi.ref_into(),
                items: other.items.map_into(),
            }
        }
    }

    impl From<&ItemForeignMod> for syn::ItemForeignMod {
        fn from(other: &ItemForeignMod) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                abi: other.abi.ref_into(),
                brace_token: default(),
                items: other.items.map_into(),
            }
        }
    }

    // ItemType

    impl From<&syn::ItemType> for ItemType {
        fn from(other: &syn::ItemType) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                ty: other.ty.map_into(),
            }
        }
    }

    impl From<&ItemType> for syn::ItemType {
        fn from(other: &ItemType) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                type_token: default(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                eq_token: default(),
                ty: other.ty.map_into(),
                semi_token: default(),
            }
        }
    }

    // ItemExistential

    impl From<&syn::ItemExistential> for ItemExistential {
        fn from(other: &syn::ItemExistential) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                colon_token: other.colon_token.is_some(),
                bounds: other.bounds.map_into(),
            }
        }
    }

    impl From<&ItemExistential> for syn::ItemExistential {
        fn from(other: &ItemExistential) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                existential_token: default(),
                type_token: default(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                colon_token: default_or_none(other.colon_token),
                bounds: other.bounds.map_into(),
                semi_token: default(),
            }
        }
    }

    // ItemStruct

    /* TODO: document
    /// # Panics
    ///
     */
    impl From<&syn::ItemStruct> for ItemStruct {
        fn from(other: &syn::ItemStruct) -> Self {
            let fields: Fields = other.fields.ref_into();
            assert_struct_semi(&fields, other.semi_token.is_some());

            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                fields,
            }
        }
    }

    impl From<&ItemStruct> for syn::ItemStruct {
        fn from(other: &ItemStruct) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                struct_token: default(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                fields: other.fields.ref_into(),
                semi_token: default_or_none(!other.fields.is_named()),
            }
        }
    }

    // ItemEnum

    impl From<&syn::ItemEnum> for ItemEnum {
        fn from(other: &syn::ItemEnum) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                variants: other.variants.map_into(),
            }
        }
    }

    impl From<&ItemEnum> for syn::ItemEnum {
        fn from(other: &ItemEnum) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                enum_token: default(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                brace_token: default(),
                variants: other.variants.map_into(),
            }
        }
    }

    // ItemUnion

    impl From<&syn::ItemUnion> for ItemUnion {
        fn from(other: &syn::ItemUnion) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                fields: other.fields.ref_into(),
            }
        }
    }

    impl From<&ItemUnion> for syn::ItemUnion {
        fn from(other: &ItemUnion) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                union_token: default(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                fields: other.fields.ref_into(),
            }
        }
    }

    // ItemTrait

    impl From<&syn::ItemTrait> for ItemTrait {
        fn from(other: &syn::ItemTrait) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                unsafety: other.unsafety.is_some(),
                auto_token: other.auto_token.is_some(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                colon_token: other.colon_token.is_some(),
                supertraits: other.supertraits.map_into(),
                items: other.items.map_into(),
            }
        }
    }

    impl From<&ItemTrait> for syn::ItemTrait {
        fn from(other: &ItemTrait) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                unsafety: default_or_none(other.unsafety),
                auto_token: default_or_none(other.auto_token),
                trait_token: default(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                colon_token: default_or_none(other.colon_token),
                supertraits: other.supertraits.map_into(),
                brace_token: default(),
                items: other.items.map_into(),
            }
        }
    }

    // ItemTraitAlias

    impl From<&syn::ItemTraitAlias> for ItemTraitAlias {
        fn from(other: &syn::ItemTraitAlias) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                bounds: other.bounds.map_into(),
            }
        }
    }

    impl From<&ItemTraitAlias> for syn::ItemTraitAlias {
        fn from(other: &ItemTraitAlias) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                trait_token: default(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                eq_token: default(),
                bounds: other.bounds.map_into(),
                semi_token: default(),
            }
        }
    }

    // ItemImpl

    impl From<&syn::ItemImpl> for ItemImpl {
        fn from(other: &syn::ItemImpl) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                defaultness: other.defaultness.is_some(),
                unsafety: other.unsafety.is_some(),
                generics: other.generics.ref_into(),
                trait_: other
                    .trait_
                    .ref_map(|(x, y, _)| (x.is_some(), y.ref_into())),
                self_ty: other.self_ty.map_into(),
                items: other.items.map_into(),
            }
        }
    }

    impl From<&ItemImpl> for syn::ItemImpl {
        fn from(other: &ItemImpl) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                defaultness: default_or_none(other.defaultness),
                unsafety: default_or_none(other.unsafety),
                impl_token: default(),
                generics: other.generics.ref_into(),
                trait_: other
                    .trait_
                    .ref_map(|(x, y)| (default_or_none(*x), y.ref_into(), default())),
                self_ty: other.self_ty.map_into(),
                brace_token: default(),
                items: other.items.map_into(),
            }
        }
    }

    // ItemMacro

    impl From<&syn::ItemMacro> for ItemMacro {
        fn from(other: &syn::ItemMacro) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                ident: other.ident.map_into(),
                mac: other.mac.ref_into(),
                semi_token: other.semi_token.is_some(),
            }
        }
    }

    impl From<&ItemMacro> for syn::ItemMacro {
        fn from(other: &ItemMacro) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                ident: other.ident.map_into(),
                mac: other.mac.ref_into(),
                semi_token: default_or_none(other.semi_token),
            }
        }
    }

    // ItemMacro2

    impl From<&syn::ItemMacro2> for ItemMacro2 {
        fn from(other: &syn::ItemMacro2) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
                args: other.args.ref_into(),
                body: other.body.ref_into(),
            }
        }
    }

    impl From<&ItemMacro2> for syn::ItemMacro2 {
        fn from(other: &ItemMacro2) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                macro_token: default(),
                ident: other.ident.ref_into(),
                paren_token: default(),
                args: other.args.ref_into(),
                brace_token: default(),
                body: other.body.ref_into(),
            }
        }
    }

    // ItemVerbatim

    impl From<&syn::ItemVerbatim> for ItemVerbatim {
        fn from(other: &syn::ItemVerbatim) -> Self {
            Self {
                tts: other.tts.ref_into(),
            }
        }
    }

    impl From<&ItemVerbatim> for syn::ItemVerbatim {
        fn from(other: &ItemVerbatim) -> Self {
            Self {
                tts: other.tts.ref_into(),
            }
        }
    }

    // UseTree

    impl From<&syn::UseTree> for UseTree {
        fn from(other: &syn::UseTree) -> Self {
            use super::UseTree::*;
            use syn::UseTree;
            match other {
                UseTree::Path(x) => Path(x.into()),
                UseTree::Name(x) => Name(x.into()),
                UseTree::Rename(x) => Rename(x.into()),
                UseTree::Glob(_) => Glob,
                UseTree::Group(x) => Group(x.into()),
            }
        }
    }

    impl From<&UseTree> for syn::UseTree {
        fn from(other: &UseTree) -> Self {
            use syn::UseTree::*;
            match other {
                UseTree::Path(x) => Path(x.into()),
                UseTree::Name(x) => Name(x.into()),
                UseTree::Rename(x) => Rename(x.into()),
                UseTree::Glob => Glob(syn::UseGlob {
                    star_token: default(),
                }),
                UseTree::Group(x) => Group(x.into()),
            }
        }
    }

    // UsePath

    impl From<&syn::UsePath> for UsePath {
        fn from(other: &syn::UsePath) -> Self {
            Self {
                ident: other.ident.ref_into(),
                tree: other.tree.map_into(),
            }
        }
    }

    impl From<&UsePath> for syn::UsePath {
        fn from(other: &UsePath) -> Self {
            Self {
                ident: other.ident.ref_into(),
                colon2_token: default(),
                tree: other.tree.map_into(),
            }
        }
    }

    // UseName

    impl From<&syn::UseName> for UseName {
        fn from(other: &syn::UseName) -> Self {
            Self {
                ident: other.ident.ref_into(),
            }
        }
    }

    impl From<&UseName> for syn::UseName {
        fn from(other: &UseName) -> Self {
            Self {
                ident: other.ident.ref_into(),
            }
        }
    }

    // UseRename

    impl From<&syn::UseRename> for UseRename {
        fn from(other: &syn::UseRename) -> Self {
            Self {
                ident: other.ident.ref_into(),
                rename: other.rename.ref_into(),
            }
        }
    }

    impl From<&UseRename> for syn::UseRename {
        fn from(other: &UseRename) -> Self {
            Self {
                ident: other.ident.ref_into(),
                as_token: default(),
                rename: other.rename.ref_into(),
            }
        }
    }

    // UseGroup

    impl From<&syn::UseGroup> for UseGroup {
        fn from(other: &syn::UseGroup) -> Self {
            Self {
                items: other.items.map_into(),
            }
        }
    }

    impl From<&UseGroup> for syn::UseGroup {
        fn from(other: &UseGroup) -> Self {
            Self {
                brace_token: default(),
                items: other.items.map_into(),
            }
        }
    }

    // ForeignItemFn

    impl From<&syn::ForeignItemFn> for ForeignItemFn {
        fn from(other: &syn::ForeignItemFn) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
                decl: other.decl.map_into(),
            }
        }
    }

    impl From<&ForeignItemFn> for syn::ForeignItemFn {
        fn from(other: &ForeignItemFn) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
                decl: other.decl.map_into(),
                semi_token: default(),
            }
        }
    }

    // ForeignItemStatic

    impl From<&syn::ForeignItemStatic> for ForeignItemStatic {
        fn from(other: &syn::ForeignItemStatic) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                mutability: other.mutability.is_some(),
                ident: other.ident.ref_into(),
                ty: other.ty.map_into(),
            }
        }
    }

    impl From<&ForeignItemStatic> for syn::ForeignItemStatic {
        fn from(other: &ForeignItemStatic) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                static_token: default(),
                mutability: default_or_none(other.mutability),
                ident: other.ident.ref_into(),
                colon_token: default(),
                ty: other.ty.map_into(),
                semi_token: default(),
            }
        }
    }

    // ForeignItemType

    impl From<&syn::ForeignItemType> for ForeignItemType {
        fn from(other: &syn::ForeignItemType) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
            }
        }
    }

    impl From<&ForeignItemType> for syn::ForeignItemType {
        fn from(other: &ForeignItemType) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                type_token: default(),
                ident: other.ident.ref_into(),
                semi_token: default(),
            }
        }
    }

    // ForeignItemMacro

    impl From<&syn::ForeignItemMacro> for ForeignItemMacro {
        fn from(other: &syn::ForeignItemMacro) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                mac: other.mac.ref_into(),
                semi_token: other.semi_token.is_some(),
            }
        }
    }

    impl From<&ForeignItemMacro> for syn::ForeignItemMacro {
        fn from(other: &ForeignItemMacro) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                mac: other.mac.ref_into(),
                semi_token: default_or_none(other.semi_token),
            }
        }
    }

    // ForeignItemVerbatim

    impl From<&syn::ForeignItemVerbatim> for ForeignItemVerbatim {
        fn from(other: &syn::ForeignItemVerbatim) -> Self {
            Self {
                tts: other.tts.ref_into(),
            }
        }
    }

    impl From<&ForeignItemVerbatim> for syn::ForeignItemVerbatim {
        fn from(other: &ForeignItemVerbatim) -> Self {
            Self {
                tts: other.tts.ref_into(),
            }
        }
    }

    // TraitItemConst

    impl From<&syn::TraitItemConst> for TraitItemConst {
        fn from(other: &syn::TraitItemConst) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                ident: other.ident.ref_into(),
                ty: other.ty.ref_into(),
                default: other.default.ref_map(|(_, x)| x.ref_into()),
            }
        }
    }

    impl From<&TraitItemConst> for syn::TraitItemConst {
        fn from(other: &TraitItemConst) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                const_token: default(),
                ident: other.ident.ref_into(),
                colon_token: default(),
                ty: other.ty.ref_into(),
                default: other.default.ref_map(|x| (default(), x.ref_into())),
                semi_token: default(),
            }
        }
    }

    // TraitItemMethod

    /* TODO: document
    /// # Panics
    ///
     */
    impl From<&syn::TraitItemMethod> for TraitItemMethod {
        fn from(other: &syn::TraitItemMethod) -> Self {
            if other.default.is_some() {
                // `fn foo() -> bool {};`
                assert!(other.semi_token.is_none(), "unexpected token: `;`");
            } else {
                // `fn foo() -> bool`
                assert!(other.semi_token.is_some(), "expected `;`");
            }

            Self {
                attrs: other.attrs.map_into(),
                sig: other.sig.ref_into(),
                default: other.default.map_into(),
            }
        }
    }

    impl From<&TraitItemMethod> for syn::TraitItemMethod {
        fn from(other: &TraitItemMethod) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                sig: other.sig.ref_into(),
                default: other.default.map_into(),
                semi_token: default_or_none(other.default.is_none()),
            }
        }
    }

    // TraitItemType

    impl From<&syn::TraitItemType> for TraitItemType {
        fn from(other: &syn::TraitItemType) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                colon_token: other.colon_token.is_some(),
                bounds: other.bounds.map_into(),
                default: other.default.ref_map(|(_, x)| x.ref_into()),
            }
        }
    }

    impl From<&TraitItemType> for syn::TraitItemType {
        fn from(other: &TraitItemType) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                type_token: default(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                colon_token: default_or_none(other.colon_token),
                bounds: other.bounds.map_into(),
                default: other.default.ref_map(|x| (default(), x.ref_into())),
                semi_token: default(),
            }
        }
    }

    // TraitItemMacro

    impl From<&syn::TraitItemMacro> for TraitItemMacro {
        fn from(other: &syn::TraitItemMacro) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                mac: other.mac.ref_into(),
                semi_token: other.semi_token.is_some(),
            }
        }
    }

    impl From<&TraitItemMacro> for syn::TraitItemMacro {
        fn from(other: &TraitItemMacro) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                mac: other.mac.ref_into(),
                semi_token: default_or_none(other.semi_token),
            }
        }
    }

    // TraitItemVerbatim

    impl From<&syn::TraitItemVerbatim> for TraitItemVerbatim {
        fn from(other: &syn::TraitItemVerbatim) -> Self {
            Self {
                tts: other.tts.ref_into(),
            }
        }
    }

    impl From<&TraitItemVerbatim> for syn::TraitItemVerbatim {
        fn from(other: &TraitItemVerbatim) -> Self {
            Self {
                tts: other.tts.ref_into(),
            }
        }
    }

    // ImplItemConst

    impl From<&syn::ImplItemConst> for ImplItemConst {
        fn from(other: &syn::ImplItemConst) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                defaultness: other.defaultness.is_some(),
                ident: other.ident.ref_into(),
                ty: other.ty.ref_into(),
                expr: other.expr.ref_into(),
            }
        }
    }

    impl From<&ImplItemConst> for syn::ImplItemConst {
        fn from(other: &ImplItemConst) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                defaultness: default_or_none(other.defaultness),
                const_token: default(),
                ident: other.ident.ref_into(),
                colon_token: default(),
                ty: other.ty.ref_into(),
                eq_token: default(),
                expr: other.expr.ref_into(),
                semi_token: default(),
            }
        }
    }

    // ImplItemMethod

    impl From<&syn::ImplItemMethod> for ImplItemMethod {
        fn from(other: &syn::ImplItemMethod) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                defaultness: other.defaultness.is_some(),
                sig: other.sig.ref_into(),
                block: other.block.ref_into(),
            }
        }
    }

    impl From<&ImplItemMethod> for syn::ImplItemMethod {
        fn from(other: &ImplItemMethod) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                defaultness: default_or_none(other.defaultness),
                sig: other.sig.ref_into(),
                block: other.block.ref_into(),
            }
        }
    }

    // ImplItemType

    impl From<&syn::ImplItemType> for ImplItemType {
        fn from(other: &syn::ImplItemType) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                defaultness: other.defaultness.is_some(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                ty: other.ty.ref_into(),
            }
        }
    }

    impl From<&ImplItemType> for syn::ImplItemType {
        fn from(other: &ImplItemType) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                defaultness: default_or_none(other.defaultness),
                type_token: default(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                eq_token: default(),
                ty: other.ty.ref_into(),
                semi_token: default(),
            }
        }
    }

    // ImplItemExistential

    impl From<&syn::ImplItemExistential> for ImplItemExistential {
        fn from(other: &syn::ImplItemExistential) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                colon_token: other.colon_token.is_some(),
                bounds: other.bounds.map_into(),
            }
        }
    }

    impl From<&ImplItemExistential> for syn::ImplItemExistential {
        fn from(other: &ImplItemExistential) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                existential_token: default(),
                type_token: default(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                colon_token: default_or_none(other.colon_token),
                bounds: other.bounds.map_into(),
                semi_token: default(),
            }
        }
    }

    // ImplItemMacro

    impl From<&syn::ImplItemMacro> for ImplItemMacro {
        fn from(other: &syn::ImplItemMacro) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                mac: other.mac.ref_into(),
                semi_token: other.semi_token.is_some(),
            }
        }
    }

    impl From<&ImplItemMacro> for syn::ImplItemMacro {
        fn from(other: &ImplItemMacro) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                mac: other.mac.ref_into(),
                semi_token: default_or_none(other.semi_token),
            }
        }
    }

    // ImplItemVerbatim

    impl From<&syn::ImplItemVerbatim> for ImplItemVerbatim {
        fn from(other: &syn::ImplItemVerbatim) -> Self {
            Self {
                tts: other.tts.ref_into(),
            }
        }
    }

    impl From<&ImplItemVerbatim> for syn::ImplItemVerbatim {
        fn from(other: &ImplItemVerbatim) -> Self {
            Self {
                tts: other.tts.ref_into(),
            }
        }
    }

    // MethodSig

    impl From<&syn::MethodSig> for MethodSig {
        fn from(other: &syn::MethodSig) -> Self {
            Self {
                constness: other.constness.is_some(),
                unsafety: other.unsafety.is_some(),
                asyncness: other.asyncness.is_some(),
                abi: other.abi.map_into(),
                ident: other.ident.ref_into(),
                decl: other.decl.ref_into(),
            }
        }
    }

    impl From<&MethodSig> for syn::MethodSig {
        fn from(other: &MethodSig) -> Self {
            Self {
                constness: default_or_none(other.constness),
                unsafety: default_or_none(other.unsafety),
                asyncness: default_or_none(other.asyncness),
                abi: other.abi.map_into(),
                ident: other.ident.ref_into(),
                decl: other.decl.ref_into(),
            }
        }
    }

    // FnDecl

    impl From<&syn::FnDecl> for FnDecl {
        fn from(other: &syn::FnDecl) -> Self {
            Self {
                generics: other.generics.ref_into(),
                inputs: other.inputs.map_into(),
                variadic: other.variadic.is_some(),
                output: other.output.ref_into(),
            }
        }
    }

    impl From<&FnDecl> for syn::FnDecl {
        fn from(other: &FnDecl) -> Self {
            Self {
                fn_token: default(),
                generics: other.generics.ref_into(),
                paren_token: default(),
                inputs: other.inputs.map_into(),
                variadic: default_or_none(other.variadic),
                output: other.output.ref_into(),
            }
        }
    }

    // ArgSelfRef

    impl From<&syn::ArgSelfRef> for ArgSelfRef {
        fn from(other: &syn::ArgSelfRef) -> Self {
            Self {
                lifetime: other.lifetime.map_into(),
                mutability: other.mutability.is_some(),
            }
        }
    }

    impl From<&ArgSelfRef> for syn::ArgSelfRef {
        fn from(other: &ArgSelfRef) -> Self {
            Self {
                and_token: default(),
                lifetime: other.lifetime.map_into(),
                mutability: default_or_none(other.mutability),
                self_token: default(),
            }
        }
    }

    // ArgSelf

    impl From<&syn::ArgSelf> for ArgSelf {
        fn from(other: &syn::ArgSelf) -> Self {
            Self {
                mutability: other.mutability.is_some(),
            }
        }
    }

    impl From<&ArgSelf> for syn::ArgSelf {
        fn from(other: &ArgSelf) -> Self {
            Self {
                mutability: default_or_none(other.mutability),
                self_token: default(),
            }
        }
    }

    // ArgCaptured

    impl From<&syn::ArgCaptured> for ArgCaptured {
        fn from(other: &syn::ArgCaptured) -> Self {
            Self {
                pat: other.pat.ref_into(),
                ty: other.ty.ref_into(),
            }
        }
    }

    impl From<&ArgCaptured> for syn::ArgCaptured {
        fn from(other: &ArgCaptured) -> Self {
            Self {
                pat: other.pat.ref_into(),
                colon_token: default(),
                ty: other.ty.ref_into(),
            }
        }
    }
}
