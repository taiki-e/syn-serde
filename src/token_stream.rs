use super::*;
use std::fmt;

ast_struct! {
    /// An abstract stream of tokens, or more concretely a sequence of token trees.
    ///
    /// This type provides interfaces for iterating over token trees and for
    /// collecting token trees into one stream.
    #[derive(Clone, Default)]
    #[serde(transparent)]
    pub struct TokenStream {
        inner: Vec<TokenTree>,
    }
}

impl TokenStream {
    fn _new(inner: Vec<TokenTree>) -> Self {
        Self { inner }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

ast_enum! {
    /// A single token or a delimited sequence of token trees (e.g. `[1, (), ..]`).
    #[derive(Clone)]
    pub enum TokenTree {
        /// A token stream surrounded by bracket delimiters.
        Group(Group),
        /// An identifier.
        Ident(Ident),
        /// A single punctuation character (`+`, `,`, `$`, etc.).
        Punct(Punct),
        /// A literal character (`'a'`), string (`"hello"`), number (`2.3`), etc.
        #[serde(rename = "lit")]
        Literal(Literal),
    }
}

ast_struct! {
    /// A delimited token stream.
    ///
    /// A `Group` internally contains a `TokenStream` which is surrounded by
    /// `Delimiter`s.
    #[derive(Clone)]
    pub struct Group {
        delimiter: Delimiter,
        stream: TokenStream,
    }
}

ast_enum! {
    /// Describes how a sequence of token trees is delimited.
    #[derive(Clone, Copy)]
    pub enum Delimiter {
        /// `( ... )`
        Parenthesis,
        /// `{ ... }`
        Brace,
        /// `[ ... ]`
        Bracket,
        /// `Ø ... Ø`
        ///
        /// An implicit delimiter, that may, for example, appear around tokens
        /// coming from a "macro variable" `$var`. It is important to preserve
        /// operator priorities in cases like `$var * 3` where `$var` is `1 + 2`.
        /// Implicit delimiters may not survive roundtrip of a token stream through
        /// a string.
        None,
    }
}

ast_struct! {
    /// An `Punct` is an single punctuation character like `+`, `-` or `#`.
    ///
    /// Multicharacter operators like `+=` are represented as two instances of
    /// `Punct` with different forms of `Spacing` returned.
    #[derive(Clone, Copy)]
    pub struct Punct {
        op: char,
        spacing: Spacing,
    }
}

ast_enum! {
    /// Whether an `Punct` is followed immediately by another `Punct` or followed by
    /// another token or whitespace.
    #[derive(Clone, Copy)]
    pub enum Spacing {
        /// E.g. `+` is `Alone` in `+ =`, `+ident` or `+()`.
        Alone,
        /// E.g. `+` is `Joint` in `+=` or `'#`.
        ///
        /// Additionally, single quote `'` can join with identifiers to form
        /// lifetimes `'ident`.
        Joint,
    }
}

ast_struct! {
    /// A word of Rust code, which may be a keyword or legal variable name.
    ///
    /// An identifier consists of at least one Unicode code point, the first of
    /// which has the XID_Start property and the rest of which have the XID_Continue
    /// property.
    ///
    /// - The empty string is not an identifier. Use `Option<Ident>`.
    /// - A lifetime is not an identifier. Use `syn::Lifetime` instead.
    #[derive(Clone, Eq, PartialEq)]
    #[serde(transparent)]
    pub struct Ident {
        inner: String,
    }
}

ast_struct! {
    /// A literal string (`"hello"`), byte string (`b"hello"`), character (`'a'`),
    /// byte character (`b'a'`), an integer or floating point number with or without
    /// a suffix (`1`, `1u8`, `2.3`, `2.3f32`).
    ///
    /// Boolean literals like `true` and `false` do not belong here, they are
    /// `Ident`s.
    #[derive(Clone)]
    #[serde(transparent)]
    pub struct Literal {
        pub(crate) text: String,
    }
}

impl Literal {
    fn _new(text: String) -> Self {
        Self { text }
    }

    pub(crate) fn u8_suffixed(n: u8) -> Self {
        Self::_new(format!(concat!("{}", stringify!(u8)), n))
    }

    pub(crate) fn string(t: &str) -> Self {
        let mut s = t.chars().flat_map(char::escape_default).collect::<String>();
        s.push('"');
        s.insert(0, '"');
        Self::_new(s)
    }

    pub(crate) fn character(t: char) -> Self {
        Self::_new(format!("'{}'", t.escape_default().collect::<String>()))
    }

    #[allow(clippy::match_overlapping_arm)]
    pub(crate) fn byte_string(bytes: &[u8]) -> Self {
        let mut escaped = "b\"".to_string();
        for b in bytes {
            match *b {
                b'\0' => escaped.push_str(r"\0"),
                b'\t' => escaped.push_str(r"\t"),
                b'\n' => escaped.push_str(r"\n"),
                b'\r' => escaped.push_str(r"\r"),
                b'"' => escaped.push_str("\\\""),
                b'\\' => escaped.push_str("\\\\"),
                b'\x20'..=b'\x7E' => escaped.push(*b as char),
                _ => escaped.push_str(&format!("\\x{:02X}", b)),
            }
        }
        escaped.push('"');
        Self::_new(escaped)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.text.fmt(f)
    }
}

mod convert {
    use super::*;

    // TokenStream
    syn_trait_impl!(proc_macro2::TokenStream);
    impl From<&proc_macro2::TokenStream> for TokenStream {
        fn from(other: &proc_macro2::TokenStream) -> Self {
            Self::_new(
                other
                    .clone()
                    .into_iter()
                    .map::<TokenTree, _>(|x| x.ref_into())
                    .collect(),
            )
        }
    }
    impl From<&TokenStream> for proc_macro2::TokenStream {
        fn from(other: &TokenStream) -> Self {
            other
                .inner
                .iter()
                .map::<proc_macro2::TokenTree, _>(Into::into)
                .collect()
        }
    }

    // TokenTree
    syn_trait_impl!(proc_macro2::TokenTree);
    impl From<&proc_macro2::TokenTree> for TokenTree {
        fn from(other: &proc_macro2::TokenTree) -> Self {
            use super::TokenTree::*;
            use proc_macro2::TokenTree;
            match other {
                TokenTree::Group(t) => Group(t.into()),
                TokenTree::Ident(t) => Ident(t.into()),
                TokenTree::Punct(t) => Punct(t.into()),
                TokenTree::Literal(t) => Literal(t.into()),
            }
        }
    }
    impl From<&TokenTree> for proc_macro2::TokenTree {
        fn from(other: &TokenTree) -> Self {
            use proc_macro2::TokenTree::*;
            match other {
                TokenTree::Group(t) => Group(t.into()),
                TokenTree::Ident(t) => Ident(t.into()),
                TokenTree::Punct(t) => Punct(t.into()),
                TokenTree::Literal(t) => Literal(t.into()),
            }
        }
    }

    // Group
    syn_trait_impl!(proc_macro2::Group);
    impl From<&proc_macro2::Group> for Group {
        fn from(other: &proc_macro2::Group) -> Self {
            Self {
                delimiter: other.delimiter().ref_into(),
                stream: other.stream().ref_into(),
            }
        }
    }
    impl From<&Group> for proc_macro2::Group {
        fn from(other: &Group) -> Self {
            Self::new(other.delimiter.ref_into(), other.stream.ref_into())
        }
    }

    // Delimiter
    syn_trait_impl!(proc_macro2::Delimiter);
    impl From<&proc_macro2::Delimiter> for Delimiter {
        fn from(other: &proc_macro2::Delimiter) -> Self {
            use super::Delimiter::*;
            use proc_macro2::Delimiter;
            match other {
                Delimiter::Parenthesis => Parenthesis,
                Delimiter::Brace => Brace,
                Delimiter::Bracket => Bracket,
                Delimiter::None => None,
            }
        }
    }
    impl From<&Delimiter> for proc_macro2::Delimiter {
        fn from(other: &Delimiter) -> Self {
            use proc_macro2::Delimiter::*;
            match other {
                Delimiter::Parenthesis => Parenthesis,
                Delimiter::Brace => Brace,
                Delimiter::Bracket => Bracket,
                Delimiter::None => None,
            }
        }
    }

    // Ident
    syn_trait_impl!(proc_macro2::Ident);
    impl From<&proc_macro2::Ident> for Ident {
        fn from(other: &proc_macro2::Ident) -> Self {
            Self {
                inner: other.to_string(),
            }
        }
    }
    impl From<&Ident> for proc_macro2::Ident {
        fn from(other: &Ident) -> Self {
            Self::new(&other.inner, Span::call_site())
        }
    }

    // Punct
    syn_trait_impl!(proc_macro2::Punct);
    impl From<&proc_macro2::Punct> for Punct {
        fn from(other: &proc_macro2::Punct) -> Self {
            Self {
                op: other.as_char(),
                spacing: other.spacing().ref_into(),
            }
        }
    }
    impl From<&Punct> for proc_macro2::Punct {
        fn from(other: &Punct) -> Self {
            Self::new(other.op, other.spacing.ref_into())
        }
    }

    // Spacing
    syn_trait_impl!(proc_macro2::Spacing);
    impl From<&proc_macro2::Spacing> for Spacing {
        fn from(other: &proc_macro2::Spacing) -> Self {
            use super::Spacing::*;
            use proc_macro2::Spacing;
            match other {
                Spacing::Alone => Alone,
                Spacing::Joint => Joint,
            }
        }
    }
    impl From<&Spacing> for proc_macro2::Spacing {
        fn from(other: &Spacing) -> Self {
            use proc_macro2::Spacing::*;
            match other {
                Spacing::Alone => Alone,
                Spacing::Joint => Joint,
            }
        }
    }

    // Literal
    syn_trait_impl!(proc_macro2::Literal);
    impl From<&proc_macro2::Literal> for Literal {
        fn from(other: &proc_macro2::Literal) -> Self {
            Self {
                text: other.to_string(),
            }
        }
    }
    impl From<&Literal> for proc_macro2::Literal {
        fn from(other: &Literal) -> Self {
            use proc_macro2::*;
            let stream = other.text.parse::<TokenStream>().unwrap();
            match stream.into_iter().next().unwrap() {
                TokenTree::Literal(l) => l,
                _ => unreachable!(),
            }
        }
    }
}
