use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn_codegen::{Data, Definitions, Node, Type};

use crate::{
    convert::{EMPTY_STRUCTS, IGNORED_TYPES},
    file, gen,
};

const AST_ENUM_SRC: &str = "src/gen/ast_enum.rs";

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
        _ => None,
    }
}

fn node(impls: &mut TokenStream, node: &Node, defs: &Definitions) {
    if IGNORED_TYPES.contains(&&*node.ident) {
        return;
    }

    if let Data::Enum(variants) = &node.data {
        let mut body = TokenStream::new();

        for (variant, fields) in variants {
            body.extend(rename(&node.ident, variant).map(|s| quote!(#[serde(rename = #s)])));

            let variant = format_ident!("{}", variant);

            if fields.is_empty() {
                body.extend(quote!(#variant,));
            } else {
                assert!(fields.len() == 1 || node.ident == "Stmt");
                match &fields[0] {
                    Type::Syn(s) if EMPTY_STRUCTS.contains(&&**s) => {
                        body.extend(quote!(#variant,));
                    }
                    Type::Syn(s) | Type::Ext(s) => {
                        let ty = format_ident!("{}", s);
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

        if !node.exhaustive {
            body.extend(quote! {
                #[doc(hidden)]
                __Nonexhaustive,
            });
        }

        let ident = format_ident!("{}", node.ident);
        let doc = format!(" An adapter for [`enum@syn::{}`].", node.ident);
        impls.extend(quote! {
            #[doc = #doc]
            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "snake_case")]
            pub enum #ident {
                #body
            }
        });
    }
}

pub(crate) fn generate(defs: &Definitions) -> Result<()> {
    let impls = gen::traverse(defs, node);
    let path = file::root_dir().join(AST_ENUM_SRC);
    file::write(path, quote! {
        use crate::*;

        #impls
    })?;
    Ok(())
}
