use proc_macro2::TokenStream;
use syn_codegen::{Definitions, Node};

pub(crate) fn traverse(
    defs: &Definitions,
    node: fn(&mut TokenStream, &Node, &Definitions),
) -> TokenStream {
    let mut types = defs.types.clone();
    types.sort_by(|a, b| a.ident.cmp(&b.ident));

    let mut impls = TokenStream::new();
    for s in types {
        if s.ident == "Reserved" {
            continue;
        }
        node(&mut impls, &s, defs);
    }

    impls
}
