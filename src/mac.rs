use super::*;

ast_struct! {
    /// A macro invocation: `println!("{}", mac)`.
    pub struct Macro {
        pub path: Path,
        pub delimiter: MacroDelimiter,
        pub tts: TokenStream,
    }
}

ast_enum! {
    /// A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`.
    pub enum MacroDelimiter #manual_from_impl {
        Paren,
        Brace,
        Bracket,
    }
}

mod convert {
    use super::*;

    // Macro

    impl From<&syn::Macro> for Macro {
        fn from(other: &syn::Macro) -> Self {
            Self {
                path: other.path.ref_into(),
                delimiter: other.delimiter.ref_into(),
                tts: other.tts.ref_into(),
            }
        }
    }

    impl From<&Macro> for syn::Macro {
        fn from(other: &Macro) -> Self {
            Self {
                path: other.path.ref_into(),
                bang_token: default(),
                delimiter: other.delimiter.ref_into(),
                tts: other.tts.ref_into(),
            }
        }
    }

    // MacroDelimiter

    impl From<&syn::MacroDelimiter> for MacroDelimiter {
        fn from(other: &syn::MacroDelimiter) -> Self {
            use super::MacroDelimiter::*;
            use syn::MacroDelimiter;
            match other {
                MacroDelimiter::Paren(_) => Paren,
                MacroDelimiter::Brace(_) => Brace,
                MacroDelimiter::Bracket(_) => Bracket,
            }
        }
    }

    impl From<&MacroDelimiter> for syn::MacroDelimiter {
        fn from(other: &MacroDelimiter) -> Self {
            use syn::MacroDelimiter::*;
            match other {
                MacroDelimiter::Paren => Paren(default()),
                MacroDelimiter::Brace => Brace(default()),
                MacroDelimiter::Bracket => Bracket(default()),
            }
        }
    }
}
