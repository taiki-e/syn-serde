// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::*;
pub use crate::ast_struct::File;
use crate::ast_struct::ItemImpl;

mod convert {
    use super::*;

    // File
    syn_trait_impl!(syn::File);
    impl From<&syn::File> for File {
        fn from(node: &syn::File) -> Self {
            Self {
                shebang: node.shebang.clone(),
                attrs: node.attrs.map_into(),
                items: node.items.map_into(),
            }
        }
    }
    impl From<&File> for syn::File {
        fn from(node: &File) -> Self {
            Self {
                shebang: node.shebang.clone(),
                frontmatter: None,
                attrs: node.attrs.map_into(),
                items: node.items.map_into(),
            }
        }
    }

    // ItemImpl
    syn_trait_impl!(syn::ItemImpl);
    impl From<&syn::ItemImpl> for ItemImpl {
        fn from(node: &syn::ItemImpl) -> Self {
            Self {
                attrs: node.attrs.map_into(),
                modifiers: node.modifiers.ref_into(),
                unsafety: node.unsafety.is_some(),
                generics: node.generics.ref_into(),
                trait_: node.trait_.ref_map(|(_0, _1)| _0.ref_into()),
                self_ty: node.self_ty.map_into(),
                items: node.items.map_into(),
            }
        }
    }
    impl From<&ItemImpl> for syn::ItemImpl {
        fn from(node: &ItemImpl) -> Self {
            Self {
                attrs: node.attrs.map_into(),
                modifiers: node.modifiers.ref_into(),
                unsafety: default_or_none(node.unsafety),
                impl_token: default(),
                generics: node.generics.ref_into(),
                trait_: node.trait_.ref_map(|_0| (_0.ref_into(), default())),
                self_ty: node.self_ty.map_into(),
                brace_token: default(),
                items: node.items.map_into(),
            }
        }
    }
}
