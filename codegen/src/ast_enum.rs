use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn_codegen::{Data, Definitions, Node, Type};

use crate::{convert, file, gen, Result};

const AST_ENUM_SRC: &str = "../src/gen/ast_enum.rs";

const IGNORED_TYPES: &[&str] = &[
    // renamed
    "Member",
    "RangeLimits",
    "UseTree",
    "Visibility",
    "BinOp",
    "UnOp",
    "Pat",
    "Stmt",
    "Type",
];

fn node(impls: &mut TokenStream, node: &Node, _defs: &Definitions) {
    if convert::IGNORED_TYPES.contains(&&*node.ident) || IGNORED_TYPES.contains(&&*node.ident) {
        return;
    }

    match &node.data {
        Data::Enum(variants) => {
            let mut body = TokenStream::new();

            for (variant, fields) in variants {
                let variant = format_ident!("{}", variant);

                if fields.is_empty() {
                    body.extend(quote! {
                        #variant,
                    });
                } else {
                    assert_eq!(fields.len(), 1);
                    match &fields[0] {
                        Type::Syn(s) | Type::Ext(s) => {
                            let field = format_ident!("{}", s);

                            body.extend(quote! {
                                #variant(#field),
                            });
                        }
                        Type::Token(_) | Type::Group(_) => {
                            body.extend(quote! {
                                #variant,
                            });
                        }
                        _ => unreachable!("Data::Enum: {}", node.ident),
                    }
                }
            }

            let ident = format_ident!("{}", node.ident);
            if !node.exhaustive {
                body.extend(quote! {
                    #[doc(hidden)]
                    __Nonexhaustive,
                });
            }

            impls.extend(quote! {
                #[derive(Serialize, Deserialize)]
                #[serde(rename_all = "snake_case")]
                pub enum #ident {
                    #body
                }
            });
        }
        Data::Struct(_) => {}
        Data::Private => unreachable!("Data::Private: {}", node.ident),
    }
}

pub(crate) fn generate(defs: &Definitions) -> Result<()> {
    let impls = gen::traverse(defs, node);
    let path = file::manifest_dir().join(AST_ENUM_SRC);
    file::write(path, quote! {
        use crate::*;

        #impls
    })?;
    Ok(())
}
