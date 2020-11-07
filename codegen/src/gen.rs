use proc_macro2::TokenStream;
use syn_codegen::{Definitions, Node};

pub(crate) fn traverse(
    defs: &Definitions,
    node: fn(&mut TokenStream, &Node, &Definitions),
) -> TokenStream {
    let mut types = defs.types.clone();
    types.sort_by(|a, b| a.ident.cmp(&b.ident));

    let mut impls = TokenStream::new();
    for ty in types.iter().filter(|ty| {
        // We don't provide types that are not available only with "full" feature
        (ty.features.any.is_empty() || ty.features.any.contains("full")) && ty.ident != "Reserved"
    }) {
        node(&mut impls, ty, defs);
    }

    impls
}
