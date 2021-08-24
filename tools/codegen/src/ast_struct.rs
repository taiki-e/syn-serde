use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn_codegen::{Data, Definitions, Node, Punctuated, Type};

use crate::{convert::EMPTY_STRUCTS, file, gen};

const AST_ENUM_SRC: &str = "src/gen/ast_struct.rs";

const SKIPED: &[&str] = &[
    // data.rs
    "Field", // TODO
    // expr.rs
    "Arm",
    // generics.rs
    "Generics",
    "PredicateType", // TODO
    // item.rs
    "ItemMod", // TODO
    "ItemStruct",
    "TraitItemMethod",
    "Receiver",
    // pat.rs
    "PatOr", // TODO
    // ty.rs
    "ReturnType",
];

fn struct_attrs(ident: &str) -> TokenStream {
    match ident {
        "Lifetime" => quote!(#[derive(Clone)]),
        "BoundLifetimes" => quote!(#[derive(Default)]),
        _ => quote!(),
    }
}

// Some fields always have the same attributes.
fn field_attrs(field: &str, ty: &Type, defs: &Definitions) -> TokenStream {
    fn is_keyword(token: &str) -> bool {
        matches!(
            token,
            "Mut"
                | "Ref"
                | "Const"
                | "Dyn"
                | "Unsafe"
                | "Default"
                | "Async"
                | "Static"
                | "Move"
                | "Auto"
        )
    }

    match ty {
        Type::Box(ty) => return field_attrs(field, ty, defs),
        Type::Option(ty) => match &**ty {
            Type::Token(ty) | Type::Group(ty) => {
                let attr = quote!(#[serde(default, skip_serializing_if = "not")]);
                return if is_keyword(ty) {
                    assert!(matches!(
                        field,
                        "mutability"
                            | "by_ref"
                            | "const_token"
                            | "constness"
                            | "dyn_token"
                            | "unsafety"
                            | "defaultness"
                            | "asyncness"
                            | "movability"
                            | "capture"
                            | "auto_token"
                    ));
                    let s = &defs.tokens[ty];
                    quote! {
                        #[serde(rename = #s)]
                        #attr
                    }
                } else {
                    attr
                };
            }
            _ => return quote!(#[serde(default, skip_serializing_if = "Option::is_none")]),
        },
        Type::Syn(ty) => match &**ty {
            "Visibility" => {
                assert_eq!(field, "vis");
                return quote!(#[serde(default, skip_serializing_if = "Visibility::is_inherited")]);
            }
            "Generics" => {
                assert_eq!(field, "generics");
                return quote!(#[serde(default, skip_serializing_if = "Generics::is_none")]);
            }
            "PathArguments" => {
                assert_eq!(field, "arguments");
                return quote!(#[serde(default, skip_serializing_if = "PathArguments::is_none")]);
            }
            "TraitBoundModifier" => {
                assert_eq!(field, "modifier");
                return quote!(#[serde(default, skip_serializing_if = "TraitBoundModifier::is_none")]);
            }
            "ReturnType" => {
                assert_eq!(field, "output");
                return quote!(#[serde(default)]);
            }
            "Block" => {
                if field == "block" {
                    return quote!(#[serde(rename = "stmts")]);
                }
                // TODO: should we rename "body" to "stmts"?
                assert!(matches!(field, "body" | "then_branch"));
            }
            _ => {}
        },
        _ => {}
    }
    quote!()
}

fn skip_serializing_if(ident: &str, field: &str, ty: &Type) -> Option<String> {
    match (ident, field) {
        (_, "attrs")
        | ("Attribute", "tokens")
        | ("TypeParam", "bounds")
        | ("LifetimeDef", "bounds")
        | ("ItemTrait", "supertraits")
        | ("TraitItemType", "bounds") => Some(format!("{}::is_empty", outer_ty(ty))),
        _ => None,
    }
}

fn allow_transparent(ident: &str, field: &str, ty: &Type) -> bool {
    const DISALLOWED: &[&str] = &[
        // TODO: revisit
        "MethodTurbofish",
    ];

    field != "attrs"
        && !matches!(ty, Type::Option(_))
        && !DISALLOWED.contains(&ident)
        && !ident.starts_with("Type")
}

fn flatten(ident: &str, field: &str, ty: &Type) -> bool {
    match (field, base_ty(ty)) {
        ("member", Some("Member")) | ("mac", Some("Macro")) | ("sig", Some("Signature")) => true,
        ("lit", Some("Lit")) => ident.ends_with("Lit"),
        ("path", Some("Path")) => ident.ends_with("Path"),
        _ => false,
    }
}

fn rename<'a>(_ident: &str, field: &'a str) -> Option<&'a str> {
    field.strip_suffix('_')
}

fn base_ty(ty: &Type) -> Option<&str> {
    match ty {
        Type::Syn(ty) | Type::Ext(ty) | Type::Std(ty) => Some(ty),
        _ => None,
    }
}

fn outer_ty(ty: &Type) -> &str {
    match ty {
        Type::Box(_) => "Box",
        Type::Vec(_) => "Vec",
        Type::Punctuated(_) => "Punctuated",
        Type::Option(_) => "Option",
        Type::Syn(ty) | Type::Ext(ty) | Type::Std(ty) => ty,
        _ => unreachable!("outer_ty: {:?}", ty),
    }
}

fn format_ty(ty: &Type) -> Option<TokenStream> {
    match ty {
        Type::Box(t)
        | Type::Vec(t)
        | Type::Punctuated(Punctuated { element: t, .. })
        | Type::Option(t) => match &**t {
            Type::Token(_) | Type::Group(_) => Some(quote!(bool)),
            Type::Tuple(t) => {
                let container = format_ident!("{}", outer_ty(ty));
                let tys: Vec<_> = t.iter().filter_map(format_ty).collect();
                assert_ne!(tys.len(), 0);

                let t = if tys.len() == 1 { quote!(#(#tys)*) } else { quote!((#(#tys),*)) };
                Some(quote!(#container<#t>))
            }
            _ => {
                let container = format_ident!("{}", outer_ty(ty));
                let t = format_ty(t).unwrap_or_else(|| unimplemented!("format_ty: {:?}", ty));
                Some(quote!(#container<#t>))
            }
        },
        Type::Token(_) | Type::Group(_) => None,
        Type::Ext(t) if t == "Span" => None,
        Type::Syn(t) if t == "Reserved" => None,
        Type::Syn(t) if EMPTY_STRUCTS.contains(&&**t) => None,
        Type::Syn(t) | Type::Ext(t) | Type::Std(t) => {
            let t = format_ident!("{}", t);
            Some(quote!(#t))
        }
        Type::Tuple(_) => unreachable!("format_ty: {:?}", ty),
    }
}

fn node(impls: &mut TokenStream, node: &Node, defs: &Definitions) {
    if SKIPED.contains(&&*node.ident) || EMPTY_STRUCTS.contains(&&*node.ident) {
        return;
    }

    if let Data::Struct(fields) = &node.data {
        let mut body = Vec::new();
        let mut last = "";
        for (field, ty) in fields {
            if let Some(t) = format_ty(ty) {
                let attrs = field_attrs(field, ty, defs);
                let rename = rename(&node.ident, field).map(|s| quote!(#[serde(rename = #s)]));
                let skip_serializing_if = skip_serializing_if(&node.ident, field, ty)
                    .map(|s| quote!(#[serde(default, skip_serializing_if = #s)]));
                let flatten = if flatten(&node.ident, field, ty) {
                    quote!(#[serde(flatten)])
                } else {
                    quote!()
                };
                let f = format_ident!("{}", field);

                body.push(quote! {
                    #attrs
                    #rename
                    #skip_serializing_if
                    #flatten
                    pub(crate) #f: #t,
                });
                last = &**field;
            }
        }

        let transparent = if body.len() == 1 && allow_transparent(&node.ident, last, &fields[last])
        {
            Some(quote!(#[serde(transparent)]))
        } else {
            None
        };

        let attrs = struct_attrs(&node.ident);
        let ident = format_ident!("{}", node.ident);
        let doc = format!(" An adapter for [`struct@syn::{}`].", node.ident);
        impls.extend(quote! {
            #[doc = #doc]
            #[derive(Serialize, Deserialize)]
            #attrs
            #transparent
            pub struct #ident {
                #(#body)*
            }
        });
    }
}

pub(crate) fn generate(defs: &Definitions) -> Result<()> {
    let impls = gen::traverse(defs, node);
    let path = &file::root_dir().join(AST_ENUM_SRC);
    file::write(path, &quote! {
        use crate::*;

        #impls
    })?;
    Ok(())
}
