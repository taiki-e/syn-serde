// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::*;
pub use crate::{
    ast_enum::{
        FnArg, ForeignItem, ImplItem, Item, Safety, StaticMutability, TraitItem, UseTree,
        WhereClausePlacement,
    },
    ast_struct::{
        ConstModifiers, FnModifiers, ForeignItemFn, ForeignItemMacro, ForeignItemStatic,
        ForeignItemType, ImplItemConst, ImplItemFn, ImplItemMacro, ImplItemType, ImplModifiers,
        ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemForeignMod, ItemImpl, ItemMacro,
        ItemStatic, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, ItemUse, Signature,
        TraitItemConst, TraitItemMacro, TraitItemType, TraitModifiers, TypeModifiers, UseGroup,
        UseName, UsePath, UseRename, Variadic,
    },
};

ast_struct! {
    /// An adapter for [`struct@syn::ItemMod`].
    pub struct ItemMod {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub(crate) vis: Visibility,
        #[serde(rename = "unsafe")]
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) unsafety: bool,
        pub(crate) ident: Ident,
        // TODO: should not skip_serializing_if
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) content: Option<Vec<Item>>,
        // TODO: can remove
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) semi: bool,
    }
}

ast_struct! {
    /// An adapter for [`struct@syn::ItemStruct`].
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
    /// An adapter for [`struct@syn::TraitItemFn`].
    pub struct TraitItemFn {
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
    /// An adapter for [`struct@syn::Receiver`].
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
        #[serde(default, skip_serializing_if = "not")]
        pub(crate) colon_token: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) ty: Option<Box<Type>>,
    }
}

impl StaticMutability {
    pub(crate) fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}
impl Default for StaticMutability {
    fn default() -> Self {
        Self::None
    }
}

impl Safety {
    pub(crate) fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }
}
impl Default for Safety {
    fn default() -> Self {
        Self::Default
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

    // TraitItemFn
    syn_trait_impl!(syn::TraitItemFn);
    impl From<&syn::TraitItemFn> for TraitItemFn {
        fn from(other: &syn::TraitItemFn) -> Self {
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
    impl From<&TraitItemFn> for syn::TraitItemFn {
        fn from(other: &TraitItemFn) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                modifiers: default(),
                sig: other.sig.ref_into(),
                default: other.default.map_into(),
                semi_token: default_or_none(other.default.is_none()),
            }
        }
    }

    // Receiver
    syn_trait_impl!(syn::Receiver);
    impl From<&syn::Receiver> for Receiver {
        fn from(node: &syn::Receiver) -> Self {
            use syn::ReceiverKind;
            match &node.kind {
                ReceiverKind::Value => Self {
                    attrs: node.attrs.map_into(),
                    reference: false,
                    lifetime: None,
                    mutability: node.mutability.is_some(),
                    colon_token: false,
                    ty: None,
                },
                ReceiverKind::Reference(_, lt, mut_) => Self {
                    attrs: node.attrs.map_into(),
                    reference: true,
                    lifetime: lt.map_into(),
                    mutability: mut_.is_some(),
                    colon_token: false,
                    ty: None,
                },
                ReceiverKind::Typed(_, ty) => Self {
                    attrs: node.attrs.map_into(),
                    reference: false,
                    lifetime: None,
                    mutability: false,
                    colon_token: true,
                    ty: Some(ty.map_into()),
                },
                _ => unreachable!(),
            }
        }
    }
    impl From<&Receiver> for syn::Receiver {
        fn from(node: &Receiver) -> Self {
            use syn::ReceiverKind;
            let kind = if node.colon_token {
                ReceiverKind::Typed(
                    default(),
                    node.ty.as_ref().expect("ty required when colon_token").map_into(),
                )
            } else if node.reference {
                ReceiverKind::Reference(
                    default(),
                    node.lifetime.map_into(),
                    default_or_none(node.mutability),
                )
            } else {
                ReceiverKind::Value
            };
            Self {
                attrs: node.attrs.map_into(),
                mutability: default_or_none(
                    !node.reference && !node.colon_token && node.mutability,
                ),
                self_token: default(),
                kind,
            }
        }
    }
}
