use proc_macro2::Delimiter::{Brace, Parenthesis};
use proc_macro2::*;
use std::iter::FromIterator;
use syn::punctuated::Punctuated;
use syn::*;

#[macro_use]
mod macros;

fn op(c: char) -> TokenTree {
    Punct::new(c, Spacing::Alone).into()
}

fn lit<T: Into<Literal>>(t: T) -> TokenTree {
    t.into().into()
}

fn ident(sym: &str) -> Ident {
    Ident::new(sym, Span::call_site())
}

fn word(sym: &str) -> TokenTree {
    ident(sym).into()
}

fn delimited(delim: Delimiter, tokens: Vec<TokenTree>) -> TokenTree {
    Group::new(delim, tokens.into_iter().collect()).into()
}

#[test]
fn test_unit() {
    let raw = "struct Unit;";

    let expected = DeriveInput {
        ident: ident("Unit"),
        vis: Visibility::Inherited,
        attrs: Vec::new(),
        generics: Generics::default(),
        data: Data::Struct(DataStruct {
            semi_token: Some(Default::default()),
            struct_token: Default::default(),
            fields: Fields::Unit,
        }),
    };

    let json = r#"
{
  "ident": "Unit",
  "data": {
    "struct": {
      "fields": "unit"
    }
  }
}
"#;

    let raw: DeriveInput = syn::parse_str(raw).unwrap();
    let ser: serde_syn::DeriveInput = serde_json::from_str(json).unwrap();
    let ser = DeriveInput::from(&ser);

    assert_eq!(expected, raw);
    assert_eq!(expected, ser);
    assert_eq!(ser, raw);
}

#[test]
fn test_struct() {
    let raw = "
        #[derive(Debug, Clone)]
        pub struct Item {
            pub ident: Ident,
            pub attrs: Vec<Attribute>
        }
    ";

    let expected = DeriveInput {
        ident: ident("Item"),
        vis: Visibility::Public(VisPublic {
            pub_token: Default::default(),
        }),
        attrs: vec![Attribute {
            bracket_token: Default::default(),
            pound_token: Default::default(),
            style: AttrStyle::Outer,
            path: ident("derive").into(),
            tts: TokenStream::from_iter(vec![delimited(
                Parenthesis,
                vec![word("Debug"), op(','), word("Clone")],
            )]),
        }],
        generics: Generics::default(),
        data: Data::Struct(DataStruct {
            semi_token: None,
            struct_token: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: punctuated![
                    Field {
                        ident: Some(ident("ident")),
                        colon_token: Some(Default::default()),
                        vis: Visibility::Public(VisPublic {
                            pub_token: Default::default(),
                        }),
                        attrs: Vec::new(),
                        ty: TypePath {
                            qself: None,
                            path: ident("Ident").into(),
                        }
                        .into(),
                    },
                    Field {
                        ident: Some(ident("attrs")),
                        colon_token: Some(Default::default()),
                        vis: Visibility::Public(VisPublic {
                            pub_token: Default::default(),
                        }),
                        attrs: Vec::new(),
                        ty: TypePath {
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: punctuated![PathSegment {
                                    ident: ident("Vec"),
                                    arguments: PathArguments::AngleBracketed(
                                        AngleBracketedGenericArguments {
                                            colon2_token: None,
                                            lt_token: Default::default(),
                                            args: punctuated![GenericArgument::Type(Type::from(
                                                TypePath {
                                                    qself: None,
                                                    path: ident("Attribute").into(),
                                                }
                                            )),],
                                            gt_token: Default::default(),
                                        },
                                    ),
                                },],
                            },
                        }
                        .into(),
                    },
                ],
            }),
        }),
    };

    let json = r#"
{
  "attrs": [
    {
      "style": "outer",
      "path": {
        "segments": [
          {
            "ident": "derive"
          }
        ]
      },
      "tts": [
        {
          "group": {
            "delimiter": "parenthesis",
            "stream": [
              {
                "ident": "Debug"
              },
              {
                "punct": {
                  "op": ",",
                  "spacing": "alone"
                }
              },
              {
                "ident": "Clone"
              }
            ]
          }
        }
      ]
    }
  ],
  "vis": "pub",
  "ident": "Item",
  "data": {
    "struct": {
      "fields": {
        "named": [
          {
            "vis": "pub",
            "ident": "ident",
            "colon_token": true,
            "ty": {
              "path": {
                "segments": [
                  {
                    "ident": "Ident"
                  }
                ]
              }
            }
          },
          {
            "vis": "pub",
            "ident": "attrs",
            "colon_token": true,
            "ty": {
              "path": {
                "segments": [
                  {
                    "ident": "Vec",
                    "arguments": {
                      "angle_bracketed": {
                        "args": [
                          {
                            "type": {
                              "path": {
                                "segments": [
                                  {
                                    "ident": "Attribute"
                                  }
                                ]
                              }
                            }
                          }
                        ]
                      }
                    }
                  }
                ]
              }
            }
          }
        ]
      }
    }
  }
}
"#;

    let actual = syn::parse_str(raw).unwrap();
    let json: serde_syn::DeriveInput = serde_json::from_str(json).unwrap();
    let json = DeriveInput::from(&json);

    assert_eq!(expected, actual);
    assert_eq!(expected, json);
    assert_eq!(json, actual);

    let expected_meta_item: Meta = MetaList {
        ident: ident("derive"),
        paren_token: Default::default(),
        nested: punctuated![
            NestedMeta::Meta(Meta::Word(ident("Debug"))),
            NestedMeta::Meta(Meta::Word(ident("Clone"))),
        ],
    }
    .into();

    assert_eq!(
        expected_meta_item,
        actual.attrs[0].interpret_meta().unwrap()
    );
}

#[test]
fn test_union() {
    let raw = "
        union MaybeUninit<T> {
            uninit: (),
            value: T
        }
    ";

    let expected = DeriveInput {
        ident: ident("MaybeUninit"),
        vis: Visibility::Inherited,
        attrs: Vec::new(),
        generics: Generics {
            lt_token: Some(Default::default()),
            params: punctuated![GenericParam::Type(TypeParam {
                attrs: Vec::new(),
                ident: ident("T"),
                bounds: Default::default(),
                default: None,
                colon_token: None,
                eq_token: None,
            }),],
            gt_token: Some(Default::default()),
            where_clause: None,
        },
        data: Data::Union(DataUnion {
            union_token: Default::default(),
            fields: FieldsNamed {
                brace_token: Default::default(),
                named: punctuated![
                    Field {
                        ident: Some(ident("uninit")),
                        colon_token: Some(Default::default()),
                        vis: Visibility::Inherited,
                        attrs: Vec::new(),
                        ty: TypeTuple {
                            paren_token: Default::default(),
                            elems: Punctuated::new(),
                        }
                        .into(),
                    },
                    Field {
                        ident: Some(ident("value")),
                        colon_token: Some(Default::default()),
                        vis: Visibility::Inherited,
                        attrs: Vec::new(),
                        ty: TypePath {
                            qself: None,
                            path: ident("T").into(),
                        }
                        .into(),
                    },
                ],
            },
        }),
    };

    let json = r#"
{
  "ident": "MaybeUninit",
  "generics": {
    "params": [
      {
        "type": {
          "ident": "T"
        }
      }
    ]
  },
  "data": {
    "union": {
      "fields": [
        {
          "ident": "uninit",
          "colon_token": true,
          "ty": {
            "tuple": {
              "elems": []
            }
          }
        },
        {
          "ident": "value",
          "colon_token": true,
          "ty": {
            "path": {
              "segments": [
                {
                  "ident": "T"
                }
              ]
            }
          }
        }
      ]
    }
  }
}
"#;

    let actual = syn::parse_str(raw).unwrap();
    let json: serde_syn::DeriveInput = serde_json::from_str(json).unwrap();
    let json = DeriveInput::from(&json);

    assert_eq!(expected, actual);
    assert_eq!(expected, json);
    assert_eq!(json, actual);
}

#[test]
fn test_enum() {
    let raw = r#"
        /// See the std::result module documentation for details.
        #[must_use]
        pub enum Result<T, E> {
            Ok(T),
            Err(E),
            Surprise = 0isize,

            // Smuggling data into a proc_macro_derive,
            // in the style of https://github.com/dtolnay/proc-macro-hack
            ProcMacroHack = (0, "data").0
        }
    "#;

    let expected = DeriveInput {
        ident: ident("Result"),
        vis: Visibility::Public(VisPublic {
            pub_token: Default::default(),
        }),
        attrs: vec![
            Attribute {
                bracket_token: Default::default(),
                pound_token: Default::default(),
                style: AttrStyle::Outer,
                path: ident("doc").into(),
                tts: TokenStream::from_iter(vec![
                    op('='),
                    lit(Literal::string(
                        " See the std::result module documentation for details.",
                    )),
                ]),
            },
            Attribute {
                bracket_token: Default::default(),
                pound_token: Default::default(),
                style: AttrStyle::Outer,
                path: ident("must_use").into(),
                tts: TokenStream::new(),
            },
        ],
        generics: Generics {
            lt_token: Some(Default::default()),
            params: punctuated![
                GenericParam::Type(TypeParam {
                    attrs: Vec::new(),
                    ident: ident("T"),
                    bounds: Default::default(),
                    default: None,
                    colon_token: None,
                    eq_token: None,
                }),
                GenericParam::Type(TypeParam {
                    attrs: Vec::new(),
                    ident: ident("E"),
                    bounds: Default::default(),
                    colon_token: None,
                    eq_token: None,
                    default: None,
                }),
            ],
            gt_token: Some(Default::default()),
            where_clause: None,
        },
        data: Data::Enum(DataEnum {
            variants: punctuated![
                Variant {
                    ident: ident("Ok"),
                    attrs: Vec::new(),
                    fields: Fields::Unnamed(FieldsUnnamed {
                        paren_token: Default::default(),
                        unnamed: punctuated![Field {
                            colon_token: None,
                            ident: None,
                            vis: Visibility::Inherited,
                            attrs: Vec::new(),
                            ty: TypePath {
                                qself: None,
                                path: ident("T").into(),
                            }
                            .into(),
                        },],
                    }),
                    discriminant: None,
                },
                Variant {
                    ident: ident("Err"),
                    attrs: Vec::new(),
                    fields: Fields::Unnamed(FieldsUnnamed {
                        paren_token: Default::default(),
                        unnamed: punctuated![Field {
                            ident: None,
                            colon_token: None,
                            vis: Visibility::Inherited,
                            attrs: Vec::new(),
                            ty: TypePath {
                                qself: None,
                                path: ident("E").into(),
                            }
                            .into(),
                        },],
                    }),
                    discriminant: None,
                },
                Variant {
                    ident: ident("Surprise"),
                    attrs: Vec::new(),
                    fields: Fields::Unit,
                    discriminant: Some((
                        Default::default(),
                        Expr::Lit(ExprLit {
                            attrs: Vec::new(),
                            lit: Lit::Int(LitInt::new(0, IntSuffix::Isize, Span::call_site())),
                        }),
                    )),
                },
                Variant {
                    ident: ident("ProcMacroHack"),
                    attrs: Vec::new(),
                    fields: Fields::Unit,
                    discriminant: Some((
                        Default::default(),
                        Expr::Field(ExprField {
                            attrs: Vec::new(),
                            base: Box::new(Expr::Tuple(ExprTuple {
                                attrs: Vec::new(),
                                paren_token: Default::default(),
                                elems: punctuated![
                                    Expr::Lit(ExprLit {
                                        attrs: Vec::new(),
                                        lit: Lit::Int(LitInt::new(
                                            0,
                                            IntSuffix::None,
                                            Span::call_site()
                                        )),
                                    }),
                                    Expr::Lit(ExprLit {
                                        attrs: Vec::new(),
                                        lit: Lit::Str(LitStr::new("data", Span::call_site())),
                                    }),
                                ],
                            })),
                            dot_token: Default::default(),
                            member: Member::Unnamed(Index {
                                index: 0,
                                span: Span::call_site(),
                            }),
                        }),
                    )),
                },
            ],
            brace_token: Default::default(),
            enum_token: Default::default(),
        }),
    };

    let json = r#"
{
  "attrs": [
    {
      "style": "outer",
      "path": {
        "segments": [
          {
            "ident": "doc"
          }
        ]
      },
      "tts": [
        {
          "punct": {
            "op": "=",
            "spacing": "alone"
          }
        },
        {
          "lit": "\" See the std::result module documentation for details.\""
        }
      ]
    },
    {
      "style": "outer",
      "path": {
        "segments": [
          {
            "ident": "must_use"
          }
        ]
      }
    }
  ],
  "vis": "pub",
  "ident": "Result",
  "generics": {
    "params": [
      {
        "type": {
          "ident": "T"
        }
      },
      {
        "type": {
          "ident": "E"
        }
      }
    ]
  },
  "data": {
    "enum": {
      "variants": [
        {
          "ident": "Ok",
          "fields": {
            "unnamed": [
              {
                "ty": {
                  "path": {
                    "segments": [
                      {
                        "ident": "T"
                      }
                    ]
                  }
                }
              }
            ]
          }
        },
        {
          "ident": "Err",
          "fields": {
            "unnamed": [
              {
                "ty": {
                  "path": {
                    "segments": [
                      {
                        "ident": "E"
                      }
                    ]
                  }
                }
              }
            ]
          }
        },
        {
          "ident": "Surprise",
          "fields": "unit",
          "discriminant": {
            "lit": {
              "int": "0isize"
            }
          }
        },
        {
          "ident": "ProcMacroHack",
          "fields": "unit",
          "discriminant": {
            "field": {
              "base": {
                "tuple": {
                  "elems": [
                    {
                      "lit": {
                        "int": "0"
                      }
                    },
                    {
                      "lit": {
                        "str": "\"data\""
                      }
                    }
                  ]
                }
              },
              "index": 0
            }
          }
        }
      ]
    }
  }
}
"#;

    let actual = syn::parse_str(raw).unwrap();
    let json: serde_syn::DeriveInput = serde_json::from_str(json).unwrap();
    let json = DeriveInput::from(&json);

    assert_eq!(expected, actual);
    assert_eq!(expected, json);
    assert_eq!(json, actual);

    let expected_meta_items = vec![
        MetaNameValue {
            ident: ident("doc"),
            eq_token: Default::default(),
            lit: Lit::Str(LitStr::new(
                " See the std::result module documentation for details.",
                Span::call_site(),
            )),
        }
        .into(),
        Meta::Word(ident("must_use")),
    ];

    let actual_meta_items: Vec<_> = actual
        .attrs
        .into_iter()
        .map(|attr| attr.interpret_meta().unwrap())
        .collect();

    assert_eq!(expected_meta_items, actual_meta_items);
}

#[test]
fn test_attr_with_path() {
    let raw = r#"
        #[::attr_args::identity
            fn main() { assert_eq!(foo(), "Hello, world!"); }]
        struct Dummy;
    "#;

    let expected = DeriveInput {
        ident: ident("Dummy"),
        vis: Visibility::Inherited,
        attrs: vec![Attribute {
            bracket_token: Default::default(),
            pound_token: Default::default(),
            style: AttrStyle::Outer,
            path: Path {
                leading_colon: Some(Default::default()),
                segments: punctuated![
                    PathSegment::from(ident("attr_args")),
                    PathSegment::from(ident("identity")),
                ],
            },
            tts: TokenStream::from_iter(vec![
                word("fn"),
                word("main"),
                delimited(Parenthesis, vec![]),
                delimited(
                    Brace,
                    vec![
                        word("assert_eq"),
                        op('!'),
                        delimited(
                            Parenthesis,
                            vec![
                                word("foo"),
                                delimited(Parenthesis, vec![]),
                                op(','),
                                lit(Literal::string("Hello, world!")),
                            ],
                        ),
                        op(';'),
                    ],
                ),
            ]),
        }],
        generics: Generics::default(),
        data: Data::Struct(DataStruct {
            fields: Fields::Unit,
            semi_token: Some(Default::default()),
            struct_token: Default::default(),
        }),
    };

    let json = r#"
{
  "attrs": [
    {
      "style": "outer",
      "path": {
        "leading_colon": true,
        "segments": [
          {
            "ident": "attr_args"
          },
          {
            "ident": "identity"
          }
        ]
      },
      "tts": [
        {
          "ident": "fn"
        },
        {
          "ident": "main"
        },
        {
          "group": {
            "delimiter": "parenthesis",
            "stream": []
          }
        },
        {
          "group": {
            "delimiter": "brace",
            "stream": [
              {
                "ident": "assert_eq"
              },
              {
                "punct": {
                  "op": "!",
                  "spacing": "alone"
                }
              },
              {
                "group": {
                  "delimiter": "parenthesis",
                  "stream": [
                    {
                      "ident": "foo"
                    },
                    {
                      "group": {
                        "delimiter": "parenthesis",
                        "stream": []
                      }
                    },
                    {
                      "punct": {
                        "op": ",",
                        "spacing": "alone"
                      }
                    },
                    {
                      "lit": "\"Hello, world!\""
                    }
                  ]
                }
              },
              {
                "punct": {
                  "op": ";",
                  "spacing": "alone"
                }
              }
            ]
          }
        }
      ]
    }
  ],
  "ident": "Dummy",
  "data": {
    "struct": {
      "fields": "unit"
    }
  }
}
"#;

    let actual = syn::parse_str(raw).unwrap();
    let json: serde_syn::DeriveInput = serde_json::from_str(json).unwrap();
    let json = DeriveInput::from(&json);

    assert_eq!(expected, actual);
    assert_eq!(expected, json);
    assert_eq!(json, actual);

    assert!(actual.attrs[0].interpret_meta().is_none());
}

#[test]
fn test_attr_with_non_mod_style_path() {
    let raw = r#"
        #[inert <T>]
        struct S;
    "#;

    let expected = DeriveInput {
        ident: ident("S"),
        vis: Visibility::Inherited,
        attrs: vec![Attribute {
            bracket_token: Default::default(),
            pound_token: Default::default(),
            style: AttrStyle::Outer,
            path: Path {
                leading_colon: None,
                segments: punctuated![PathSegment::from(ident("inert"))],
            },
            tts: TokenStream::from_iter(vec![op('<'), word("T"), op('>')]),
        }],
        generics: Generics::default(),
        data: Data::Struct(DataStruct {
            fields: Fields::Unit,
            semi_token: Some(Default::default()),
            struct_token: Default::default(),
        }),
    };

    let json = r#"
{
  "attrs": [
    {
      "style": "outer",
      "path": {
        "segments": [
          {
            "ident": "inert"
          }
        ]
      },
      "tts": [
        {
          "punct": {
            "op": "<",
            "spacing": "alone"
          }
        },
        {
          "ident": "T"
        },
        {
          "punct": {
            "op": ">",
            "spacing": "alone"
          }
        }
      ]
    }
  ],
  "ident": "S",
  "data": {
    "struct": {
      "fields": "unit"
    }
  }
}
"#;

    let actual = syn::parse_str(raw).unwrap();
    let json: serde_syn::DeriveInput = serde_json::from_str(json).unwrap();
    let json = DeriveInput::from(&json);

    assert_eq!(expected, actual);
    assert_eq!(expected, json);
    assert_eq!(json, actual);

    assert!(actual.attrs[0].interpret_meta().is_none());
}

#[test]
fn test_attr_with_mod_style_path_with_self() {
    let raw = r#"
        #[foo::self]
        struct S;
    "#;

    let expected = DeriveInput {
        ident: ident("S"),
        vis: Visibility::Inherited,
        attrs: vec![Attribute {
            bracket_token: Default::default(),
            pound_token: Default::default(),
            style: AttrStyle::Outer,
            path: Path {
                leading_colon: None,
                segments: punctuated![
                    PathSegment::from(ident("foo")),
                    PathSegment::from(ident("self")),
                ],
            },
            tts: TokenStream::new(),
        }],
        generics: Generics::default(),
        data: Data::Struct(DataStruct {
            fields: Fields::Unit,
            semi_token: Some(Default::default()),
            struct_token: Default::default(),
        }),
    };

    let json = r#"
{
  "attrs": [
    {
      "style": "outer",
      "path": {
        "segments": [
          {
            "ident": "foo"
          },
          {
            "ident": "self"
          }
        ]
      }
    }
  ],
  "ident": "S",
  "data": {
    "struct": {
      "fields": "unit"
    }
  }
}
"#;

    let actual = syn::parse_str(raw).unwrap();
    let json: serde_syn::DeriveInput = serde_json::from_str(json).unwrap();
    let json = DeriveInput::from(&json);

    assert_eq!(expected, actual);
    assert_eq!(expected, json);
    assert_eq!(json, actual);

    assert!(actual.attrs[0].interpret_meta().is_none());
}

#[test]
fn test_pub_restricted() {
    // Taken from tests/rust/src/test/ui/resolve/auxiliary/privacy-struct-ctor.rs
    let raw = r#"
        pub(in m) struct Z(pub(in m::n) u8);
    "#;

    let expected = DeriveInput {
        ident: ident("Z"),
        vis: Visibility::Restricted(VisRestricted {
            path: Box::new(ident("m").into()),
            in_token: Some(Default::default()),
            paren_token: Default::default(),
            pub_token: Default::default(),
        }),
        attrs: vec![],
        generics: Generics::default(),
        data: Data::Struct(DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed {
                paren_token: Default::default(),
                unnamed: punctuated![Field {
                    ident: None,
                    vis: Visibility::Restricted(VisRestricted {
                        path: Box::new(Path {
                            leading_colon: None,
                            segments: punctuated![
                                PathSegment::from(ident("m")),
                                PathSegment::from(ident("n")),
                            ],
                        }),
                        in_token: Some(Default::default()),
                        paren_token: Default::default(),
                        pub_token: Default::default(),
                    }),
                    colon_token: None,
                    attrs: vec![],
                    ty: TypePath {
                        qself: None,
                        path: ident("u8").into(),
                    }
                    .into(),
                },],
            }),
            semi_token: Some(Default::default()),
            struct_token: Default::default(),
        }),
    };

    let json = r#"
{
  "vis": {
    "restricted": {
      "in_token": true,
      "path": {
        "segments": [
          {
            "ident": "m"
          }
        ]
      }
    }
  },
  "ident": "Z",
  "data": {
    "struct": {
      "fields": {
        "unnamed": [
          {
            "vis": {
              "restricted": {
                "in_token": true,
                "path": {
                  "segments": [
                    {
                      "ident": "m"
                    },
                    {
                      "ident": "n"
                    }
                  ]
                }
              }
            },
            "ty": {
              "path": {
                "segments": [
                  {
                    "ident": "u8"
                  }
                ]
              }
            }
          }
        ]
      }
    }
  }
}
"#;

    let actual = syn::parse_str(raw).unwrap();
    let json: serde_syn::DeriveInput = serde_json::from_str(json).unwrap();
    let json = DeriveInput::from(&json);

    assert_eq!(expected, actual);
    assert_eq!(expected, json);
    assert_eq!(json, actual);

    assert_eq!(expected, actual);
}

#[test]
fn test_vis_crate() {
    let raw = r#"
        crate struct S;
    "#;

    let expected = DeriveInput {
        ident: ident("S"),
        vis: Visibility::Crate(VisCrate {
            crate_token: Default::default(),
        }),
        attrs: vec![],
        generics: Generics::default(),
        data: Data::Struct(DataStruct {
            semi_token: Some(Default::default()),
            struct_token: Default::default(),
            fields: Fields::Unit,
        }),
    };

    let json = r#"
{
  "vis": "crate",
  "ident": "S",
  "data": {
    "struct": {
      "fields": "unit"
    }
  }
}
"#;

    let actual = syn::parse_str(raw).unwrap();
    let json: serde_syn::DeriveInput = serde_json::from_str(json).unwrap();
    let json = DeriveInput::from(&json);

    assert_eq!(expected, actual);
    assert_eq!(expected, json);
    assert_eq!(json, actual);
}

#[test]
fn test_pub_restricted_crate() {
    let raw = r#"
        pub(crate) struct S;
    "#;

    let expected = DeriveInput {
        ident: ident("S"),
        vis: Visibility::Restricted(VisRestricted {
            pub_token: Default::default(),
            paren_token: Default::default(),
            in_token: None,
            path: Box::new(ident("crate").into()),
        }),
        attrs: vec![],
        generics: Generics::default(),
        data: Data::Struct(DataStruct {
            semi_token: Some(Default::default()),
            struct_token: Default::default(),
            fields: Fields::Unit,
        }),
    };

    let json = r#"
{
  "vis": {
    "restricted": {
      "path": {
        "segments": [
          {
            "ident": "crate"
          }
        ]
      }
    }
  },
  "ident": "S",
  "data": {
    "struct": {
      "fields": "unit"
    }
  }
}
"#;

    let actual = syn::parse_str(raw).unwrap();
    let json: serde_syn::DeriveInput = serde_json::from_str(json).unwrap();
    let json = DeriveInput::from(&json);

    assert_eq!(expected, actual);
    assert_eq!(expected, json);
    assert_eq!(json, actual);
}

#[test]
fn test_pub_restricted_super() {
    let raw = r#"
        pub(super) struct S;
    "#;

    let expected = DeriveInput {
        ident: ident("S"),
        vis: Visibility::Restricted(VisRestricted {
            path: Box::new(ident("super").into()),
            in_token: None,
            paren_token: Default::default(),
            pub_token: Default::default(),
        }),
        attrs: vec![],
        generics: Generics::default(),
        data: Data::Struct(DataStruct {
            semi_token: Some(Default::default()),
            struct_token: Default::default(),
            fields: Fields::Unit,
        }),
    };

    let json = r#"
{
  "vis": {
    "restricted": {
      "path": {
        "segments": [
          {
            "ident": "super"
          }
        ]
      }
    }
  },
  "ident": "S",
  "data": {
    "struct": {
      "fields": "unit"
    }
  }
}
"#;

    let actual = syn::parse_str(raw).unwrap();
    let json: serde_syn::DeriveInput = serde_json::from_str(json).unwrap();
    let json = DeriveInput::from(&json);

    assert_eq!(expected, actual);
    assert_eq!(expected, json);
    assert_eq!(json, actual);
}

#[test]
fn test_pub_restricted_in_super() {
    let raw = r#"
        pub(in super) struct S;
    "#;

    let expected = DeriveInput {
        ident: ident("S"),
        vis: Visibility::Restricted(VisRestricted {
            path: Box::new(ident("super").into()),
            in_token: Some(Default::default()),
            paren_token: Default::default(),
            pub_token: Default::default(),
        }),
        attrs: vec![],
        generics: Generics::default(),
        data: Data::Struct(DataStruct {
            semi_token: Some(Default::default()),
            struct_token: Default::default(),
            fields: Fields::Unit,
        }),
    };

    let json = r#"
{
  "vis": {
    "restricted": {
      "in_token": true,
      "path": {
        "segments": [
          {
            "ident": "super"
          }
        ]
      }
    }
  },
  "ident": "S",
  "data": {
    "struct": {
      "fields": "unit"
    }
  }
}
"#;

    let actual = syn::parse_str(raw).unwrap();
    let json: serde_syn::DeriveInput = serde_json::from_str(json).unwrap();
    let json = DeriveInput::from(&json);

    assert_eq!(expected, actual);
    assert_eq!(expected, json);
    assert_eq!(json, actual);
}

/*
#[test]
fn test_fields_on_unit_struct() {
    let raw = "struct S;";
    let struct_body = match syn::parse_str::<DeriveInput>(raw).unwrap().data {
        Data::Struct(body) => body,
        _ => panic!("expected a struct"),
    };

    assert_eq!(0, struct_body.fields.iter().count());
}

#[test]
fn test_fields_on_named_struct() {
    let raw = "struct S {
        foo: i32,
        pub bar: String,
    }";
    let struct_body = match syn::parse_str::<DeriveInput>(raw).unwrap().data {
        Data::Struct(body) => body,
        _ => panic!("expected a struct"),
    };

    let expected = vec![
        Field {
            attrs: vec![],
            vis: Visibility::Inherited,
            ident: Some(ident("foo")),
            colon_token: Some(Default::default()),
            ty: syn::parse_str("i32").unwrap(),
        },
        Field {
            attrs: vec![],
            vis: Visibility::Public(VisPublic {
                pub_token: Default::default(),
            }),
            ident: Some(ident("bar")),
            colon_token: Some(Default::default()),
            ty: syn::parse_str("String").unwrap(),
        },
    ];
    let expected = expected.iter().collect::<Vec<_>>();

    assert_eq!(expected, struct_body.fields.iter().collect::<Vec<_>>());
}

#[test]
fn test_fields_on_tuple_struct() {
    let raw = "struct S(i32, pub String);";
    let struct_body = match syn::parse_str::<DeriveInput>(raw).unwrap().data {
        Data::Struct(body) => body,
        _ => panic!("expected a struct"),
    };

    let expected = vec![
        Field {
            attrs: vec![],
            vis: Visibility::Inherited,
            ident: None,
            colon_token: None,
            ty: syn::parse_str("i32").unwrap(),
        },
        Field {
            attrs: vec![],
            vis: Visibility::Public(VisPublic {
                pub_token: Default::default(),
            }),
            ident: None,
            colon_token: None,
            ty: syn::parse_str("String").unwrap(),
        },
    ];
    let expected = expected.iter().collect::<Vec<_>>();

    assert_eq!(expected, struct_body.fields.iter().collect::<Vec<_>>());
}
*/

#[test]
fn test_ambiguous_crate() {
    // The field type is `(crate::X)` not `crate (::X)`.
    let raw = "struct S(crate::X);";

    let expected = DeriveInput {
        ident: ident("S"),
        vis: Visibility::Inherited,
        attrs: vec![],
        generics: Generics::default(),
        data: Data::Struct(DataStruct {
            struct_token: Default::default(),
            fields: Fields::Unnamed(FieldsUnnamed {
                paren_token: Default::default(),
                unnamed: punctuated![Field {
                    attrs: Vec::new(),
                    vis: Visibility::Inherited,
                    ident: None,
                    colon_token: None,
                    ty: Type::Path(TypePath {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: punctuated![ident("crate").into(), ident("X").into(),],
                        },
                    }),
                }],
            }),
            semi_token: Some(Default::default()),
        }),
    };

    let json = r#"
{
  "ident": "S",
  "data": {
    "struct": {
      "fields": {
        "unnamed": [
          {
            "ty": {
              "path": {
                "segments": [
                  {
                    "ident": "crate"
                  },
                  {
                    "ident": "X"
                  }
                ]
              }
            }
          }
        ]
      }
    }
  }
}
"#;

    let actual = syn::parse_str(raw).unwrap();
    let json: serde_syn::DeriveInput = serde_json::from_str(json).unwrap();
    let json = DeriveInput::from(&json);

    assert_eq!(expected, actual);
    assert_eq!(expected, json);
    assert_eq!(json, actual);
}
