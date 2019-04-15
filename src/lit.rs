use super::*;
use proc_macro2::Span;
use std::str;
use syn::{FloatSuffix, IntSuffix};

ast_enum_of_structs! {
    /// A Rust literal such as a string or integer or boolean.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum Lit {
        /// A UTF-8 string literal: `"foo"`.
        pub Str(LitStr #transparent {
            token: Literal,
        }),

        /// A byte string literal: `b"foo"`.
        pub ByteStr(LitByteStr #transparent {
            token: Literal,
        }),

        /// A byte literal: `b'f'`.
        pub Byte(LitByte #transparent {
            token: Literal,
        }),

        /// A character literal: `'a'`.
        pub Char(LitChar #transparent {
            token: Literal,
        }),

        /// An integer literal: `1` or `1u16`.
        ///
        /// Holds up to 64 bits of data. Use `LitVerbatim` for any larger
        /// integer literal.
        pub Int(LitInt #transparent {
            token: Literal,
        }),

        /// A floating point literal: `1f64` or `1.0e10f64`.
        ///
        /// Must be finite. May not be infinte or NaN.
        pub Float(LitFloat #transparent {
            token: Literal,
        }),

        /// A boolean literal: `true` or `false`.
        pub Bool(LitBool #transparent {
            value: bool,
        }),

        /// A raw token literal not interpreted by Syn, possibly because it
        /// represents an integer larger than 64 bits.
        pub Verbatim(LitVerbatim #transparent {
            token: Literal,
        }),
    }
}

impl LitStr {
    pub fn new(value: &str) -> Self {
        Self {
            token: Literal::string(value),
        }
    }

    pub fn value(&self) -> String {
        value::parse_lit_str(&self.token.text)
    }
}

impl LitByteStr {
    pub fn new(value: &[u8]) -> Self {
        Self {
            token: Literal::byte_string(value),
        }
    }

    pub fn value(&self) -> Vec<u8> {
        value::parse_lit_byte_str(&self.token.text)
    }
}

impl LitByte {
    pub fn new(value: u8) -> Self {
        Self {
            token: Literal::u8_suffixed(value),
        }
    }

    pub fn value(&self) -> u8 {
        value::parse_lit_byte(&self.token.text)
    }
}

impl LitChar {
    pub fn new(value: char) -> Self {
        Self {
            token: Literal::character(value),
        }
    }

    pub fn value(&self) -> char {
        value::parse_lit_char(&self.token.text)
    }
}

impl LitInt {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(value: u64, suffix: IntSuffix) -> Self {
        let token = match suffix {
            IntSuffix::Isize => Literal::isize_suffixed(value as isize),
            IntSuffix::I8 => Literal::i8_suffixed(value as i8),
            IntSuffix::I16 => Literal::i16_suffixed(value as i16),
            IntSuffix::I32 => Literal::i32_suffixed(value as i32),
            IntSuffix::I64 => Literal::i64_suffixed(value as i64),
            IntSuffix::I128 => Literal::i128_suffixed(i128::from(value)),
            IntSuffix::Usize => Literal::usize_suffixed(value as usize),
            IntSuffix::U8 => Literal::u8_suffixed(value as u8),
            IntSuffix::U16 => Literal::u16_suffixed(value as u16),
            IntSuffix::U32 => Literal::u32_suffixed(value as u32),
            IntSuffix::U64 => Literal::u64_suffixed(value),
            IntSuffix::U128 => Literal::u128_suffixed(u128::from(value)),
            IntSuffix::None => Literal::u64_unsuffixed(value),
        };
        Self { token }
    }

    pub fn value(&self) -> u64 {
        value::parse_lit_int(&self.token.text).unwrap()
    }

    pub fn suffix(&self) -> IntSuffix {
        let value = &self.token.text;
        for (s, suffix) in vec![
            ("i8", IntSuffix::I8),
            ("i16", IntSuffix::I16),
            ("i32", IntSuffix::I32),
            ("i64", IntSuffix::I64),
            ("i128", IntSuffix::I128),
            ("isize", IntSuffix::Isize),
            ("u8", IntSuffix::U8),
            ("u16", IntSuffix::U16),
            ("u32", IntSuffix::U32),
            ("u64", IntSuffix::U64),
            ("u128", IntSuffix::U128),
            ("usize", IntSuffix::Usize),
        ] {
            if value.ends_with(s) {
                return suffix;
            }
        }
        IntSuffix::None
    }
}

impl LitFloat {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(value: f64, suffix: FloatSuffix) -> Self {
        let token = match suffix {
            FloatSuffix::F32 => Literal::f32_suffixed(value as f32),
            FloatSuffix::F64 => Literal::f64_suffixed(value),
            FloatSuffix::None => Literal::f64_unsuffixed(value),
        };
        Self { token }
    }

    pub fn value(&self) -> f64 {
        value::parse_lit_float(&self.token.text)
    }

    pub fn suffix(&self) -> FloatSuffix {
        let value = &self.token.text;
        for (s, suffix) in vec![("f32", FloatSuffix::F32), ("f64", FloatSuffix::F64)] {
            if value.ends_with(s) {
                return suffix;
            }
        }
        FloatSuffix::None
    }
}

mod convert {
    use super::*;

    // LitStr

    impl From<&syn::LitStr> for LitStr {
        fn from(other: &syn::LitStr) -> Self {
            Self::new(&other.value())
        }
    }

    impl From<&LitStr> for syn::LitStr {
        fn from(other: &LitStr) -> Self {
            Self::new(&other.value(), Span::call_site())
        }
    }

    // LitByteStr

    impl From<&syn::LitByteStr> for LitByteStr {
        fn from(other: &syn::LitByteStr) -> Self {
            Self::new(&other.value())
        }
    }

    impl From<&LitByteStr> for syn::LitByteStr {
        fn from(other: &LitByteStr) -> Self {
            Self::new(&other.value(), Span::call_site())
        }
    }

    // LitByte

    impl From<&syn::LitByte> for LitByte {
        fn from(other: &syn::LitByte) -> Self {
            Self::new(other.value())
        }
    }

    impl From<&LitByte> for syn::LitByte {
        fn from(other: &LitByte) -> Self {
            Self::new(other.value(), Span::call_site())
        }
    }

    // LitChar

    impl From<&syn::LitChar> for LitChar {
        fn from(other: &syn::LitChar) -> Self {
            Self::new(other.value())
        }
    }

    impl From<&LitChar> for syn::LitChar {
        fn from(other: &LitChar) -> Self {
            Self::new(other.value(), Span::call_site())
        }
    }

    // LitInt

    impl From<&syn::LitInt> for LitInt {
        fn from(other: &syn::LitInt) -> Self {
            Self::new(other.value(), other.suffix())
        }
    }

    impl From<&LitInt> for syn::LitInt {
        fn from(other: &LitInt) -> Self {
            Self::new(other.value(), other.suffix(), Span::call_site())
        }
    }

    // LitFloat

    impl From<&syn::LitFloat> for LitFloat {
        fn from(other: &syn::LitFloat) -> Self {
            Self::new(other.value(), other.suffix())
        }
    }

    impl From<&LitFloat> for syn::LitFloat {
        fn from(other: &LitFloat) -> Self {
            Self::new(other.value(), other.suffix(), Span::call_site())
        }
    }

    // LitBool

    impl From<&syn::LitBool> for LitBool {
        fn from(other: &syn::LitBool) -> Self {
            Self { value: other.value }
        }
    }

    impl From<&LitBool> for syn::LitBool {
        fn from(other: &LitBool) -> Self {
            Self {
                value: other.value,
                span: Span::call_site(),
            }
        }
    }

    // LitVerbatim

    impl From<&syn::LitVerbatim> for LitVerbatim {
        fn from(other: &syn::LitVerbatim) -> Self {
            Self {
                token: other.token.ref_into(),
            }
        }
    }

    impl From<&LitVerbatim> for syn::LitVerbatim {
        fn from(other: &LitVerbatim) -> Self {
            Self {
                token: other.token.ref_into(),
            }
        }
    }
}

mod value {
    use super::*;
    use std::char;
    use std::ops::{Index, RangeFrom};

    impl Lit {
        /// Interpret a Syn literal from a proc-macro2 literal.
        ///
        /// Not all proc-macro2 literals are valid Syn literals. In particular,
        /// doc comments are considered by proc-macro2 to be literals but in Syn
        /// they are [`Attribute`].
        ///
        /// [`Attribute`]: struct.Attribute.html
        ///
        /// # Panics
        ///
        /// Panics if the input is a doc comment literal.
        pub fn new(token: Literal) -> Self {
            let value = &token.text;

            match value::byte(value, 0) {
                b'"' | b'r' => return Lit::Str(LitStr { token }),
                b'b' => match value::byte(value, 1) {
                    b'"' | b'r' => return Lit::ByteStr(LitByteStr { token }),
                    b'\'' => return Lit::Byte(LitByte { token }),
                    _ => {}
                },
                b'\'' => return Lit::Char(LitChar { token }),
                b'0'..=b'9' => {
                    if number_is_int(value) {
                        return Lit::Int(LitInt { token });
                    } else if number_is_float(value) {
                        return Lit::Float(LitFloat { token });
                    } else {
                        // number overflow
                        return Lit::Verbatim(LitVerbatim { token });
                    }
                }
                _ => {
                    if value == "true" || value == "false" {
                        return Lit::Bool(LitBool {
                            value: value == "true",
                        });
                    }
                }
            }

            panic!("Unrecognized literal: {}", value);
        }
    }

    fn number_is_int(value: &str) -> bool {
        if number_is_float(value) {
            false
        } else {
            value::parse_lit_int(value).is_some()
        }
    }

    fn number_is_float(value: &str) -> bool {
        if value.contains('.') {
            true
        } else if value.starts_with("0x") || value.ends_with("size") {
            false
        } else {
            value.contains('e') || value.contains('E')
        }
    }

    /// Get the byte at offset idx, or a default of `b'\0'` if we're looking
    /// past the end of the input buffer.
    pub(super) fn byte<S: AsRef<[u8]> + ?Sized>(s: &S, idx: usize) -> u8 {
        let s = s.as_ref();
        if idx < s.len() {
            s[idx]
        } else {
            0
        }
    }

    fn next_chr(s: &str) -> char {
        s.chars().next().unwrap_or('\0')
    }

    pub(super) fn parse_lit_str(s: &str) -> String {
        match byte(s, 0) {
            b'"' => parse_lit_str_cooked(s),
            b'r' => parse_lit_str_raw(s),
            _ => unreachable!(),
        }
    }

    fn parse_lit_str_cooked(mut s: &str) -> String {
        assert_eq!(byte(s, 0), b'"');
        s = &s[1..];

        let mut out = String::new();
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
            out.push(ch);
        }

        assert_eq!(s, "\"");
        out
    }

    fn parse_lit_str_raw(mut s: &str) -> String {
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

        s[pounds + 1..s.len() - pounds - 1].to_owned()
    }

    pub(super) fn parse_lit_byte_str(s: &str) -> Vec<u8> {
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
        parse_lit_str_raw(&s[1..]).into_bytes()
    }

    pub(super) fn parse_lit_byte(s: &str) -> u8 {
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

    pub(super) fn parse_lit_char(mut s: &str) -> char {
        assert_eq!(byte(s, 0), b'\'');
        s = &s[1..];

        let ch = match byte(s, 0) {
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
                    b => panic!("unexpected byte {:?} after \\ character in byte literal", b),
                }
            }
            _ => {
                let ch = next_chr(s);
                s = &s[ch.len_utf8()..];
                ch
            }
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

    #[allow(clippy::unseparated_literal_suffix)]
    pub(super) fn parse_lit_int(mut s: &str) -> Option<u64> {
        let base = match (byte(s, 0), byte(s, 1)) {
            (b'0', b'x') => {
                s = &s[2..];
                16
            }
            (b'0', b'o') => {
                s = &s[2..];
                8
            }
            (b'0', b'b') => {
                s = &s[2..];
                2
            }
            (b'0'..=b'9', _) => 10,
            _ => unreachable!(),
        };

        let mut value = 0u64;
        loop {
            let b = byte(s, 0);
            let digit = match b {
                b'0'..=b'9' => u64::from(b - b'0'),
                b'a'..=b'f' if base > 10 => 10 + u64::from(b - b'a'),
                b'A'..=b'F' if base > 10 => 10 + u64::from(b - b'A'),
                b'_' => {
                    s = &s[1..];
                    continue;
                }
                // NOTE: Looking at a floating point literal, we don't want to
                // consider these integers.
                b'.' if base == 10 => return None,
                b'e' | b'E' if base == 10 => return None,
                _ => break,
            };

            if digit >= base {
                panic!("Unexpected digit {:x} out of base range", digit);
            }

            value = match value.checked_mul(base) {
                Some(value) => value,
                None => return None,
            };
            value = match value.checked_add(digit) {
                Some(value) => value,
                None => return None,
            };
            s = &s[1..];
        }

        Some(value)
    }

    pub(super) fn parse_lit_float(input: &str) -> f64 {
        // Rust's floating point literals are very similar to the ones parsed by
        // the standard library, except that rust's literals can contain
        // ignorable underscores. Let's remove those underscores.
        let mut bytes = input.to_owned().into_bytes();
        let mut write = 0;
        for read in 0..bytes.len() {
            if bytes[read] == b'_' {
                continue; // Don't increase write
            }
            if write != read {
                let x = bytes[read];
                bytes[write] = x;
            }
            write += 1;
        }
        bytes.truncate(write);
        let input = String::from_utf8(bytes).unwrap();
        let end = input.find('f').unwrap_or_else(|| input.len());
        input[..end].parse().unwrap()
    }
}
