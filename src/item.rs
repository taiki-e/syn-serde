use super::*;

#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::ast_enum::{FnArg, ForeignItem, ImplItem, Item, TraitItem};

ast_struct! {
    /// A constant item: `const MAX: u16 = 65535`.
    pub struct ItemConst {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        pub(crate) ident: Ident,
        pub(crate) ty: Box<Type>,
        pub(crate) expr: Box<Expr>,
    }
}

ast_struct! {
    /// An enum definition: `enum Foo<A, B> { A(A), B(B) }`.
    pub struct ItemEnum {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        pub(crate) ident: Ident,
        #[serde(default, skip_serializing_if = "Generics::is_none")]
        pub(crate) generics: Generics,
        pub(crate) variants: Punctuated<Variant>,
    }
}

ast_struct! {
    /// An `extern crate` item: `extern crate serde`.
    pub struct ItemExternCrate {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        pub(crate) ident: Ident,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) rename: Option<Ident>,
    }
}

ast_struct! {
    /// A free-standing function: `fn process(n: usize) -> Result<()> { ...
    /// }`.
    pub struct ItemFn {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        #[serde(flatten)]
        pub(crate) sig: Signature,
        #[serde(rename = "stmts")]
        pub(crate) block: Box<Block>,
    }
}

ast_struct! {
    /// A block of foreign items: `extern "C" { ... }`.
    pub struct ItemForeignMod {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) abi: Abi,
        pub(crate) items: Vec<ForeignItem>,
    }
}

ast_struct! {
    /// An impl block providing trait or associated items: `impl<A> Trait
    /// for Data<A> { ... }`.
    pub struct ItemImpl {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(rename = "default")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) defaultness: bool,
        #[serde(rename = "unsafe")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) unsafety: bool,
        #[serde(default, skip_serializing_if = "Generics::is_none")]
        pub(crate) generics: Generics,
        /// Trait this impl implements.
        #[serde(rename = "trait")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) trait_: Option<(bool, Path)>,
        /// The Self type of the impl.
        pub(crate) self_ty: Box<Type>,
        pub(crate) items: Vec<ImplItem>,
    }
}

ast_struct! {
    /// A macro invocation, which includes `macro_rules!` definitions.
    pub struct ItemMacro {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        /// The `example` in `macro_rules! example { ... }`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) ident: Option<Ident>,
        #[serde(flatten)]
        pub(crate) mac: Macro,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) semi_token: bool,
    }
}

ast_struct! {
    /// A 2.0-style declarative macro introduced by the `macro` keyword.
    pub struct ItemMacro2 {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        pub(crate) ident: Ident,
        pub(crate) rules: TokenStream,
    }
}

ast_struct! {
    /// A module or module declaration: `mod m` or `mod m { ... }`.
    pub struct ItemMod {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        pub(crate) ident: Ident,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) content: Option<Vec<Item>>,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) semi: bool,
    }
}

ast_struct! {
    /// A static item: `static BIKE: Shed = Shed(42)`.
    pub struct ItemStatic {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        #[serde(rename = "mut")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) mutability: bool,
        pub(crate) ident: Ident,
        pub(crate) ty: Box<Type>,
        pub(crate) expr: Box<Expr>,
    }
}

ast_struct! {
    /// A struct definition: `struct Foo<A> { x: A }`.
    pub struct ItemStruct {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        pub(crate) ident: Ident,
        #[serde(default, skip_serializing_if = "Generics::is_none")]
        pub(crate) generics: Generics,
        pub(crate) fields: Fields,
        // #[serde(default, skip_serializing_if = "not")]
        // pub(crate) semi_token: bool,
    }
}

ast_struct! {
    /// A trait definition: `pub trait Iterator { ... }`.
    pub struct ItemTrait {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        #[serde(rename = "unsafe")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) unsafety: bool,
        #[serde(rename = "auto")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) auto_token: bool,
        pub(crate) ident: Ident,
        #[serde(default, skip_serializing_if = "Generics::is_none")]
        pub(crate) generics: Generics,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) colon_token: bool,
        #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
        pub(crate) supertraits: Punctuated<TypeParamBound>,
        pub(crate) items: Vec<TraitItem>,
    }
}

ast_struct! {
    /// A trait alias: `pub trait SharableIterator = Iterator + Sync`.
    pub struct ItemTraitAlias {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        pub(crate) ident: Ident,
        #[serde(default, skip_serializing_if = "Generics::is_none")]
        pub(crate) generics: Generics,
        pub(crate) bounds: Punctuated<TypeParamBound>,
    }
}

ast_struct! {
    /// A type alias: `type Result<T> = std::result::Result<T, MyError>`.
    pub struct ItemType {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        pub(crate) ident: Ident,
        #[serde(default, skip_serializing_if = "Generics::is_none")]
        pub(crate) generics: Generics,
        pub(crate) ty: Box<Type>,
    }
}

ast_struct! {
    /// A union definition: `union Foo<A, B> { x: A, y: B }`.
    pub struct ItemUnion {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        pub(crate) ident: Ident,
        #[serde(default, skip_serializing_if = "Generics::is_none")]
        pub(crate) generics: Generics,
        pub(crate) fields: FieldsNamed,
    }
}

ast_struct! {
    /// A use declaration: `use std::collections::HashMap`.
    pub struct ItemUse {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) leading_colon: bool,
        pub(crate) tree: UseTree,
    }
}

ast_enum! {
    /// A suffix of an import tree in a `use` item: `Type as Renamed` or `*`.
    pub enum UseTree {
        /// A path prefix of imports in a `use` item: `std::...`.
        Path(UsePath),

        /// An identifier imported by a `use` item: `HashMap`.
        #[serde(rename = "ident")]
        Name(UseName),

        /// An renamed identifier imported by a `use` item: `HashMap as Map`.
        Rename(UseRename),

        /// A glob import in a `use` item: `*`.
        #[serde(rename = "*")]
        Glob,

        /// A braced group of imports in a `use` item: `{A, B, C}`.
        Group(UseGroup),
    }
}

ast_struct! {
    /// A path prefix of imports in a `use` item: `std::...`.
    pub struct UsePath {
        pub(crate) ident: Ident,
        pub(crate) tree: Box<UseTree>,
    }
}

ast_struct! {
    /// An identifier imported by a `use` item: `HashMap`.
    #[serde(transparent)]
    pub struct UseName {
        pub(crate) ident: Ident,
    }
}

ast_struct! {
    /// An renamed identifier imported by a `use` item: `HashMap as Map`.
    pub struct UseRename {
        pub(crate) ident: Ident,
        pub(crate) rename: Ident,
    }
}

ast_struct! {
    /// A braced group of imports in a `use` item: `{A, B, C}`.
    #[serde(transparent)]
    pub struct UseGroup {
        pub(crate) items: Punctuated<UseTree>,
    }
}

ast_struct! {
    /// A foreign function in an `extern` block.
    pub struct ForeignItemFn {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        #[serde(flatten)]
        pub(crate) sig: Signature,
    }
}

ast_struct! {
    /// A foreign static item in an `extern` block: `static ext: u8`.
    pub struct ForeignItemStatic {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        #[serde(rename = "mut")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) mutability: bool,
        pub(crate) ident: Ident,
        pub(crate) ty: Box<Type>,
    }
}

ast_struct! {
    /// A foreign type in an `extern` block: `type void`.
    pub struct ForeignItemType {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        pub(crate) ident: Ident,
    }
}

ast_struct! {
    /// A macro invocation within an extern block.
    pub struct ForeignItemMacro {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(flatten)]
        pub(crate) mac: Macro,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) semi_token: bool,
    }
}

ast_struct! {
    /// An associated constant within the definition of a trait.
    pub struct TraitItemConst {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) ident: Ident,
        pub(crate) ty: Type,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) default: Option<Expr>,
    }
}

ast_struct! {
    /// A trait method within the definition of a trait.
    pub struct TraitItemMethod {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(flatten)]
        pub(crate) sig: Signature,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) default: Option<Block>,
        // #[serde(default, skip_serializing_if = "not")]
        // pub(crate) semi_token: bool,
    }
}

ast_struct! {
    /// An associated type within the definition of a trait.
    pub struct TraitItemType {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) ident: Ident,
        #[serde(default, skip_serializing_if = "Generics::is_none")]
        pub(crate) generics: Generics,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) colon_token: bool,
        #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
        pub(crate) bounds: Punctuated<TypeParamBound>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) default: Option<Type>,
    }
}

ast_struct! {
    /// A macro invocation within the definition of a trait.
    pub struct TraitItemMacro {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(flatten)]
        pub(crate) mac: Macro,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) semi_token: bool,
    }
}

ast_struct! {
    /// An associated constant within an impl block.
    pub struct ImplItemConst {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        #[serde(rename = "default")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) defaultness: bool,
        pub(crate) ident: Ident,
        pub(crate) ty: Type,
        pub(crate) expr: Expr,
    }
}

ast_struct! {
    /// A method within an impl block.
    pub struct ImplItemMethod {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        #[serde(rename = "default")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) defaultness: bool,
        #[serde(flatten)]
        pub(crate) sig: Signature,
        #[serde(rename = "stmts")]
        pub(crate) block: Block,
    }
}

ast_struct! {
    /// An associated type within an impl block.
    pub struct ImplItemType {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        #[serde(rename = "default")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) defaultness: bool,
        pub(crate) ident: Ident,
        #[serde(default, skip_serializing_if = "Generics::is_none")]
        pub(crate) generics: Generics,
        pub(crate) ty: Type,
    }
}

ast_struct! {
    /// A macro invocation within an impl block.
    pub struct ImplItemMacro {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(flatten)]
        pub(crate) mac: Macro,
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) semi_token: bool,
    }
}

ast_struct! {
    /// A function signature in a trait or implementation: `unsafe fn
    /// initialize(&self)`.
    pub struct Signature {
        #[serde(rename = "const")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) constness: bool,
        #[serde(rename = "async")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) asyncness: bool,
        #[serde(rename = "unsafe")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) unsafety: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) abi: Option<Abi>,
        pub(crate) ident: Ident,
        #[serde(default, skip_serializing_if = "Generics::is_none")]
        pub(crate) generics: Generics,
        pub(crate) inputs: Punctuated<FnArg>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) variadic: Option<Variadic>,
        #[serde(default)]
        pub(crate) output: ReturnType,
    }
}

ast_struct! {
    /// The `self` argument of an associated method, whether taken by value
    /// or by reference.
    pub struct Receiver {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(rename = "ref")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) reference: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) lifetime: Option<Lifetime>,
        #[serde(rename = "mut")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) mutability: bool,
    }
}

mod convert {
    use super::*;

    // ItemStruct
    syn_trait_impl!(syn::ItemStruct);
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

    // TraitItemMethod
    syn_trait_impl!(syn::TraitItemMethod);
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

    // UseTree
    syn_trait_impl!(syn::UseTree);
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
                UseTree::Glob => Glob(syn::UseGlob { star_token: default() }),
                UseTree::Group(x) => Group(x.into()),
            }
        }
    }

    // Receiver
    syn_trait_impl!(syn::Receiver);
    impl From<&syn::Receiver> for Receiver {
        fn from(node: &syn::Receiver) -> Self {
            Self {
                attrs: node.attrs.map_into(),
                reference: node.reference.is_some(),
                lifetime: node.reference.as_ref().and_then(|(_0, _1)| _1.map_into()),
                mutability: node.mutability.is_some(),
            }
        }
    }
    impl From<&Receiver> for syn::Receiver {
        fn from(node: &Receiver) -> Self {
            Self {
                attrs: node.attrs.map_into(),
                reference: if node.reference {
                    Some((default(), node.lifetime.map_into()))
                } else {
                    None
                },
                mutability: default_or_none(node.mutability),
                self_token: default(),
            }
        }
    }
}
