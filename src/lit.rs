use super::*;

ast_enum! {
    /// A Rust literal such as a string or integer or boolean.
    pub enum Lit {
        /// A UTF-8 string literal: `"foo"`.
        Str(LitStr),

        /// A byte string literal: `b"foo"`.
        ByteStr(LitByteStr),

        /// A byte literal: `b'f'`.
        Byte(LitByte),

        /// A character literal: `'a'`.
        Char(LitChar),

        /// An integer literal: `1` or `1u16`.
        Int(LitInt),

        /// A floating point literal: `1f64` or `1.0e10f64`.
        ///
        /// Must be finite. May not be infinte or NaN.
        Float(LitFloat),

        /// A boolean literal: `true` or `false`.
        Bool(LitBool),

        /// A raw token literal not interpreted by Syn.
        Verbatim(Literal),
    }
}

ast_struct! {
    /// A UTF-8 string literal: `"foo"`.
    #[serde(transparent)]
    pub struct LitStr {
        token: Literal,
    }
}

ast_struct! {
    /// A byte string literal: `b"foo"`.
    #[serde(transparent)]
    pub struct LitByteStr {
        token: Literal,
    }
}

ast_struct! {
    /// A byte literal: `b'f'`.
    #[serde(transparent)]
    pub struct LitByte {
        token: Literal,
    }
}

ast_struct! {
    /// A character literal: `'a'`.
    #[serde(transparent)]
    pub struct LitChar {
        token: Literal,
    }
}

ast_struct! {
    /// An integer literal: `1` or `1u16`.
    #[serde(transparent)]
    pub struct LitInt {
        token: Literal,
    }
}

ast_struct! {
    /// A floating point literal: `1f64` or `1.0e10f64`.
    ///
    /// Must be finite. May not be infinte or NaN.
    #[serde(transparent)]
    pub struct LitFloat {
        token: Literal,
    }
}

ast_struct! {
    /// A boolean literal: `true` or `false`.
    #[serde(transparent)]
    pub struct LitBool {
        pub(crate) value: bool,
    }
}

ast_enum! {
    /// The style of a string literal, either plain quoted or a raw string like
    /// `r##"data"##`.
    pub enum StrStyle {
        /// An ordinary string like `"data"`.
        Cooked,
        /// A raw string like `r##"data"##`.
        ///
        /// The unsigned integer is the number of `#` symbols used.
        Raw(usize),
    }
}

mod value {
    use super::*;
    use proc_macro2::{TokenStream, TokenTree};
    use std::{
        char,
        ops::{Index, RangeFrom},
    };

    /// Get the byte at offset idx, or a default of `b'\0'` if we're looking
    /// past the end of the input buffer.
    pub(crate) fn byte<S: AsRef<[u8]> + ?Sized>(s: &S, idx: usize) -> u8 {
        let s = s.as_ref();
        if idx < s.len() { s[idx] } else { 0 }
    }

    fn next_chr(s: &str) -> char {
        s.chars().next().unwrap_or('\0')
    }

    // Returns (content, suffix).
    pub(crate) fn parse_lit_str(s: &str) -> (Box<str>, Box<str>) {
        match byte(s, 0) {
            b'"' => parse_lit_str_cooked(s),
            b'r' => parse_lit_str_raw(s),
            _ => unreachable!(),
        }
    }

    fn parse_lit_str_cooked(mut s: &str) -> (Box<str>, Box<str>) {
        assert_eq!(byte(s, 0), b'"');
        s = &s[1..];

        let mut content = String::new();
        'outer: loop {
            let ch = match byte(s, 0) {
                b'"' => break,
                b'\\' => {
                    let b = byte(s, 1);
                    s = &s[2..];
                    match b {
                        b'x' => {
                            let (byte, rest) = backslash_x(s);
                            s = rest;
                            assert!(byte <= 0x80, "Invalid \\x byte in string literal");
                            char::from_u32(u32::from(byte)).unwrap()
                        }
                        b'u' => {
                            let (chr, rest) = backslash_u(s);
                            s = rest;
                            chr
                        }
                        b'n' => '\n',
                        b'r' => '\r',
                        b't' => '\t',
                        b'\\' => '\\',
                        b'0' => '\0',
                        b'\'' => '\'',
                        b'"' => '"',
                        b'\r' | b'\n' => loop {
                            let ch = next_chr(s);
                            if ch.is_whitespace() {
                                s = &s[ch.len_utf8()..];
                            } else {
                                continue 'outer;
                            }
                        },
                        b => panic!("unexpected byte {:?} after \\ character in byte literal", b),
                    }
                }
                b'\r' => {
                    assert_eq!(byte(s, 1), b'\n', "Bare CR not allowed in string");
                    s = &s[2..];
                    '\n'
                }
                _ => {
                    let ch = next_chr(s);
                    s = &s[ch.len_utf8()..];
                    ch
                }
            };
            content.push(ch);
        }

        assert!(s.starts_with('"'));
        let content = content.into_boxed_str();
        let suffix = s[1..].to_owned().into_boxed_str();
        (content, suffix)
    }

    fn parse_lit_str_raw(mut s: &str) -> (Box<str>, Box<str>) {
        assert_eq!(byte(s, 0), b'r');
        s = &s[1..];

        let mut pounds = 0;
        while byte(s, pounds) == b'#' {
            pounds += 1;
        }
        assert_eq!(byte(s, pounds), b'"');
        assert_eq!(byte(s, s.len() - pounds - 1), b'"');
        for end in s[s.len() - pounds..].bytes() {
            assert_eq!(end, b'#');
        }

        let content = s[pounds + 1..s.len() - pounds - 1].to_owned().into_boxed_str();
        let suffix = Box::<str>::default(); // todo
        (content, suffix)
    }

    pub(crate) fn parse_lit_byte_str(s: &str) -> Vec<u8> {
        assert_eq!(byte(s, 0), b'b');
        match byte(s, 1) {
            b'"' => parse_lit_byte_str_cooked(s),
            b'r' => parse_lit_byte_str_raw(s),
            _ => unreachable!(),
        }
    }

    fn parse_lit_byte_str_cooked(mut s: &str) -> Vec<u8> {
        assert_eq!(byte(s, 0), b'b');
        assert_eq!(byte(s, 1), b'"');
        s = &s[2..];

        // We're going to want to have slices which don't respect codepoint boundaries.
        let mut s = s.as_bytes();

        let mut out = Vec::new();
        'outer: loop {
            let byte = match byte(s, 0) {
                b'"' => break,
                b'\\' => {
                    let b = byte(s, 1);
                    s = &s[2..];
                    match b {
                        b'x' => {
                            let (b, rest) = backslash_x(s);
                            s = rest;
                            b
                        }
                        b'n' => b'\n',
                        b'r' => b'\r',
                        b't' => b'\t',
                        b'\\' => b'\\',
                        b'0' => b'\0',
                        b'\'' => b'\'',
                        b'"' => b'"',
                        b'\r' | b'\n' => loop {
                            let byte = byte(s, 0);
                            let ch = char::from_u32(u32::from(byte)).unwrap();
                            if ch.is_whitespace() {
                                s = &s[1..];
                            } else {
                                continue 'outer;
                            }
                        },
                        b => panic!("unexpected byte {:?} after \\ character in byte literal", b),
                    }
                }
                b'\r' => {
                    assert_eq!(byte(s, 1), b'\n', "Bare CR not allowed in string");
                    s = &s[2..];
                    b'\n'
                }
                b => {
                    s = &s[1..];
                    b
                }
            };
            out.push(byte);
        }

        assert_eq!(s, b"\"");
        out
    }

    fn parse_lit_byte_str_raw(s: &str) -> Vec<u8> {
        assert_eq!(byte(s, 0), b'b');
        String::from(parse_lit_str_raw(&s[1..]).0).into_bytes()
    }

    pub(crate) fn parse_lit_byte(s: &str) -> u8 {
        assert_eq!(byte(s, 0), b'b');
        assert_eq!(byte(s, 1), b'\'');

        // We're going to want to have slices which don't respect codepoint boundaries.
        let mut s = s[2..].as_bytes();

        let b = match byte(s, 0) {
            b'\\' => {
                let b = byte(s, 1);
                s = &s[2..];
                match b {
                    b'x' => {
                        let (b, rest) = backslash_x(s);
                        s = rest;
                        b
                    }
                    b'n' => b'\n',
                    b'r' => b'\r',
                    b't' => b'\t',
                    b'\\' => b'\\',
                    b'0' => b'\0',
                    b'\'' => b'\'',
                    b'"' => b'"',
                    b => panic!("unexpected byte {:?} after \\ character in byte literal", b),
                }
            }
            b => {
                s = &s[1..];
                b
            }
        };

        assert_eq!(byte(s, 0), b'\'');
        b
    }

    pub(crate) fn parse_lit_char(mut s: &str) -> char {
        assert_eq!(byte(s, 0), b'\'');
        s = &s[1..];

        let ch = if byte(s, 0) == b'\\' {
            let b = byte(s, 1);
            s = &s[2..];
            match b {
                b'x' => {
                    let (byte, rest) = backslash_x(s);
                    s = rest;
                    assert!(byte <= 0x80, "Invalid \\x byte in string literal");
                    char::from_u32(u32::from(byte)).unwrap()
                }
                b'u' => {
                    let (chr, rest) = backslash_u(s);
                    s = rest;
                    chr
                }
                b'n' => '\n',
                b'r' => '\r',
                b't' => '\t',
                b'\\' => '\\',
                b'0' => '\0',
                b'\'' => '\'',
                b'"' => '"',
                b => panic!("unexpected byte {:?} after \\ character in byte literal", b),
            }
        } else {
            let ch = next_chr(s);
            s = &s[ch.len_utf8()..];
            ch
        };
        assert_eq!(s, "\'", "Expected end of char literal");
        ch
    }

    fn backslash_x<S>(s: &S) -> (u8, &S)
    where
        S: Index<RangeFrom<usize>, Output = S> + AsRef<[u8]> + ?Sized,
    {
        let mut ch = 0;
        let b0 = byte(s, 0);
        let b1 = byte(s, 1);
        ch += 0x10
            * match b0 {
                b'0'..=b'9' => b0 - b'0',
                b'a'..=b'f' => 10 + (b0 - b'a'),
                b'A'..=b'F' => 10 + (b0 - b'A'),
                _ => panic!("unexpected non-hex character after \\x"),
            };
        ch += match b1 {
            b'0'..=b'9' => b1 - b'0',
            b'a'..=b'f' => 10 + (b1 - b'a'),
            b'A'..=b'F' => 10 + (b1 - b'A'),
            _ => panic!("unexpected non-hex character after \\x"),
        };
        (ch, &s[2..])
    }

    fn backslash_u(mut s: &str) -> (char, &str) {
        if byte(s, 0) != b'{' {
            panic!("expected {{ after \\u");
        }
        s = &s[1..];

        let mut ch = 0;
        for _ in 0..6 {
            let b = byte(s, 0);
            match b {
                b'0'..=b'9' => {
                    ch *= 0x10;
                    ch += u32::from(b - b'0');
                    s = &s[1..];
                }
                b'a'..=b'f' => {
                    ch *= 0x10;
                    ch += u32::from(10 + b - b'a');
                    s = &s[1..];
                }
                b'A'..=b'F' => {
                    ch *= 0x10;
                    ch += u32::from(10 + b - b'A');
                    s = &s[1..];
                }
                b'}' => break,
                _ => panic!("unexpected non-hex character after \\u"),
            }
        }
        assert!(byte(s, 0) == b'}');
        s = &s[1..];

        if let Some(ch) = char::from_u32(ch) {
            (ch, s)
        } else {
            panic!("character code {:x} is not a valid unicode character", ch);
        }
    }

    pub(crate) fn to_literal(s: &str) -> Literal {
        let stream = s.parse::<TokenStream>().unwrap();
        match stream.into_iter().next().unwrap() {
            TokenTree::Literal(l) => l.ref_into(),
            _ => unreachable!(),
        }
    }
}

mod convert {
    use super::*;

    // LitStr
    impl From<&syn::LitStr> for LitStr {
        fn from(other: &syn::LitStr) -> Self {
            Self { token: Literal::string(&other.value()) }
        }
    }
    impl From<&LitStr> for syn::LitStr {
        fn from(other: &LitStr) -> Self {
            let (value, _) = value::parse_lit_str(&other.token.to_string());
            Self::new(&value, Span::call_site())
        }
    }

    // LitByteStr
    impl From<&syn::LitByteStr> for LitByteStr {
        fn from(other: &syn::LitByteStr) -> Self {
            Self { token: Literal::byte_string(&other.value()) }
        }
    }
    impl From<&LitByteStr> for syn::LitByteStr {
        fn from(other: &LitByteStr) -> Self {
            let value = value::parse_lit_byte_str(&other.token.to_string());
            Self::new(&value, Span::call_site())
        }
    }

    // LitByte
    impl From<&syn::LitByte> for LitByte {
        fn from(other: &syn::LitByte) -> Self {
            Self { token: Literal::u8_suffixed(other.value()) }
        }
    }
    impl From<&LitByte> for syn::LitByte {
        fn from(other: &LitByte) -> Self {
            let value = value::parse_lit_byte(&other.token.to_string());
            Self::new(value, Span::call_site())
        }
    }

    // LitChar
    impl From<&syn::LitChar> for LitChar {
        fn from(other: &syn::LitChar) -> Self {
            Self { token: Literal::character(other.value()) }
        }
    }
    impl From<&LitChar> for syn::LitChar {
        fn from(other: &LitChar) -> Self {
            let value = value::parse_lit_char(&other.token.to_string());
            Self::new(value, Span::call_site())
        }
    }

    // LitInt
    impl From<&syn::LitInt> for LitInt {
        fn from(other: &syn::LitInt) -> Self {
            Self { token: value::to_literal(&other.to_string()) }
        }
    }
    impl From<&LitInt> for syn::LitInt {
        fn from(other: &LitInt) -> Self {
            Self::new(&other.token.to_string(), Span::call_site())
        }
    }

    // LitFloat
    impl From<&syn::LitFloat> for LitFloat {
        fn from(other: &syn::LitFloat) -> Self {
            Self { token: value::to_literal(&other.to_string()) }
        }
    }
    impl From<&LitFloat> for syn::LitFloat {
        fn from(other: &LitFloat) -> Self {
            Self::new(&other.token.to_string(), Span::call_site())
        }
    }
}
