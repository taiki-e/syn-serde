use super::*;

#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use crate::ast_enum::MacroDelimiter;

ast_struct! {
    /// A macro invocation: `println!("{}", mac)`.
    pub struct Macro {
        pub(crate) path: Path,
        pub(crate) delimiter: MacroDelimiter,
        pub(crate) tokens: TokenStream,
    }
}
