// SPDX-License-Identifier: Apache-2.0 OR MIT

use core::fmt::{self, Write as _};

use super::*;

ast_struct! {
    /// An adapter for [`struct@proc_macro2::TokenStream`].
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
}

ast_enum! {
    /// An adapter for [`enum@proc_macro2::TokenTree`].
    #[derive(Clone)]
    pub enum TokenTree {
        Group(Group),
        Ident(Ident),
        Punct(Punct),
        #[serde(rename = "lit")]
        Literal(Literal),
    }
}

ast_struct! {
    /// An adapter for [`struct@proc_macro2::Group`].
    #[derive(Clone)]
    pub struct Group {
        delimiter: Delimiter,
        stream: TokenStream,
    }
}

ast_enum! {
    /// An adapter for [`enum@proc_macro2::Delimiter`].
    #[derive(Clone, Copy)]
    pub enum Delimiter {
        Parenthesis,
        Brace,
        Bracket,
        None,
    }
}

ast_struct! {
    /// An adapter for [`struct@proc_macro2::Punct`].
    #[derive(Clone, Copy)]
    pub struct Punct {
        op: char,
        spacing: Spacing,
    }
}

ast_enum! {
    /// An adapter for [`enum@proc_macro2::Spacing`].
    #[derive(Clone, Copy)]
    pub enum Spacing {
        Alone,
        Joint,
    }
}

ast_struct! {
    /// An adapter for [`struct@proc_macro2::Ident`].
    #[derive(Clone, Eq, PartialEq)]
    #[serde(transparent)]
    pub struct Ident {
        inner: String,
    }
}

ast_struct! {
    /// An adapter for [`struct@proc_macro2::Literal`].
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
        let mut escaped = "b\"".to_owned();
        for b in bytes {
            match *b {
                b'\0' => escaped.push_str(r"\0"),
                b'\t' => escaped.push_str(r"\t"),
                b'\n' => escaped.push_str(r"\n"),
                b'\r' => escaped.push_str(r"\r"),
                b'"' => escaped.push_str("\\\""),
                b'\\' => escaped.push_str("\\\\"),
                b'\x20'..=b'\x7E' => escaped.push(*b as char),
                _ => {
                    let _ = write!(escaped, "\\x{b:02X}");
                }
            }
        }
        escaped.push('"');
        Self::_new(escaped)
    }
}

// TODO: when release the next minor version, remove this.
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.text, f)
    }
}

mod convert {
    use super::*;

    // TokenStream
    syn_trait_impl!(proc_macro2::TokenStream);
    impl From<&proc_macro2::TokenStream> for TokenStream {
        fn from(other: &proc_macro2::TokenStream) -> Self {
            Self::_new(other.clone().into_iter().map::<TokenTree, _>(|x| x.ref_into()).collect())
        }
    }
    impl From<&TokenStream> for proc_macro2::TokenStream {
        fn from(other: &TokenStream) -> Self {
            other.inner.iter().map::<proc_macro2::TokenTree, _>(Into::into).collect()
        }
    }

    // TokenTree
    syn_trait_impl!(proc_macro2::TokenTree);
    impl From<&proc_macro2::TokenTree> for TokenTree {
        fn from(other: &proc_macro2::TokenTree) -> Self {
            use super::TokenTree::*;
            match other {
                proc_macro2::TokenTree::Group(t) => Group(t.into()),
                proc_macro2::TokenTree::Ident(t) => Ident(t.into()),
                proc_macro2::TokenTree::Punct(t) => Punct(t.into()),
                proc_macro2::TokenTree::Literal(t) => Literal(t.into()),
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
            Self { delimiter: other.delimiter().ref_into(), stream: other.stream().ref_into() }
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
            match other {
                proc_macro2::Delimiter::Parenthesis => Parenthesis,
                proc_macro2::Delimiter::Brace => Brace,
                proc_macro2::Delimiter::Bracket => Bracket,
                proc_macro2::Delimiter::None => None,
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
            Self { inner: other.to_string() }
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
            Self { op: other.as_char(), spacing: other.spacing().ref_into() }
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
            match other {
                proc_macro2::Spacing::Alone => Alone,
                proc_macro2::Spacing::Joint => Joint,
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
            Self { text: other.to_string() }
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
