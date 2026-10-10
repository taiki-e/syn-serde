// SPDX-License-Identifier: Apache-2.0 OR MIT

use syn_codegen::{Definitions, Node};
use test_helper::codegen::proc_macro2::TokenStream;

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
