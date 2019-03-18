macro_rules! ast_struct {
    (
        $(#[$attr:meta])*
        pub struct $name:ident #transparent $($rest:tt)*
    ) => {
        $(#[$attr])*
        #[derive(crate::Serialize, crate::Deserialize)]
        #[serde(transparent)]
        pub struct $name $($rest)*
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident $($rest:tt)*
    ) => {
        $(#[$attr])*
        #[derive(crate::Serialize, crate::Deserialize)]
        pub struct $name $($rest)*
    };
}

macro_rules! ast_enum {
    (
        $(#[$enum_attr:meta])*
        pub enum $name:ident #manual_from_impl { $($variants:tt)* }
    ) => (
        $(#[$enum_attr])*
        #[derive(crate::Serialize, crate::Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            $($variants)*
        }
    );

    (
        $(#[$enum_attr:meta])*
        pub enum $name:ident $(# $tags:ident)* {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident $( ($member:ident $($rest:tt)*) )*,
            )*
        }
    ) => (
        $(#[$enum_attr])*
        #[derive(crate::Serialize, crate::Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            $(
                $(#[$variant_attr])*
                $variant $( ($member $($rest)*) )*,
            )*
        }

        impl From<&syn::$name> for $name {
            fn from(other: &syn::$name) -> Self {
                match other {
                    $(
                        syn::$name::$variant(x) => $name::$variant(x.into()),
                    )*
                }
            }
        }

        impl From<&$name> for syn::$name {
            fn from(other: &$name) -> Self {
                match other {
                    $(
                        $name::$variant(x) => syn::$name::$variant(x.into()),
                    )*
                }
            }
        }
    );
}

macro_rules! ast_enum_of_structs {
    (
        $(#[$enum_attr:meta])*
        pub enum $name:ident $(# $tags:ident)* {
            $(
                $(#[$variant_attr:meta])*
                pub $variant:ident $( ($member:ident $($rest:tt)*) )*,
            )*
        }
    ) => (
        ast_enum! {
            $(#[$enum_attr])*
            pub enum $name $(# $tags)* {
                $(
                    $(#[$variant_attr])*
                    $variant $( ($member) )*,
                )*
            }
        }

        $(
            maybe_ast_struct! {
                $(#[$variant_attr])*
                $(
                    pub struct $member $($rest)*
                )*
            }

            $(
                impl From<$member> for $name {
                    fn from(e: $member) -> Self {
                        $name::$variant(e)
                    }
                }
            )*
        )*
    )
}

macro_rules! maybe_ast_struct {
    (
        $(#[$attr:meta])*
        $(
            pub struct $name:ident
        )*
    ) => ();

    ($($rest:tt)*) => (ast_struct! { $($rest)* });
}
