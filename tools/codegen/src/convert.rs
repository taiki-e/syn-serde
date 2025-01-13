// SPDX-License-Identifier: Apache-2.0 OR MIT

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn_codegen::{Data, Definitions, Node, Type};

use crate::{file, traverse};

const CONVERT_SRC: &str = "src/gen/convert.rs";

// optimize
pub(crate) const IGNORED_TYPES: &[&str] =
    &["Arm", "ExprMatch", "Generics", "ItemStruct", "Receiver", "ReturnType", "TraitItemFn"];

pub(crate) const EMPTY_STRUCTS: &[&str] =
    &["TypeInfer", "TypeNever", "UseGlob", "VisCrate", "VisPublic"];

fn visit(ty: &Type, var: &TokenStream, defs: &Definitions) -> (Option<TokenStream>, TokenStream) {
    match ty {
        Type::Box(_) | Type::Vec(_) | Type::Punctuated(_) => {
            let from = Some(quote!(#var.map_into()));
            let into = quote!(#var.map_into());
            (from, into)
        }
        Type::Option(t) => match &**t {
            Type::Token(_) | Type::Group(_) => {
                let from = Some(quote!(#var.is_some()));
                let into = quote!(default_or_none(#var));
                (from, into)
            }
            Type::Tuple(t) => {
                let mut from_expr = Vec::with_capacity(t.len());
                let mut from_pat = Vec::with_capacity(t.len());
                let mut into_expr = Vec::with_capacity(t.len());
                let mut into_pat = Vec::with_capacity(t.len());

                for (i, t) in t.iter().enumerate() {
                    let id = format_ident!("_{i}");
                    let (from, into) = visit(t, &quote!((*#id)), defs);

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
                    let from = Some(quote!(#var.ref_map(|(#(#from_pat),*)| #(#from_expr),*)));
                    let into = quote!(#var.ref_map(|#(#into_pat),*| (#(#into_expr),*)));
                    (from, into)
                } else {
                    let from = Some(quote!(#var.ref_map(|(#(#from_pat),*)| (#(#from_expr),*))));
                    let into = quote!(#var.ref_map(|(#(#into_pat),*)| (#(#into_expr),*)));
                    (from, into)
                }
            }
            Type::Box(_) | Type::Vec(_) | Type::Punctuated(_) => {
                let from = Some(quote!(#var.ref_map(MapInto::map_into)));
                let into = quote!(#var.ref_map(MapInto::map_into));
                (from, into)
            }
            _ => {
                let from = Some(quote!(#var.map_into()));
                let into = quote!(#var.map_into());
                (from, into)
            }
        },
        Type::Token(_) | Type::Group(_) => {
            let from = None;
            let into = quote!(default());
            (from, into)
        }
        Type::Ext(t) if t == "Span" => {
            let from = None;
            let into = quote!(proc_macro2::Span::call_site());
            (from, into)
        }
        Type::Syn(t) if t == "Reserved" => {
            let from = None;
            let into = quote!(default());
            (from, into)
        }
        Type::Syn(t) if EMPTY_STRUCTS.contains(&&**t) => {
            let node = &defs.types[defs.types.iter().position(|node| node.ident == *t).unwrap()];
            let ident = format_ident!("{}", node.ident);
            if let Data::Struct(fields) = &node.data {
                let from = None;
                let fields = fields.keys().map(|f| format_ident!("{f}"));
                let into = quote!(syn::#ident { #(#fields: default(),)* });
                return (from, into);
            }
            unreachable!()
        }
        Type::Syn(_) | Type::Ext(_) => {
            let from = Some(quote!(#var.ref_into()));
            let into = quote!(#var.ref_into());
            (from, into)
        }
        Type::Std(t) => {
            if let "usize" | "u32" | "bool" = &**t {
                let from = Some(quote!(#var));
                let into = quote!(#var);
                (from, into)
            } else {
                let from = Some(quote!(#var.into()));
                let into = quote!(#var.into());
                (from, into)
            }
        }
        Type::Tuple(t) => unreachable!("Type::Tuple: {:?}", t),
    }
}

fn node(impls: &mut TokenStream, node: &Node, defs: &Definitions) {
    if IGNORED_TYPES.contains(&&*node.ident) || EMPTY_STRUCTS.contains(&&*node.ident) {
        return;
    }

    let ident = format_ident!("{}", &node.ident);
    let mut from_impl = TokenStream::new();
    let mut into_impl = TokenStream::new();

    match &node.data {
        Data::Enum(variants) => {
            let mut from_variants = TokenStream::new();
            let mut into_variants = TokenStream::new();

            for (variant, fields) in variants {
                let variant = format_ident!("{variant}");

                if fields.is_empty() {
                    from_variants.extend(quote! {
                        syn::#ident::#variant => #ident::#variant,
                    });
                    into_variants.extend(quote! {
                        #ident::#variant => syn::#ident::#variant,
                    });
                    continue;
                }

                let mut from_expr = Vec::with_capacity(fields.len());
                let mut from_pat = Vec::with_capacity(fields.len());
                let mut into_expr = Vec::with_capacity(fields.len());
                let mut into_pat = Vec::with_capacity(fields.len());

                for (i, t) in fields.iter().enumerate() {
                    let id = format_ident!("_{i}");
                    let (from, into) = visit(t, &quote!((*#id)), defs);

                    from_pat.push(id.clone());
                    into_expr.push(into);
                    if from.is_some() {
                        into_pat.push(id);
                        from_expr.push(from);
                    }
                }

                if from_expr.is_empty() {
                    from_variants.extend(quote! {
                        syn::#ident::#variant(..) => #ident::#variant,
                    });
                    into_variants.extend(quote! {
                        #ident::#variant => syn::#ident::#variant(#(#into_expr),*),
                    });
                } else {
                    from_variants.extend(quote! {
                        syn::#ident::#variant(#(#from_pat),*) => #ident::#variant(#(#from_expr),*),
                    });
                    into_variants.extend(quote! {
                        #ident::#variant(#(#into_pat),*) => syn::#ident::#variant(#(#into_expr),*),
                    });
                }
            }

            let mut non_exhaustive =
                if node.exhaustive { None } else { Some(quote!(_ => unreachable!())) };

            from_impl.extend(quote! {
                match node {
                    #from_variants
                    #non_exhaustive
                }
            });
            if !variants.is_empty() {
                non_exhaustive = None;
            }
            into_impl.extend(quote! {
                match node {
                    #into_variants
                    #non_exhaustive
                }
            });
        }
        Data::Struct(fields) => {
            let mut from_fields = TokenStream::new();
            let mut into_fields = TokenStream::new();

            for (field, ty) in fields {
                let field = format_ident!("{field}");
                let ref_tokens = quote!(node.#field);

                let (from, into) = visit(ty, &ref_tokens, defs);

                if from.is_some() {
                    from_fields.extend(quote!(#field: #from,));
                }
                into_fields.extend(quote!(#field: #into,));
            }

            assert!(!fields.is_empty(), "fields.is_empty: {ident}");
            assert!(!from_fields.is_empty(), "from_fields.is_empty(): {ident}");

            from_impl.extend(quote! {
                Self { #from_fields }
            });
            into_impl.extend(quote! {
                Self { #into_fields }
            });
        }
        Data::Private => return,
    }

    impls.extend(quote! {
        syn_trait_impl!(syn::#ident);
        impl From<&syn::#ident> for #ident {
            fn from(node: &syn::#ident) -> Self {
                #from_impl
            }
        }
        impl From<&#ident> for syn::#ident {
            fn from(node: &#ident) -> Self {
                #into_impl
            }
        }
    });
}

pub(crate) fn generate(defs: &Definitions) {
    let impls = traverse::traverse(defs, node);
    let path = &file::workspace_root().join(CONVERT_SRC);
    file::write(function_name!(), path, quote! {
        #![allow(unused_parens)]
        #![allow(
            clippy::double_parens,
            clippy::just_underscores_and_digits,
            clippy::match_single_binding,
        )]

        use crate::*;

        #impls
    })
    .unwrap();
}
