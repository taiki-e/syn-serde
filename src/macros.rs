// SPDX-License-Identifier: Apache-2.0 OR MIT

macro_rules! ast_struct {
    (
        $(#[$attrs:meta])*
        pub struct $name:ident $($rest:tt)*
    ) => {
        #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
        $(#[$attrs])*
        pub struct $name $($rest)*
    };
}

macro_rules! ast_enum {
    (
        $(#[$attrs:meta])*
        pub enum $name:ident $($rest:tt)*
    ) => (
        #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
        #[serde(rename_all = "snake_case")]
        $(#[$attrs])*
        pub enum $name $($rest)*
    );
}

macro_rules! syn_trait_impl {
    ($path:ident :: $ty:ident) => {
        impl crate::sealed::Sealed for $path::$ty {}
        impl crate::Syn for $path::$ty {
            type Adapter = $ty;
            fn to_adapter(&self) -> Self::Adapter {
                Self::Adapter::from(self)
            }
            fn from_adapter(adapter: &Self::Adapter) -> Self {
                Self::from(adapter)
            }
        }
    };
}
