use super::*;

ast_struct! {
    /// A Rust lifetime: `'a`.
    ///
    /// Lifetime names must conform to the following rules:
    ///
    /// - Must start with an apostrophe.
    /// - Must not consist of just an apostrophe: `'`.
    /// - Character after the apostrophe must be `_` or a Unicode code point with
    ///   the XID_Start property.
    /// - All following characters must be Unicode code points with the XID_Continue
    ///   property.
    #[derive(Clone)]
    #[serde(transparent)]
    pub struct Lifetime {
        pub(crate) ident: Ident,
    }
}
