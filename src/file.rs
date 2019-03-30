use super::*;

ast_struct! {
    /// A complete file of Rust source code.
    pub struct File {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        shebang: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        attrs: Vec<Attribute>,
        items: Vec<Item>,
    }
}

mod convert {
    use super::*;

    // File

    impl From<&syn::File> for File {
        fn from(other: &syn::File) -> Self {
            Self {
                shebang: other.shebang.as_ref().cloned(),
                attrs: other.attrs.map_into(),
                items: other.items.map_into(),
            }
        }
    }

    impl From<&File> for syn::File {
        fn from(other: &File) -> Self {
            Self {
                shebang: other.shebang.as_ref().cloned(),
                attrs: other.attrs.map_into(),
                items: other.items.map_into(),
            }
        }
    }
}
