use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn_codegen::{Data, Definitions, Node, Type};

use crate::{file, gen, Result};

const CONVERT_SRC: &str = "../src/gen/convert.rs";

pub(crate) const IGNORED_TYPES: &[&str] = &[
    /* we don't have them */
    "DeriveInput",
    "Data",
    "DataStruct",
    "DataEnum",
    "DataUnion",
    /* private */
    "LitByte",
    "LitByteStr",
    "LitChar",
    "LitFloat",
    "LitInt",
    "LitStr",
    /* cannot be implemented by codegen */
    "Type",
    "UseTree",
    "Visibility",
    "Receiver",
    /* optimize */
    "Generics",
    "ExprMatch",
    "Arm",
    "TraitItemMethod",
    "ItemStruct",
    "ReturnType",
];
const EMPTY_STRUCTS: &[&str] = &["TypeInfer", "TypeNever", "UseGlob", "VisCrate", "VisPublic"];

fn visit(ty: &Type, name: &TokenStream) -> (Option<TokenStream>, TokenStream) {
    match ty {
        Type::Box(_) | Type::Vec(_) | Type::Punctuated(_) => {
            let from = Some(quote!(#name.map_into()));
            let into = quote!(#name.map_into());
            (from, into)
        }
        Type::Option(t) => match &**t {
            Type::Token(_) | Type::Group(_) => {
                let from = Some(quote!(#name.is_some()));
                let into = quote!(default_or_none(#name));
                (from, into)
            }
            Type::Tuple(t) => {
                let mut from_expr = Vec::new();
                let mut from_pat = Vec::new();
                let mut into_expr = Vec::new();
                let mut into_pat = Vec::new();

                for (i, t) in t.iter().enumerate() {
                    let id = format_ident!("_{}", i);
                    let (from, into) = visit(t, &quote!((*#id)));

                    from_pat.push(id.clone());
                    into_expr.push(into);
                    if from.is_some() {
                        into_pat.push(id);
                        from_expr.push(from);
                    }
                }
                assert_eq!(from_pat.len(), into_expr.len());
                assert_eq!(into_pat.len(), from_expr.len());
                assert_ne!(into_pat.len(), 0);

                if into_pat.len() == 1 {
                    let from = Some(quote!(#name.ref_map(|(#(#from_pat),*)| #(#from_expr),*)));
                    let into = quote!(#name.ref_map(|#(#into_pat),*| (#(#into_expr),*)));
                    (from, into)
                } else {
                    let from = Some(quote!(#name.ref_map(|(#(#from_pat),*)| (#(#from_expr),*))));
                    let into = quote!(#name.ref_map(|(#(#into_pat),*)| (#(#into_expr),*)));
                    (from, into)
                }
            }
            Type::Box(_) | Type::Vec(_) | Type::Punctuated(_) => {
                let from = Some(quote!(#name.ref_map(MapInto::map_into)));
                let into = quote!(#name.ref_map(MapInto::map_into));
                (from, into)
            }
            Type::Std(t) if t == "String" => {
                // `From<&String> for String` requires Rust 1.36 or later.
                // Refs: https://github.com/rust-lang/rust/pull/59825
                let from = Some(quote!(#name.ref_map(ToString::to_string)));
                let into = quote!(#name.ref_map(ToString::to_string));
                (from, into)
            }
            _ => {
                let from = Some(quote!(#name.map_into()));
                let into = quote!(#name.map_into());
                (from, into)
            }
        },
        Type::Token(_) | Type::Group(_) => {
            let from = None;
            let into = quote!(default());
            (from, into)
        }
        Type::Syn(t) if t == "Reserved" => {
            let from = None;
            let into = quote!(default());
            (from, into)
        }
        Type::Ext(t) if t == "Span" => {
            let from = None;
            let into = quote!(proc_macro2::Span::call_site());
            (from, into)
        }
        Type::Syn(_) | Type::Ext(_) => {
            let from = Some(quote!(#name.ref_into()));
            let into = quote!(#name.ref_into());
            (from, into)
        }
        Type::Std(t) => {
            if let "usize" | "u32" | "bool" = &**t {
                let from = Some(quote!(#name));
                let into = quote!(#name);
                (from, into)
            } else {
                let from = Some(quote!(#name.into()));
                let into = quote!(#name.into());
                (from, into)
            }
        }
        Type::Tuple(t) => unreachable!("Type::Tuple: {:?}", t),
    }
}

fn node(impls: &mut TokenStream, node: &Node, _defs: &Definitions) {
    if IGNORED_TYPES.contains(&&*node.ident) {
        return;
    }

    let ty = format_ident!("{}", &node.ident);

    let mut from_impl = TokenStream::new();
    let mut into_impl = TokenStream::new();

    match &node.data {
        Data::Enum(variants) => {
            let mut from_variants = TokenStream::new();
            let mut into_variants = TokenStream::new();

            for (variant, fields) in variants {
                let variant_ident = format_ident!("{}", variant);

                if fields.is_empty() {
                    from_variants.extend(quote! {
                        syn::#ty::#variant_ident => {
                            #ty::#variant_ident
                        }
                    });
                    into_variants.extend(quote! {
                        #ty::#variant_ident => {
                            syn::#ty::#variant_ident
                        }
                    });
                } else {
                    let mut from_expr = Vec::new();
                    let mut from_pat = Vec::new();
                    let mut into_expr = Vec::new();
                    let mut into_pat = Vec::new();

                    for (i, t) in fields.iter().enumerate() {
                        let id = format_ident!("_{}", i);
                        let (from, into) = visit(t, &quote!((*#id)));

                        from_pat.push(id.clone());
                        into_expr.push(into);
                        if from.is_some() {
                            into_pat.push(id);
                            from_expr.push(from);
                        }
                    }

                    if from_expr.is_empty() {
                        from_variants.extend(quote! {
                            syn::#ty::#variant_ident(..) => {
                                #ty::#variant_ident
                            }
                        });
                        into_variants.extend(quote! {
                            #ty::#variant_ident => {
                                syn::#ty::#variant_ident(#(#into_expr),*)
                            }
                        });
                    } else {
                        from_variants.extend(quote! {
                            syn::#ty::#variant_ident(#(#from_pat),*) => {
                                #ty::#variant_ident(#(#from_expr),*)
                            }
                        });
                        into_variants.extend(quote! {
                            #ty::#variant_ident(#(#into_pat),*) => {
                                syn::#ty::#variant_ident(#(#into_expr),*)
                            }
                        });
                    }
                }
            }

            let nonexhaustive =
                if node.exhaustive { None } else { Some(quote!(_ => unreachable!())) };

            from_impl.extend(quote! {
                match node {
                    #from_variants
                    #nonexhaustive
                }
            });
            into_impl.extend(quote! {
                match node {
                    #into_variants
                    #nonexhaustive
                }
            });
        }
        Data::Struct(fields) => {
            let mut from_fields = TokenStream::new();
            let mut into_fields = TokenStream::new();

            for (field, ty) in fields {
                let id = format_ident!("{}", field);
                let ref_toks = quote!(node.#id);

                let (from, into) = visit(&ty, &ref_toks);

                if from.is_some() {
                    from_fields.extend(quote! {
                        #id: #from,
                    });
                }
                into_fields.extend(quote! {
                    #id: #into,
                });
            }

            assert!(!fields.is_empty(), "fields.is_empty: {}", ty);
            if !from_fields.is_empty() {
                from_impl.extend(quote! {
                    #ty {
                        #from_fields
                    }
                });
                into_impl.extend(quote! {
                    syn::#ty {
                        #into_fields
                    }
                });
            } else {
                assert!(EMPTY_STRUCTS.contains(&&*node.ident), "from_fields.is_empty(): {}", ty);
                return;
            }
        }
        Data::Private => unreachable!("Data::Private: {}", ty),
    }

    impls.extend(quote! {
        syn_trait_impl!(syn::#ty);
        impl From<&syn::#ty> for #ty {
            fn from(node: &syn::#ty) -> Self {
                #from_impl
            }
        }
        impl From<&#ty> for syn::#ty {
            fn from(node: &#ty) -> Self {
                #into_impl
            }
        }
    });
}

pub(crate) fn generate(defs: &Definitions) -> Result<()> {
    let impls = gen::traverse(defs, node);
    file::write(CONVERT_SRC, quote! {
        #![allow(unused_parens)]
        #![allow(clippy::double_parens, clippy::just_underscores_and_digits)]

        use crate::*;

        #impls
    })?;
    Ok(())
}
