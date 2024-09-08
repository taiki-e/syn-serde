// SPDX-License-Identifier: Apache-2.0 OR MIT

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn_codegen::{Data, Definitions, Node, Type};

use crate::{
    convert::{EMPTY_STRUCTS, IGNORED_TYPES},
    file, gen,
};

const AST_ENUM_SRC: &str = "src/gen/ast_enum.rs";

const SKIPPED: &[&str] = &[
    // stmt.rs
    "Stmt", // TODO
];

fn rename(ident: &str, variant: &str) -> Option<&'static str> {
    match (ident, variant) {
        ("Pat", "Wild") | ("Type", "Infer") => Some("_"),
        ("Type", "Never") => Some("!"),
        ("Stmt", "Local") => Some("let"),
        ("UseTree", "Glob") => Some("*"),
        ("UseTree", "Name") | ("Member", "Named") => Some("ident"),
        ("Member", "Unnamed") => Some("index"),
        ("RangeLimits", "HalfOpen") => Some(".."),
        ("RangeLimits", "Closed") => Some("..="),
        ("Visibility", "Public") => Some("pub"),
        ("StaticMutability", "Mut") => Some("mut"),
        _ => None,
    }
}

fn node(impls: &mut TokenStream, node: &Node, defs: &Definitions) {
    if SKIPPED.contains(&&*node.ident) || IGNORED_TYPES.contains(&&*node.ident) {
        return;
    }

    if let Data::Enum(variants) = &node.data {
        let mut body = TokenStream::new();

        for (variant, fields) in variants {
            body.extend(rename(&node.ident, variant).map(|s| quote!(#[serde(rename = #s)])));

            let variant = format_ident!("{variant}");

            if fields.is_empty() {
                body.extend(quote!(#variant,));
            } else {
                assert!(fields.len() == 1 || node.ident == "Stmt");
                match &fields[0] {
                    Type::Syn(s) if EMPTY_STRUCTS.contains(&&**s) => {
                        body.extend(quote!(#variant,));
                    }
                    Type::Syn(s) | Type::Ext(s) => {
                        let ty = format_ident!("{s}");
                        body.extend(quote!(#variant(#ty),));
                    }
                    Type::Token(t) | Type::Group(t) => {
                        if matches!(&*node.ident, "BinOp" | "UnOp") {
                            let s = &defs.tokens[t];
                            body.extend(quote!(#[serde(rename = #s)]));
                        }
                        body.extend(quote!(#variant,));
                    }
                    _ => unreachable!("Data::Enum: {}", node.ident),
                }
            }
        }

        let non_exhaustive = if node.exhaustive {
            quote! {}
        } else {
            quote! { #[non_exhaustive] }
        };

        let ident = format_ident!("{}", node.ident);
        let doc = format!(" An adapter for [`enum@syn::{}`].", node.ident);
        impls.extend(quote! {
            #[doc = #doc]
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "snake_case")]
            #non_exhaustive
            pub enum #ident {
                #body
            }
        });
    }
}

pub(crate) fn generate(defs: &Definitions) {
    let impls = gen::traverse(defs, node);
    let path = &file::workspace_root().join(AST_ENUM_SRC);
    file::write(function_name!(), path, quote! {
        use crate::*;

        #impls
    })
    .unwrap();
}
