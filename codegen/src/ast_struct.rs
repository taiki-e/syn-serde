use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn_codegen::{Data, Definitions, Node, Punctuated, Type};

use crate::{convert::EMPTY_STRUCTS, file, gen, Result};

const AST_ENUM_SRC: &str = "../src/gen/ast_struct.rs";

const SKIPED: &[&str] = &[
    // lit.rs
    "LitBool", // TODO
    // attr.rs
    "Attribute",
    // data.rs
    "Field",
    // expr.rs
    "ExprAsync",
    "ExprBlock",
    "ExprForLoop",
    "ExprLit",
    "ExprLoop",
    "ExprPath",
    "ExprTryBlock",
    "ExprUnsafe",
    "ExprWhile",
    "Arm",
    // generics.rs
    "Generics",
    "TypeParam",
    "LifetimeDef",
    "PredicateType",
    // item.rs
    "ItemFn",
    "ItemImpl",
    "ItemMod",
    "ItemStruct",
    "ItemTrait",
    "TraitItemMethod",
    "TraitItemType",
    "ImplItemMethod",
    "Receiver",
    // pat.rs
    "PatOr",
    "PatPath",
    // ty.rs
    "TypePath",
    "ReturnType",
];

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
            "Mut" | "Ref" | "Const" | "Dyn" | "Unsafe" | "Default" | "Async" | "Static" | "Move"
        )
    }

    match ty {
        Type::Vec(ty) => {
            if let Type::Syn(ty) = &**ty {
                if matches!(&**ty, "Attribute") {
                    assert!(matches!(field, "attrs"));
                    return quote!(#[serde(default, skip_serializing_if = "Vec::is_empty")]);
                }
            }
        }
        Type::Option(ty) => match &**ty {
            Type::Token(ty) | Type::Group(ty) => {
                let attr = quote!(#[serde(default, skip_serializing_if = "not")]);
                if is_keyword(ty) {
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
                    ));
                    let s = &defs.tokens[ty];
                    return quote! {
                        #[serde(rename = #s)]
                        #attr
                    };
                } else {
                    return attr;
                }
            }
            _ => return quote!(#[serde(default, skip_serializing_if = "Option::is_none")]),
        },
        Type::Syn(ty) => match &**ty {
            "Member" | "Macro" | "Signature" => {
                assert!(matches!(field, "member" | "mac" | "sig"));
                return quote!(#[serde(flatten)]);
            }
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
            _ => {}
        },
        _ => {}
    }
    quote!()
}

fn rename(_ident: &str, _field: &str) -> Option<&'static str> {
    None
}

fn container_ty(ty: &Type) -> Ident {
    match ty {
        Type::Box(_) => format_ident!("Box"),
        Type::Vec(_) => format_ident!("Vec"),
        Type::Punctuated(_) => format_ident!("Punctuated"),
        Type::Option(_) => format_ident!("Option"),
        _ => unreachable!("container_ty: {:?}", ty),
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
                let container = container_ty(ty);
                let tys: Vec<_> = t.iter().filter_map(format_ty).collect();
                assert_ne!(tys.len(), 0);

                let t = if tys.len() == 1 { quote!(#(#tys)*) } else { quote!((#(#tys),*)) };
                Some(quote!(#container<#t>))
            }
            _ => {
                let container = container_ty(ty);
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
                let f = format_ident!("{}", field);

                body.push(quote! {
                    #attrs
                    #rename
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
    let path = file::manifest_dir().join(AST_ENUM_SRC);
    file::write(path, quote! {
        use crate::*;

        #impls
    })?;
    Ok(())
}
