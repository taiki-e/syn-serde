use super::*;

ast_struct! {
    /// A complete file of Rust source code.
    pub struct File {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) shebang: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub(crate) attrs: Vec<Attribute>,
        pub(crate) items: Vec<Item>,
    }
}
