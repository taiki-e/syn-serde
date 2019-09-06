macro_rules! ast_struct {
    (
        [$($attrs_pub:tt)*]
        struct $name:ident $($rest:tt)*
    ) => {
        #[derive(crate::Serialize, crate::Deserialize)]
        $($attrs_pub)* struct $name $($rest)*
    };

    ($($t:tt)*) => {
        strip_attrs_pub!(ast_struct!($($t)*));
    };
}

macro_rules! ast_enum {
    (
        [$($attrs_pub:tt)*]
        enum $name:ident $($rest:tt)*
    ) => (
        #[derive(crate::Serialize, crate::Deserialize)]
        #[serde(rename_all = "snake_case")]
        $($attrs_pub)* enum $name $($rest)*
    );

    ($($t:tt)*) => {
        strip_attrs_pub!(ast_enum!($($t)*));
    };
}

macro_rules! strip_attrs_pub {
    ($mac:ident!($(#[$m:meta])* $pub:ident $($t:tt)*)) => {
        check_keyword_matches!(pub $pub);

        $mac!([$(#[$m])* $pub] $($t)*);
    };
}

macro_rules! check_keyword_matches {
    (struct struct) => {};
    (enum enum) => {};
    (pub pub) => {};
}

macro_rules! syn_trait_impl {
    ($path:ident :: $ty:ident) => {
        impl crate::private::Sealed for $path::$ty {}
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
