use super::*;
use proc_macro2::Span;
use std::fmt::{self, Display};

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
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(transparent)]
pub struct Lifetime {
    pub ident: Ident,
}

impl Lifetime {
    /// # Panics
    ///
    /// Panics if the lifetime does not conform to the bulleted rules above.
    ///
    pub fn new(symbol: &str) -> Self {
        Self::from(&syn::Lifetime::new(symbol, Span::call_site()))
    }
}

impl Display for Lifetime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        "'".fmt(formatter)?;
        self.ident.fmt(formatter)
    }
}

mod convert {
    use super::*;

    // Lifetime

    impl From<&syn::Lifetime> for Lifetime {
        fn from(other: &syn::Lifetime) -> Self {
            Self {
                ident: other.ident.ref_into(),
            }
        }
    }

    impl From<&Lifetime> for syn::Lifetime {
        fn from(other: &Lifetime) -> Self {
            Self::new(&other.to_string(), Span::call_site())
        }
    }
}
