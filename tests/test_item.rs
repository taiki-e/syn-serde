#![warn(rust_2018_idioms, single_use_lifetimes)]

use syn::*;

#[test]
fn test_unit() {
    let raw = "struct Unit;";

    let json = r#"
    {
      "struct": {
        "ident": "Unit",
        "fields": "unit"
      }
    }
    "#;

    let actual = syn::parse_str(raw).unwrap();
    let ser: syn_serde::Item = serde_json::from_str(json).unwrap();
    let ser = Item::from(&ser);
    assert_eq!(ser, actual);
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

    let json = r#"
    {
      "struct": {
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
            "tokens": [
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
    "#;

    let actual = syn::parse_str(raw).unwrap();
    let json: syn_serde::Item = serde_json::from_str(json).unwrap();
    let json = Item::from(&json);
    assert_eq!(json, actual);
}

#[test]
fn test_union() {
    let raw = "
        union MaybeUninit<T> {
            uninit: (),
            value: T
        }
    ";

    let json = r#"
    {
      "union": {
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
    "#;

    let actual = syn::parse_str(raw).unwrap();
    let json: syn_serde::Item = serde_json::from_str(json).unwrap();
    let json = Item::from(&json);
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

    let json = r#"
    {
      "enum": {
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
            "tokens": [
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
    "#;

    let actual = syn::parse_str(raw).unwrap();
    let json: syn_serde::Item = serde_json::from_str(json).unwrap();
    let json = Item::from(&json);
    assert_eq!(json, actual);
}

#[test]
fn test_attr_with_path() {
    let raw = r#"
        #[::attr_args::identity
            fn main() { assert_eq!(foo(), "Hello, world!"); }]
        struct Dummy;
    "#;

    let json = r#"
    {
      "struct": {
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
            "tokens": [
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
        "fields": "unit"
      }
    }
    "#;

    let actual = syn::parse_str(raw).unwrap();
    let json: syn_serde::Item = serde_json::from_str(json).unwrap();
    let json = Item::from(&json);
    assert_eq!(json, actual);
}

#[test]
fn test_attr_with_non_mod_style_path() {
    let raw = r#"
        #[inert <T>]
        struct S;
    "#;

    let json = r#"
    {
      "struct": {
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
            "tokens": [
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
        "fields": "unit"
      }
    }
    "#;

    let actual = syn::parse_str(raw).unwrap();
    let json: syn_serde::Item = serde_json::from_str(json).unwrap();
    let json = Item::from(&json);
    assert_eq!(json, actual);
}

#[test]
fn test_attr_with_mod_style_path_with_self() {
    let raw = r#"
        #[foo::self]
        struct S;
    "#;

    let json = r#"
    {
      "struct": {
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
        "fields": "unit"
      }
    }
    "#;

    let actual = syn::parse_str(raw).unwrap();
    let json: syn_serde::Item = serde_json::from_str(json).unwrap();
    let json = Item::from(&json);
    assert_eq!(json, actual);
}

#[test]
fn test_pub_restricted() {
    // Taken from tests/rust/src/test/ui/resolve/auxiliary/privacy-struct-ctor.rs
    let raw = r#"
        pub(in m) struct Z(pub(in m::n) u8);
    "#;

    let json = r#"
    {
      "struct": {
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
    "#;

    let actual = syn::parse_str(raw).unwrap();
    let json: syn_serde::Item = serde_json::from_str(json).unwrap();
    let json = Item::from(&json);
    assert_eq!(json, actual);
}

#[test]
fn test_vis_crate() {
    let raw = r#"
        crate struct S;
    "#;

    let json = r#"
    {
      "struct": {
        "vis": "crate",
        "ident": "S",
        "fields": "unit"
      }
    }
    "#;

    let actual = syn::parse_str(raw).unwrap();
    let json: syn_serde::Item = serde_json::from_str(json).unwrap();
    let json = Item::from(&json);
    assert_eq!(json, actual);
}

#[test]
fn test_pub_restricted_crate() {
    let raw = r#"
        pub(crate) struct S;
    "#;

    let json = r#"
    {
      "struct": {
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
        "fields": "unit"
      }
    }
    "#;

    let actual = syn::parse_str(raw).unwrap();
    let json: syn_serde::Item = serde_json::from_str(json).unwrap();
    let json = Item::from(&json);
    assert_eq!(json, actual);
}

#[test]
fn test_pub_restricted_super() {
    let raw = r#"
        pub(super) struct S;
    "#;

    let json = r#"
    {
      "struct": {
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
        "fields": "unit"
      }
    }
    "#;

    let actual = syn::parse_str(raw).unwrap();
    let json: syn_serde::Item = serde_json::from_str(json).unwrap();
    let json = Item::from(&json);
    assert_eq!(json, actual);
}

#[test]
fn test_pub_restricted_in_super() {
    let raw = r#"
        pub(in super) struct S;
    "#;

    let json = r#"
    {
      "struct": {
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
        "fields": "unit"
      }
    }
    "#;

    let actual = syn::parse_str(raw).unwrap();
    let json: syn_serde::Item = serde_json::from_str(json).unwrap();
    let json = Item::from(&json);
    assert_eq!(json, actual);
}

#[test]
fn test_ambiguous_crate() {
    // The field type is `(crate::X)` not `crate (::X)`.
    let raw = "struct S(crate::X);";

    let json = r#"
    {
      "struct": {
        "ident": "S",
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
    "#;

    let actual = syn::parse_str(raw).unwrap();
    let json: syn_serde::Item = serde_json::from_str(json).unwrap();
    let json = Item::from(&json);
    assert_eq!(json, actual);
}
