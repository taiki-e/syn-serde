use super::*;

ast_struct! {
    /// Data structure sent to a `proc_macro_derive` macro.
    pub struct DeriveInput {
        /// Attributes tagged on the whole struct or enum.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attrs: Vec<Attribute>,

        /// Visibility of the struct or enum.
        #[serde(default, skip_serializing_if = "Visibility::is_inherited")]
        pub vis: Visibility,

        /// Name of the struct or enum.
        pub ident: Ident,

        /// Generics required to complete the definition.
        #[serde(default, skip_serializing_if = "Generics::is_none")]
        pub generics: Generics,

        /// Data within the struct or enum.
        pub data: Data,
    }
}

ast_enum_of_structs! {
    /// The storage of a struct, enum or union data structure.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum Data {
        /// A struct input to a `proc_macro_derive` macro.
        pub Struct(DataStruct {
            pub fields: Fields,
            // #[serde(default, skip_serializing_if = "not")]
            // pub semi_token: bool,
        }),

        /// An enum input to a `proc_macro_derive` macro.
        pub Enum(DataEnum {
            pub variants: Punctuated<Variant>,
        }),

        /// A tagged union input to a `proc_macro_derive` macro.
        pub Union(DataUnion {
            pub fields: FieldsNamed,
        }),
    }
}

mod convert {
    use super::*;

    // DeriveInput

    impl From<&syn::DeriveInput> for DeriveInput {
        fn from(other: &syn::DeriveInput) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                data: other.data.ref_into(),
            }
        }
    }

    impl From<&DeriveInput> for syn::DeriveInput {
        fn from(other: &DeriveInput) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                vis: other.vis.ref_into(),
                ident: other.ident.ref_into(),
                generics: other.generics.ref_into(),
                data: other.data.ref_into(),
            }
        }
    }

    // DataStruct

    /* TODO: document
    /// # Panics
    ///
     */
    impl From<&syn::DataStruct> for DataStruct {
        fn from(other: &syn::DataStruct) -> Self {
            let fields: Fields = other.fields.ref_into();
            assert_struct_semi(&fields, other.semi_token.is_some());

            Self {
                fields: other.fields.ref_into(),
            }
        }
    }

    impl From<&DataStruct> for syn::DataStruct {
        fn from(other: &DataStruct) -> Self {
            Self {
                struct_token: default(),
                fields: other.fields.ref_into(),
                semi_token: default_or_none(!other.fields.is_named()),
            }
        }
    }

    // DataEnum

    impl From<&syn::DataEnum> for DataEnum {
        fn from(other: &syn::DataEnum) -> Self {
            Self {
                variants: other.variants.map_into(),
            }
        }
    }

    impl From<&DataEnum> for syn::DataEnum {
        fn from(other: &DataEnum) -> Self {
            Self {
                enum_token: default(),
                brace_token: default(),
                variants: other.variants.map_into(),
            }
        }
    }

    // DataUnion

    impl From<&syn::DataUnion> for DataUnion {
        fn from(other: &syn::DataUnion) -> Self {
            Self {
                fields: other.fields.ref_into(),
            }
        }
    }

    impl From<&DataUnion> for syn::DataUnion {
        fn from(other: &DataUnion) -> Self {
            Self {
                union_token: default(),
                fields: other.fields.ref_into(),
            }
        }
    }
}
