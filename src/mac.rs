use super::*;

ast_struct! {
    /// A macro invocation: `println!("{}", mac)`.
    pub struct Macro {
        pub(crate) path: Path,
        pub(crate) delimiter: MacroDelimiter,
        pub(crate) tokens: TokenStream,
    }
}

ast_enum! {
    /// A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`.
    pub enum MacroDelimiter {
        Paren,
        Brace,
        Bracket,
    }
}
