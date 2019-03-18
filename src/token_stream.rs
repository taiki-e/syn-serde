use super::*;
use proc_macro2::{LexError, Span};
use std::fmt::{self, Display};
use std::iter::FromIterator;
use std::str::FromStr;
use std::vec::IntoIter;

/// An abstract stream of tokens, or more concretely a sequence of token trees.
///
/// This type provides interfaces for iterating over token trees and for
/// collecting token trees into one stream.
#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(transparent)]
pub struct TokenStream {
    inner: Vec<TokenTree>,
}

impl TokenStream {
    fn _new(inner: Vec<TokenTree>) -> Self {
        Self { inner }
    }

    /// Returns an empty `TokenStream` containing no token trees.
    pub fn new() -> Self {
        Self::_new(Vec::new())
    }

    /// Checks if this `TokenStream` is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl FromStr for TokenStream {
    type Err = LexError;

    fn from_str(src: &str) -> Result<Self, LexError> {
        <proc_macro2::TokenStream>::from_str(src).map(|x| Self::from(&x))
    }
}

impl Extend<TokenTree> for TokenStream {
    fn extend<I: IntoIterator<Item = TokenTree>>(&mut self, streams: I) {
        self.inner.extend(streams)
    }
}

impl Extend<TokenStream> for TokenStream {
    fn extend<I: IntoIterator<Item = TokenStream>>(&mut self, streams: I) {
        self.inner
            .extend(streams.into_iter().flat_map(|stream| stream.inner))
    }
}

impl FromIterator<TokenTree> for TokenStream {
    fn from_iter<I: IntoIterator<Item = TokenTree>>(streams: I) -> Self {
        Self::_new(streams.into_iter().collect())
    }
}

impl FromIterator<TokenStream> for TokenStream {
    fn from_iter<I: IntoIterator<Item = TokenStream>>(streams: I) -> Self {
        Self::_new(streams.into_iter().flat_map(|i| i.inner).collect())
    }
}

impl Display for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <proc_macro2::TokenStream>::from(self).fmt(f)
    }
}

impl From<TokenTree> for TokenStream {
    fn from(tree: TokenTree) -> Self {
        Self::_new(vec![tree])
    }
}

/// The iteration is "shallow", e.g. the iterator doesn't recurse into
/// delimited groups, and returns whole groups as token trees.
impl IntoIterator for TokenStream {
    type Item = TokenTree;
    type IntoIter = IntoIter<TokenTree>;

    fn into_iter(self) -> IntoIter<TokenTree> {
        self.inner.into_iter()
    }
}

/// A single token or a delimited sequence of token trees (e.g. `[1, (), ..]`).
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
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

impl From<Group> for TokenTree {
    fn from(g: Group) -> Self {
        TokenTree::Group(g)
    }
}

impl From<Ident> for TokenTree {
    fn from(g: Ident) -> Self {
        TokenTree::Ident(g)
    }
}

impl From<Punct> for TokenTree {
    fn from(g: Punct) -> Self {
        TokenTree::Punct(g)
    }
}

impl From<Literal> for TokenTree {
    fn from(g: Literal) -> Self {
        TokenTree::Literal(g)
    }
}

impl Display for TokenTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenTree::Group(t) => t.fmt(f),
            TokenTree::Ident(t) => t.fmt(f),
            TokenTree::Punct(t) => t.fmt(f),
            TokenTree::Literal(t) => t.fmt(f),
        }
    }
}

/// A delimited token stream.
///
/// A `Group` internally contains a `TokenStream` which is surrounded by
/// `Delimiter`s.
#[derive(Serialize, Deserialize, Clone)]
pub struct Group {
    delimiter: Delimiter,
    stream: TokenStream,
}

/// Describes how a sequence of token trees is delimited.
#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
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

impl Group {
    /// Creates a new `Group` with the given delimiter and token stream.
    pub fn new(delimiter: Delimiter, stream: TokenStream) -> Self {
        Self { delimiter, stream }
    }

    /// Returns the delimiter of this `Group`
    pub fn delimiter(&self) -> Delimiter {
        self.delimiter
    }

    /// Returns the `TokenStream` of tokens that are delimited in this `Group`.
    ///
    /// Note that the returned token stream does not include the delimiter
    /// returned above.
    pub fn stream(&self) -> TokenStream {
        self.stream.clone()
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <proc_macro2::Group>::from(self).fmt(f)
    }
}

/// An `Punct` is an single punctuation character like `+`, `-` or `#`.
///
/// Multicharacter operators like `+=` are represented as two instances of
/// `Punct` with different forms of `Spacing` returned.
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Punct {
    op: char,
    spacing: Spacing,
}

/// Whether an `Punct` is followed immediately by another `Punct` or followed by
/// another token or whitespace.
#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Spacing {
    /// E.g. `+` is `Alone` in `+ =`, `+ident` or `+()`.
    Alone,
    /// E.g. `+` is `Joint` in `+=` or `'#`.
    ///
    /// Additionally, single quote `'` can join with identifiers to form
    /// lifetimes `'ident`.
    Joint,
}

impl Punct {
    /// Creates a new `Punct` from the given character and spacing.
    ///
    /// The `ch` argument must be a valid punctuation character permitted by the
    /// language, otherwise the function will panic.
    pub fn new(op: char, spacing: Spacing) -> Self {
        Self { op, spacing }
    }

    /// Returns the value of this punctuation character as `char`.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn as_char(&self) -> char {
        self.op
    }

    /// Returns the spacing of this punctuation character, indicating whether
    /// it's immediately followed by another `Punct` in the token stream, so
    /// they can potentially be combined into a multicharacter operator
    /// (`Joint`), or it's followed by some other token or whitespace (`Alone`)
    /// so the operator has certainly ended.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn spacing(&self) -> Spacing {
        self.spacing
    }
}

impl Display for Punct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.op.fmt(f)
    }
}

/// A word of Rust code, which may be a keyword or legal variable name.
///
/// An identifier consists of at least one Unicode code point, the first of
/// which has the XID_Start property and the rest of which have the XID_Continue
/// property.
///
/// - The empty string is not an identifier. Use `Option<Ident>`.
/// - A lifetime is not an identifier. Use `syn::Lifetime` instead.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(transparent)]
pub struct Ident {
    inner: String,
}

impl Ident {
    /// Creates a new `Ident` with the given `string`.
    ///
    /// The `string` argument must be a valid identifier permitted by the
    /// language, otherwise the function will panic.
    ///
    /// # Panics
    ///
    /// Panics if the input string is neither a keyword nor a legal variable
    /// name.
    pub fn new(string: &str) -> Self {
        Self::from(&proc_macro2::Ident::new(string, Span::call_site()))
    }
}

impl<T> PartialEq<T> for Ident
where
    T: ?Sized + AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        self.inner == other.as_ref()
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

/// A literal string (`"hello"`), byte string (`b"hello"`), character (`'a'`),
/// byte character (`b'a'`), an integer or floating point number with or without
/// a suffix (`1`, `1u8`, `2.3`, `2.3f32`).
///
/// Boolean literals like `true` and `false` do not belong here, they are
/// `Ident`s.
#[derive(Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct Literal {
    pub(crate) text: String,
}

macro_rules! suffixed_numbers {
    ($($name:ident => $kind:ident,)*) => ($(
        pub fn $name(n: $kind) -> Self {
            Self::_new(format!(concat!("{}", stringify!($kind)), n))
        }
    )*)
}

macro_rules! unsuffixed_numbers {
    ($($name:ident => $kind:ident,)*) => ($(
        pub fn $name(n: $kind) -> Self {
            Self::_new(n.to_string())
        }
    )*)
}

impl Literal {
    fn _new(text: String) -> Self {
        Self { text }
    }

    suffixed_numbers! {
        u8_suffixed => u8,
        u16_suffixed => u16,
        u32_suffixed => u32,
        u64_suffixed => u64,
        u128_suffixed => u128,
        usize_suffixed => usize,
        i8_suffixed => i8,
        i16_suffixed => i16,
        i32_suffixed => i32,
        i64_suffixed => i64,
        i128_suffixed => i128,
        isize_suffixed => isize,

        f32_suffixed => f32,
        f64_suffixed => f64,
    }

    unsuffixed_numbers! {
        u8_unsuffixed => u8,
        u16_unsuffixed => u16,
        u32_unsuffixed => u32,
        u64_unsuffixed => u64,
        u128_unsuffixed => u128,
        usize_unsuffixed => usize,
        i8_unsuffixed => i8,
        i16_unsuffixed => i16,
        i32_unsuffixed => i32,
        i64_unsuffixed => i64,
        i128_unsuffixed => i128,
        isize_unsuffixed => isize,
    }

    pub fn f32_unsuffixed(f: f32) -> Self {
        let mut s = f.to_string();
        if !s.contains('.') {
            s.push_str(".0");
        }
        Self::_new(s)
    }

    pub fn f64_unsuffixed(f: f64) -> Self {
        let mut s = f.to_string();
        if !s.contains('.') {
            s.push_str(".0");
        }
        Self::_new(s)
    }

    pub fn string(t: &str) -> Self {
        let mut s = t.chars().flat_map(char::escape_default).collect::<String>();
        s.push('"');
        s.insert(0, '"');
        Self::_new(s)
    }

    pub fn character(t: char) -> Self {
        Self::_new(format!("'{}'", t.escape_default().collect::<String>()))
    }

    #[allow(clippy::match_overlapping_arm)]
    pub fn byte_string(bytes: &[u8]) -> Self {
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

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.text.fmt(f)
    }
}

mod convert {
    use super::*;

    // TokenStream

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

    /*
    impl From<proc_macro2::TokenStream> for TokenStream {
        fn from(other: proc_macro2::TokenStream) -> Self {
            Self::_new(other.into_iter().map::<TokenTree, _>(|x| x.ref_into()).collect())
        }
    }
    */

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
