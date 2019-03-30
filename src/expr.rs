use super::*;
use proc_macro2::Span;

ast_enum_of_structs! {
    /// A Rust expression.
    ///
    /// # Syntax tree enums
    ///
    /// This type is a syntax tree enum.
    ///
    /// See the [documentation of Syn](https://docs.rs/syn/0.15/syn/enum.Expr.html#syntax-tree-enums)
    /// for more.
    pub enum Expr {
        /// A box expression: `box f`.
        pub Box(ExprBox {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            expr: Box<Expr>,
        }),

        /// A placement expression: `place <- value`.
        pub InPlace(ExprInPlace {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            place: Box<Expr>,
            value: Box<Expr>,
        }),

        /// A slice literal expression: `[a, b, c, d]`.
        pub Array(ExprArray {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            elems: Punctuated<Expr>,
        }),

        /// A function call expression: `invoke(a, b)`.
        pub Call(ExprCall {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            func: Box<Expr>,
            args: Punctuated<Expr>,
        }),

        /// A method call expression: `x.foo::<T>(a, b)`.
        pub MethodCall(ExprMethodCall {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            receiver: Box<Expr>,
            method: Ident,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            turbofish: Option<MethodTurbofish>,
            args: Punctuated<Expr>,
        }),

        /// A tuple expression: `(a, b, c, d)`.
        pub Tuple(ExprTuple {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            elems: Punctuated<Expr>,
        }),

        /// A binary operation: `a + b`, `a * b`.
        pub Binary(ExprBinary {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            left: Box<Expr>,
            op: BinOp,
            right: Box<Expr>,
        }),

        /// A unary operation: `!x`, `*x`.
        pub Unary(ExprUnary {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            op: UnOp,
            expr: Box<Expr>,
        }),

        /// A literal in place of an expression: `1`, `"foo"`.
        pub Lit(ExprLit {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(flatten)]
            lit: Lit,
        }),

        /// A cast expression: `foo as f64`.
        pub Cast(ExprCast {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            expr: Box<Expr>,
            ty: Box<Type>,
        }),

        /// A type ascription expression: `foo: f64`.
        pub Type(ExprType {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            expr: Box<Expr>,
            ty: Box<Type>,
        }),

        /// A `let` guard: `let Some(x) = opt`.
        pub Let(ExprLet {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            pats: Punctuated<Pat>,
            expr: Box<Expr>,
        }),

        /// An `if` expression with an optional `else` block: `if expr { ... }
        /// else { ... }`.
        ///
        /// The `else` branch expression may only be an `If` or `Block`
        /// expression, not any of the other types of expression.
        pub If(ExprIf {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            cond: Box<Expr>,
            then_branch: Block,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            else_branch: Option<Box<Expr>>,
        }),

        /// A while loop: `while expr { ... }`.
        pub While(ExprWhile {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            label: Option<Label>,
            cond: Box<Expr>,
            body: Block,
        }),

        /// A for loop: `for pat in expr { ... }`.
        pub ForLoop(ExprForLoop {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            label: Option<Label>,
            pat: Box<Pat>,
            expr: Box<Expr>,
            body: Block,
        }),

        /// Conditionless loop: `loop { ... }`.
        pub Loop(ExprLoop {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            label: Option<Label>,
            body: Block,
        }),

        /// A `match` expression: `match n { Some(n) => {}, None => {} }`.
        pub Match(ExprMatch {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            expr: Box<Expr>,
            arms: Vec<Arm>,
        }),

        /// A closure expression: `|a, b| a + b`.
        pub Closure(ExprClosure {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(rename = "async")]
            #[serde(default, skip_serializing_if = "not")]
            asyncness: bool,
            #[serde(default, skip_serializing_if = "not")]
            movability: bool,
            #[serde(rename = "move")]
            #[serde(default, skip_serializing_if = "not")]
            capture: bool,
            inputs: Punctuated<FnArg>,
            #[serde(default, skip_serializing_if = "ReturnType::is_none")]
            output: ReturnType,
            body: Box<Expr>,
        }),

        /// An unsafe block: `unsafe { ... }`.
        pub Unsafe(ExprUnsafe {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(flatten)]
            block: Block,
        }),

        /// A blocked scope: `{ ... }`.
        pub Block(ExprBlock {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            label: Option<Label>,
            #[serde(flatten)]
            block: Block,
        }),

        /// An assignment expression: `a = compute()`.
        pub Assign(ExprAssign {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            left: Box<Expr>,
            right: Box<Expr>,
        }),

        /// A compound assignment expression: `counter += 1`.
        pub AssignOp(ExprAssignOp {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            left: Box<Expr>,
            op: BinOp,
            right: Box<Expr>,
        }),

        /// Access of a named struct field (`obj.k`) or unnamed tuple struct
        /// field (`obj.0`).
        pub Field(ExprField {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            base: Box<Expr>,
            #[serde(flatten)]
            member: Member,
        }),

        /// A square bracketed indexing expression: `vector[2]`.
        pub Index(ExprIndex {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            expr: Box<Expr>,
            index: Box<Expr>,
        }),

        /// A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.
        pub Range(ExprRange {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            from: Option<Box<Expr>>,
            limits: RangeLimits,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            to: Option<Box<Expr>>,
        }),

        /// A path like `std::mem::replace` possibly containing generic
        /// parameters and a qualified self-type.
        ///
        /// A plain identifier like `x` is a path of length 1.
        pub Path(ExprPath {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            qself: Option<QSelf>,
            #[serde(flatten)]
            path: Path,
        }),

        /// A referencing operation: `&a` or `&mut a`.
        pub Reference(ExprReference {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(rename = "mut")]
            #[serde(default, skip_serializing_if = "not")]
            mutability: bool,
            expr: Box<Expr>,
        }),

        /// A `break`, with an optional label to break and an optional
        /// expression.
        pub Break(ExprBreak {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            label: Option<Lifetime>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            expr: Option<Box<Expr>>,
        }),

        /// A `continue`, with an optional label.
        pub Continue(ExprContinue {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            label: Option<Lifetime>,
        }),

        /// A `return`, with an optional value to be returned.
        pub Return(ExprReturn {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            expr: Option<Box<Expr>>,
        }),

        /// A macro invocation expression: `format!("{}", q)`.
        pub Macro(ExprMacro {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(flatten)]
            mac: Macro,
        }),

        /// A struct literal expression: `Point { x: 1, y: 1 }`.
        ///
        /// The `rest` provides the value of the remaining fields as in `S { a:
        /// 1, b: 1, ..rest }`.
        pub Struct(ExprStruct {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            path: Path,
            fields: Punctuated<FieldValue>,
            #[serde(default, skip_serializing_if = "not")]
            dot2_token: bool,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            rest: Option<Box<Expr>>,
        }),

        /// An array literal constructed from one repeated element: `[0u8; N]`.
        pub Repeat(ExprRepeat {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            expr: Box<Expr>,
            len: Box<Expr>,
        }),

        /// A parenthesized expression: `(a + b)`.
        pub Paren(ExprParen {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            expr: Box<Expr>,
        }),

        /// An expression contained within invisible delimiters.
        ///
        /// This variant is important for faithfully representing the precedence
        /// of expressions and is related to `None`-delimited spans in a
        /// `TokenStream`.
        pub Group(ExprGroup {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            expr: Box<Expr>,
        }),

        /// A try-expression: `expr?`.
        pub Try(ExprTry {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            expr: Box<Expr>,
        }),

        /// An async block: `async { ... }`.
        pub Async(ExprAsync {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(rename = "move")]
            #[serde(default, skip_serializing_if = "not")]
            capture: bool,
            block: Block,
        }),

        /// A try block: `try { ... }`.
        pub TryBlock(ExprTryBlock {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            block: Block,
        }),

        /// A yield expression: `yield expr`.
        pub Yield(ExprYield {
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            attrs: Vec<Attribute>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            expr: Option<Box<Expr>>,
        }),

        /// Tokens in expression position not interpreted by Syn.
        pub Verbatim(ExprVerbatim {
            tts: TokenStream,
        }),
    }
}

ast_enum! {
    /// A struct or tuple struct field accessed in a struct literal or field
    /// expression.
    pub enum Member {
        /// A named field like `self.x`.
        #[serde(rename = "ident")]
        Named(Ident),
        /// An unnamed field like `self.0`.
        #[serde(rename = "index")]
        Unnamed(Index),
    }
}

ast_struct! {
    /// The index of an unnamed tuple struct field.
    #[serde(transparent)]
    pub struct Index {
        index: u32,
    }
}

impl From<usize> for Index {
    fn from(index: usize) -> Self {
        assert!(index < u32::max_value() as usize);
        Self {
            index: index as u32,
        }
    }
}

ast_struct! {
    /// The `::<>` explicit type parameters passed to a method call:
    /// `parse::<u64>()`.
    pub struct MethodTurbofish {
        args: Punctuated<GenericMethodArgument>,
    }
}

ast_enum! {
    /// An individual generic argument to a method, like `T`.
    pub enum GenericMethodArgument {
        /// A type argument.
        Type(Type),
        /// A const expression. Must be inside of a block.
        ///
        /// NOTE: Identity expressions are represented as Type arguments, as
        /// they are indistinguishable syntactically.
        Const(Expr),
    }
}

ast_struct! {
    /// A field-value pair in a struct literal.
    pub struct FieldValue {
        /// Attributes tagged on the field.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        attrs: Vec<Attribute>,

        /// Name or index of the field.
        #[serde(flatten)]
        member: Member,

        /// The colon in `Struct { x: x }`. If written in shorthand like
        /// `Struct { x }`, there is no colon.
        #[serde(default, skip_serializing_if = "not")]
        colon_token: bool,

        /// Value of the field.
        expr: Expr,
    }
}

ast_struct! {
    /// A lifetime labeling a `for`, `while`, or `loop`.
    #[serde(transparent)]
    pub struct Label {
        name: Lifetime,
    }
}

ast_struct! {
    /// A braced block containing Rust statements.
    pub struct Block {
        /// Statements in a block
        stmts: Vec<Stmt>,
    }
}

ast_enum! {
    /// A statement, usually ending in a semicolon.
    pub enum Stmt #manual_from_impl {
        /// A local (let) binding.
        #[serde(rename = "let")]
        Local(Local),

        /// An item definition.
        Item(Item),

        /// Expr without trailing semicolon.
        Expr(Expr),

        /// Expression with trailing semicolon.
        Semi(Expr),
    }
}

ast_struct! {
    /// A local `let` binding: `let x: u64 = s.parse()?`.
    pub struct Local {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        attrs: Vec<Attribute>,
        pats: Punctuated<Pat>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ty: Option<Box<Type>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        init: Option<Box<Expr>>,
    }
}

ast_enum_of_structs! {
    /// A pattern in a local binding, function signature, match expression, or
    /// various other places.
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum Pat #manual_from_impl {
        /// A pattern that matches any value: `_`.
        #[serde(rename = "_")]
        pub Wild,

        /// A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.
        pub Ident(PatIdent {
            #[serde(rename = "ref")]
            #[serde(default, skip_serializing_if = "not")]
            by_ref: bool,
            #[serde(rename = "mut")]
            #[serde(default, skip_serializing_if = "not")]
            mutability: bool,
            ident: Ident,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            subpat: Option<Box<Pat>>,
        }),

        /// A struct or struct variant pattern: `Variant { x, y, .. }`.
        pub Struct(PatStruct {
            path: Path,
            fields: Punctuated<FieldPat>,
            #[serde(default, skip_serializing_if = "not")]
            dot2_token: bool,
        }),

        /// A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.
        pub TupleStruct(PatTupleStruct {
            path: Path,
            #[serde(flatten)]
            pat: PatTuple,
        }),

        /// A path pattern like `Color::Red`, optionally qualified with a
        /// self-type.
        ///
        /// Unqualified path patterns can legally refer to variants, structs,
        /// constants or associated constants. Qualified path patterns like
        /// `<A>::B::C` and `<A as Trait>::B::C` can only legally refer to
        /// associated constants.
        pub Path(PatPath {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            qself: Option<QSelf>,
            #[serde(flatten)]
            path: Path,
        }),

        /// A tuple pattern: `(a, b)`.
        pub Tuple(PatTuple {
            #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
            front: Punctuated<Pat>,
            #[serde(default, skip_serializing_if = "not")]
            dot2_token: bool,
            #[serde(default, skip_serializing_if = "not")]
            comma_token: bool,
            #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
            back: Punctuated<Pat>,
        }),

        /// A box pattern: `box v`.
        pub Box(PatBox {
            pat: Box<Pat>,
        }),

        /// A reference pattern: `&mut (first, second)`.
        pub Ref(PatRef {
            #[serde(rename = "mut")]
            #[serde(default, skip_serializing_if = "not")]
            mutability: bool,
            pat: Box<Pat>,
        }),

        /// A literal pattern: `0`.
        ///
        /// This holds an `Expr` rather than a `Lit` because negative numbers
        /// are represented as an `Expr::Unary`.
        pub Lit(PatLit {
            expr: Box<Expr>,
        }),

        /// A range pattern: `1..=2`.
        pub Range(PatRange {
            lo: Box<Expr>,
            limits: RangeLimits,
            hi: Box<Expr>,
        }),

        /// A dynamically sized slice pattern: `[a, b, i.., y, z]`.
        pub Slice(PatSlice {
            #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
            front: Punctuated<Pat>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            middle: Option<Box<Pat>>,
            #[serde(default, skip_serializing_if = "not")]
            dot2_token: bool,
            #[serde(default, skip_serializing_if = "not")]
            comma_token: bool,
            #[serde(default, skip_serializing_if = "Punctuated::is_empty")]
            back: Punctuated<Pat>,
        }),

        /// A macro in expression position.
        pub Macro(PatMacro {
            #[serde(flatten)]
            mac: Macro,
        }),

        /// Tokens in pattern position not interpreted by Syn.
        pub Verbatim(PatVerbatim {
            tts: TokenStream,
        }),
    }
}

ast_struct! {
    /// One arm of a `match` expression: `0...10 => { return true; }`.
    ///
    /// As in:
    ///
    /// ```edition2018
    /// # fn f() -> bool {
    /// #     let n = 0;
    /// match n {
    ///     0...10 => {
    ///         return true;
    ///     }
    ///     // ...
    ///     # _ => {}
    /// }
    /// #   false
    /// # }
    /// ```
    pub struct Arm {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        attrs: Vec<Attribute>,
        #[serde(default, skip_serializing_if = "not")]
        leading_vert: bool,
        pats: Punctuated<Pat>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        guard: Option<Box<Expr>>,
        body: Box<Expr>,
        // #[serde(default, skip_serializing_if = "not")]
        // comma: bool,
    }
}

ast_enum! {
    /// Limit types of a range, inclusive or exclusive.
    pub enum RangeLimits #manual_from_impl {
        /// Inclusive at the beginning, exclusive at the end.
        #[serde(rename = "..")]
        HalfOpen,
        /// Inclusive at the beginning and end.
        #[serde(rename = "..=")]
        Closed,
    }
}

ast_struct! {
    /// A single field in a struct pattern.
    ///
    /// Patterns like the fields of Foo `{ x, ref y, ref mut z }` are treated
    /// the same as `x: x, y: ref y, z: ref mut z` but there is no colon token.
    pub struct FieldPat {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        attrs: Vec<Attribute>,
        #[serde(flatten)]
        member: Member,
        #[serde(default, skip_serializing_if = "not")]
        colon_token: bool,
        pat: Box<Pat>,
    }
}

fn requires_terminator(expr: &Expr) -> bool {
    // see https://github.com/rust-lang/rust/blob/eb8f2586e/src/libsyntax/parse/classify.rs#L17-L37
    match *expr {
        Expr::Unsafe(..)
        | Expr::Block(..)
        | Expr::If(..)
        | Expr::Match(..)
        | Expr::While(..)
        | Expr::Loop(..)
        | Expr::ForLoop(..)
        | Expr::Async(..)
        | Expr::TryBlock(..) => false,
        _ => true,
    }
}

mod convert {
    use super::*;

    // ExprBox

    impl From<&syn::ExprBox> for ExprBox {
        fn from(other: &syn::ExprBox) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.map_into(),
            }
        }
    }

    impl From<&ExprBox> for syn::ExprBox {
        fn from(other: &ExprBox) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                box_token: default(),
                expr: other.expr.map_into(),
            }
        }
    }

    // ExprInPlace

    impl From<&syn::ExprInPlace> for ExprInPlace {
        fn from(other: &syn::ExprInPlace) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                place: other.place.map_into(),
                value: other.value.map_into(),
            }
        }
    }

    impl From<&ExprInPlace> for syn::ExprInPlace {
        fn from(other: &ExprInPlace) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                place: other.place.map_into(),
                arrow_token: default(),
                value: other.value.map_into(),
            }
        }
    }

    // ExprArray

    impl From<&syn::ExprArray> for ExprArray {
        fn from(other: &syn::ExprArray) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                elems: other.elems.map_into(),
            }
        }
    }

    impl From<&ExprArray> for syn::ExprArray {
        fn from(other: &ExprArray) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                bracket_token: default(),
                elems: other.elems.map_into(),
            }
        }
    }

    // ExprCall

    impl From<&syn::ExprCall> for ExprCall {
        fn from(other: &syn::ExprCall) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                func: other.func.map_into(),
                args: other.args.map_into(),
            }
        }
    }

    impl From<&ExprCall> for syn::ExprCall {
        fn from(other: &ExprCall) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                func: other.func.map_into(),
                paren_token: default(),
                args: other.args.map_into(),
            }
        }
    }

    // ExprMethodCall

    impl From<&syn::ExprMethodCall> for ExprMethodCall {
        fn from(other: &syn::ExprMethodCall) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                receiver: other.receiver.map_into(),
                method: other.method.ref_into(),
                turbofish: other.turbofish.map_into(),
                args: other.args.map_into(),
            }
        }
    }

    impl From<&ExprMethodCall> for syn::ExprMethodCall {
        fn from(other: &ExprMethodCall) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                receiver: other.receiver.map_into(),
                dot_token: default(),
                method: other.method.ref_into(),
                turbofish: other.turbofish.map_into(),
                paren_token: default(),
                args: other.args.map_into(),
            }
        }
    }

    // ExprTuple

    impl From<&syn::ExprTuple> for ExprTuple {
        fn from(other: &syn::ExprTuple) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                elems: other.elems.map_into(),
            }
        }
    }

    impl From<&ExprTuple> for syn::ExprTuple {
        fn from(other: &ExprTuple) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                paren_token: default(),
                elems: other.elems.map_into(),
            }
        }
    }

    // ExprBinary

    impl From<&syn::ExprBinary> for ExprBinary {
        fn from(other: &syn::ExprBinary) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                left: other.left.map_into(),
                op: other.op.ref_into(),
                right: other.right.map_into(),
            }
        }
    }

    impl From<&ExprBinary> for syn::ExprBinary {
        fn from(other: &ExprBinary) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                left: other.left.map_into(),
                op: other.op.ref_into(),
                right: other.right.map_into(),
            }
        }
    }

    // ExprUnary

    impl From<&syn::ExprUnary> for ExprUnary {
        fn from(other: &syn::ExprUnary) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                op: other.op.ref_into(),
                expr: other.expr.map_into(),
            }
        }
    }

    impl From<&ExprUnary> for syn::ExprUnary {
        fn from(other: &ExprUnary) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                op: other.op.ref_into(),
                expr: other.expr.map_into(),
            }
        }
    }

    // ExprLit

    impl From<&syn::ExprLit> for ExprLit {
        fn from(other: &syn::ExprLit) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                lit: other.lit.ref_into(),
            }
        }
    }

    impl From<&ExprLit> for syn::ExprLit {
        fn from(other: &ExprLit) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                lit: other.lit.ref_into(),
            }
        }
    }

    // ExprCast

    impl From<&syn::ExprCast> for ExprCast {
        fn from(other: &syn::ExprCast) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.map_into(),
                ty: other.ty.map_into(),
            }
        }
    }

    impl From<&ExprCast> for syn::ExprCast {
        fn from(other: &ExprCast) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.map_into(),
                as_token: default(),
                ty: other.ty.map_into(),
            }
        }
    }

    // ExprType

    impl From<&syn::ExprType> for ExprType {
        fn from(other: &syn::ExprType) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.map_into(),
                ty: other.ty.map_into(),
            }
        }
    }

    impl From<&ExprType> for syn::ExprType {
        fn from(other: &ExprType) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.map_into(),
                colon_token: default(),
                ty: other.ty.map_into(),
            }
        }
    }

    // ExprLet

    impl From<&syn::ExprLet> for ExprLet {
        fn from(other: &syn::ExprLet) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                pats: other.pats.map_into(),
                expr: other.expr.map_into(),
            }
        }
    }

    impl From<&ExprLet> for syn::ExprLet {
        fn from(other: &ExprLet) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                let_token: default(),
                pats: other.pats.map_into(),
                eq_token: default(),
                expr: other.expr.map_into(),
            }
        }
    }

    // ExprIf

    impl From<&syn::ExprIf> for ExprIf {
        fn from(other: &syn::ExprIf) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                cond: other.cond.map_into(),
                then_branch: other.then_branch.ref_into(),
                else_branch: other.else_branch.ref_map(|(_, x)| x.map_into()),
            }
        }
    }

    impl From<&ExprIf> for syn::ExprIf {
        fn from(other: &ExprIf) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                if_token: default(),
                cond: other.cond.map_into(),
                then_branch: other.then_branch.ref_into(),
                else_branch: other.else_branch.ref_map(|x| (default(), x.map_into())),
            }
        }
    }

    // ExprWhile

    impl From<&syn::ExprWhile> for ExprWhile {
        fn from(other: &syn::ExprWhile) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                label: other.label.map_into(),
                cond: other.cond.map_into(),
                body: other.body.ref_into(),
            }
        }
    }

    impl From<&ExprWhile> for syn::ExprWhile {
        fn from(other: &ExprWhile) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                label: other.label.map_into(),
                while_token: default(),
                cond: other.cond.map_into(),
                body: other.body.ref_into(),
            }
        }
    }

    // ExprForLoop

    impl From<&syn::ExprForLoop> for ExprForLoop {
        fn from(other: &syn::ExprForLoop) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                label: other.label.map_into(),
                pat: other.pat.map_into(),
                expr: other.expr.map_into(),
                body: other.body.ref_into(),
            }
        }
    }

    impl From<&ExprForLoop> for syn::ExprForLoop {
        fn from(other: &ExprForLoop) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                label: other.label.map_into(),
                for_token: default(),
                pat: other.pat.map_into(),
                in_token: default(),
                expr: other.expr.map_into(),
                body: other.body.ref_into(),
            }
        }
    }

    // ExprLoop

    impl From<&syn::ExprLoop> for ExprLoop {
        fn from(other: &syn::ExprLoop) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                label: other.label.map_into(),
                body: other.body.ref_into(),
            }
        }
    }

    impl From<&ExprLoop> for syn::ExprLoop {
        fn from(other: &ExprLoop) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                label: other.label.map_into(),
                loop_token: default(),
                body: other.body.ref_into(),
            }
        }
    }

    // ExprMatch

    impl From<&syn::ExprMatch> for ExprMatch {
        fn from(other: &syn::ExprMatch) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.map_into(),
                arms: from_syn_arms(&other.arms),
            }
        }
    }

    fn from_syn_arms(other: &[syn::Arm]) -> Vec<Arm> {
        let last = other.len().saturating_sub(1);
        other
            .iter()
            .enumerate()
            .map(|(i, other)| {
                let body = other.body.map_into();
                if i < last && requires_terminator(&*body) {
                    assert!(other.comma.is_some(), "expected `,`");
                }

                Arm {
                    attrs: other.attrs.map_into(),
                    leading_vert: other.leading_vert.is_some(),
                    pats: other.pats.map_into(),
                    guard: other.guard.ref_map(|(_, x)| x.map_into()),
                    body,
                }
            })
            .collect()
    }

    impl From<&ExprMatch> for syn::ExprMatch {
        fn from(other: &ExprMatch) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                match_token: default(),
                expr: other.expr.map_into(),
                brace_token: default(),
                arms: other.arms.map_into(),
            }
        }
    }

    // ExprClosure

    impl From<&syn::ExprClosure> for ExprClosure {
        fn from(other: &syn::ExprClosure) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                asyncness: other.asyncness.is_some(),
                movability: other.movability.is_some(),
                capture: other.capture.is_some(),
                inputs: other.inputs.map_into(),
                output: other.output.ref_into(),
                body: other.body.map_into(),
            }
        }
    }

    impl From<&ExprClosure> for syn::ExprClosure {
        fn from(other: &ExprClosure) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                asyncness: default_or_none(other.asyncness),
                movability: default_or_none(other.movability),
                capture: default_or_none(other.capture),
                or1_token: default(),
                inputs: other.inputs.map_into(),
                or2_token: default(),
                output: other.output.ref_into(),
                body: other.body.map_into(),
            }
        }
    }

    // ExprUnsafe

    impl From<&syn::ExprUnsafe> for ExprUnsafe {
        fn from(other: &syn::ExprUnsafe) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                block: other.block.ref_into(),
            }
        }
    }

    impl From<&ExprUnsafe> for syn::ExprUnsafe {
        fn from(other: &ExprUnsafe) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                unsafe_token: default(),
                block: other.block.ref_into(),
            }
        }
    }

    // ExprBlock

    impl From<&syn::ExprBlock> for ExprBlock {
        fn from(other: &syn::ExprBlock) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                label: other.label.map_into(),
                block: other.block.ref_into(),
            }
        }
    }

    impl From<&ExprBlock> for syn::ExprBlock {
        fn from(other: &ExprBlock) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                label: other.label.map_into(),
                block: other.block.ref_into(),
            }
        }
    }

    // ExprAssign

    impl From<&syn::ExprAssign> for ExprAssign {
        fn from(other: &syn::ExprAssign) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                left: other.left.map_into(),
                right: other.right.map_into(),
            }
        }
    }

    impl From<&ExprAssign> for syn::ExprAssign {
        fn from(other: &ExprAssign) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                left: other.left.map_into(),
                eq_token: default(),
                right: other.right.map_into(),
            }
        }
    }

    // ExprAssignOp

    impl From<&syn::ExprAssignOp> for ExprAssignOp {
        fn from(other: &syn::ExprAssignOp) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                left: other.left.map_into(),
                op: other.op.ref_into(),
                right: other.right.map_into(),
            }
        }
    }

    impl From<&ExprAssignOp> for syn::ExprAssignOp {
        fn from(other: &ExprAssignOp) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                left: other.left.map_into(),
                op: other.op.ref_into(),
                right: other.right.map_into(),
            }
        }
    }

    // ExprField

    impl From<&syn::ExprField> for ExprField {
        fn from(other: &syn::ExprField) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                base: other.base.map_into(),
                member: other.member.ref_into(),
            }
        }
    }

    impl From<&ExprField> for syn::ExprField {
        fn from(other: &ExprField) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                base: other.base.map_into(),
                dot_token: default(),
                member: other.member.ref_into(),
            }
        }
    }

    // ExprIndex

    impl From<&syn::ExprIndex> for ExprIndex {
        fn from(other: &syn::ExprIndex) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.map_into(),
                index: other.index.map_into(),
            }
        }
    }

    impl From<&ExprIndex> for syn::ExprIndex {
        fn from(other: &ExprIndex) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.map_into(),
                bracket_token: default(),
                index: other.index.map_into(),
            }
        }
    }

    // ExprRange

    impl From<&syn::ExprRange> for ExprRange {
        fn from(other: &syn::ExprRange) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                from: other.from.ref_map(MapInto::map_into),
                limits: other.limits.ref_into(),
                to: other.to.ref_map(MapInto::map_into),
            }
        }
    }

    impl From<&ExprRange> for syn::ExprRange {
        fn from(other: &ExprRange) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                from: other.from.ref_map(MapInto::map_into),
                limits: other.limits.ref_into(),
                to: other.to.ref_map(MapInto::map_into),
            }
        }
    }

    // ExprPath

    impl From<&syn::ExprPath> for ExprPath {
        fn from(other: &syn::ExprPath) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                qself: other.qself.map_into(),
                path: other.path.ref_into(),
            }
        }
    }

    impl From<&ExprPath> for syn::ExprPath {
        fn from(other: &ExprPath) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                qself: other.qself.map_into(),
                path: other.path.ref_into(),
            }
        }
    }

    // ExprReference

    impl From<&syn::ExprReference> for ExprReference {
        fn from(other: &syn::ExprReference) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                mutability: other.mutability.is_some(),
                expr: other.expr.map_into(),
            }
        }
    }

    impl From<&ExprReference> for syn::ExprReference {
        fn from(other: &ExprReference) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                and_token: default(),
                mutability: default_or_none(other.mutability),
                expr: other.expr.map_into(),
            }
        }
    }

    // ExprBreak

    impl From<&syn::ExprBreak> for ExprBreak {
        fn from(other: &syn::ExprBreak) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                label: other.label.map_into(),
                expr: other.expr.ref_map(MapInto::map_into),
            }
        }
    }

    impl From<&ExprBreak> for syn::ExprBreak {
        fn from(other: &ExprBreak) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                break_token: default(),
                label: other.label.map_into(),
                expr: other.expr.ref_map(MapInto::map_into),
            }
        }
    }

    // ExprContinue

    impl From<&syn::ExprContinue> for ExprContinue {
        fn from(other: &syn::ExprContinue) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                label: other.label.map_into(),
            }
        }
    }

    impl From<&ExprContinue> for syn::ExprContinue {
        fn from(other: &ExprContinue) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                continue_token: default(),
                label: other.label.map_into(),
            }
        }
    }

    // ExprReturn

    impl From<&syn::ExprReturn> for ExprReturn {
        fn from(other: &syn::ExprReturn) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.ref_map(MapInto::map_into),
            }
        }
    }

    impl From<&ExprReturn> for syn::ExprReturn {
        fn from(other: &ExprReturn) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                return_token: default(),
                expr: other.expr.ref_map(MapInto::map_into),
            }
        }
    }

    // ExprMacro

    impl From<&syn::ExprMacro> for ExprMacro {
        fn from(other: &syn::ExprMacro) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                mac: other.mac.ref_into(),
            }
        }
    }

    impl From<&ExprMacro> for syn::ExprMacro {
        fn from(other: &ExprMacro) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                mac: other.mac.ref_into(),
            }
        }
    }

    // ExprStruct

    impl From<&syn::ExprStruct> for ExprStruct {
        fn from(other: &syn::ExprStruct) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                path: other.path.ref_into(),
                fields: other.fields.map_into(),
                dot2_token: other.dot2_token.is_some(),
                rest: other.rest.ref_map(MapInto::map_into),
            }
        }
    }

    impl From<&ExprStruct> for syn::ExprStruct {
        fn from(other: &ExprStruct) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                path: other.path.ref_into(),
                brace_token: default(),
                fields: other.fields.map_into(),
                dot2_token: default_or_none(other.dot2_token),
                rest: other.rest.ref_map(MapInto::map_into),
            }
        }
    }

    // ExprRepeat

    impl From<&syn::ExprRepeat> for ExprRepeat {
        fn from(other: &syn::ExprRepeat) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.map_into(),
                len: other.len.map_into(),
            }
        }
    }

    impl From<&ExprRepeat> for syn::ExprRepeat {
        fn from(other: &ExprRepeat) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                bracket_token: default(),
                expr: other.expr.map_into(),
                semi_token: default(),
                len: other.len.map_into(),
            }
        }
    }

    // ExprParen

    impl From<&syn::ExprParen> for ExprParen {
        fn from(other: &syn::ExprParen) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.map_into(),
            }
        }
    }

    impl From<&ExprParen> for syn::ExprParen {
        fn from(other: &ExprParen) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                paren_token: default(),
                expr: other.expr.map_into(),
            }
        }
    }

    // ExprGroup

    impl From<&syn::ExprGroup> for ExprGroup {
        fn from(other: &syn::ExprGroup) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.map_into(),
            }
        }
    }

    impl From<&ExprGroup> for syn::ExprGroup {
        fn from(other: &ExprGroup) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                group_token: default(),
                expr: other.expr.map_into(),
            }
        }
    }

    // ExprTry

    impl From<&syn::ExprTry> for ExprTry {
        fn from(other: &syn::ExprTry) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.map_into(),
            }
        }
    }

    impl From<&ExprTry> for syn::ExprTry {
        fn from(other: &ExprTry) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.map_into(),
                question_token: default(),
            }
        }
    }

    // ExprAsync

    impl From<&syn::ExprAsync> for ExprAsync {
        fn from(other: &syn::ExprAsync) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                capture: other.capture.is_some(),
                block: other.block.ref_into(),
            }
        }
    }

    impl From<&ExprAsync> for syn::ExprAsync {
        fn from(other: &ExprAsync) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                async_token: default(),
                capture: default_or_none(other.capture),
                block: other.block.ref_into(),
            }
        }
    }

    // ExprTryBlock

    impl From<&syn::ExprTryBlock> for ExprTryBlock {
        fn from(other: &syn::ExprTryBlock) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                block: other.block.ref_into(),
            }
        }
    }

    impl From<&ExprTryBlock> for syn::ExprTryBlock {
        fn from(other: &ExprTryBlock) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                try_token: default(),
                block: other.block.ref_into(),
            }
        }
    }

    // ExprYield

    impl From<&syn::ExprYield> for ExprYield {
        fn from(other: &syn::ExprYield) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                expr: other.expr.ref_map(MapInto::map_into),
            }
        }
    }

    impl From<&ExprYield> for syn::ExprYield {
        fn from(other: &ExprYield) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                yield_token: default(),
                expr: other.expr.ref_map(MapInto::map_into),
            }
        }
    }

    // ExprVerbatim

    impl From<&syn::ExprVerbatim> for ExprVerbatim {
        fn from(other: &syn::ExprVerbatim) -> Self {
            Self {
                tts: other.tts.ref_into(),
            }
        }
    }

    impl From<&ExprVerbatim> for syn::ExprVerbatim {
        fn from(other: &ExprVerbatim) -> Self {
            Self {
                tts: other.tts.ref_into(),
            }
        }
    }

    // Index

    impl From<&syn::Index> for Index {
        fn from(other: &syn::Index) -> Self {
            Self { index: other.index }
        }
    }

    impl From<&Index> for syn::Index {
        fn from(other: &Index) -> Self {
            Self {
                index: other.index,
                span: Span::call_site(),
            }
        }
    }

    // MethodTurbofish

    impl From<&syn::MethodTurbofish> for MethodTurbofish {
        fn from(other: &syn::MethodTurbofish) -> Self {
            Self {
                args: other.args.map_into(),
            }
        }
    }

    impl From<&MethodTurbofish> for syn::MethodTurbofish {
        fn from(other: &MethodTurbofish) -> Self {
            Self {
                colon2_token: default(),
                lt_token: default(),
                args: other.args.map_into(),
                gt_token: default(),
            }
        }
    }

    // FieldValue

    impl From<&syn::FieldValue> for FieldValue {
        fn from(other: &syn::FieldValue) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                member: other.member.ref_into(),
                colon_token: other.colon_token.is_some(),
                expr: other.expr.ref_into(),
            }
        }
    }

    impl From<&FieldValue> for syn::FieldValue {
        fn from(other: &FieldValue) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                member: other.member.ref_into(),
                colon_token: default_or_none(other.colon_token),
                expr: other.expr.ref_into(),
            }
        }
    }

    // Label

    impl From<&syn::Label> for Label {
        fn from(other: &syn::Label) -> Self {
            Self {
                name: other.name.ref_into(),
            }
        }
    }

    impl From<&Label> for syn::Label {
        fn from(other: &Label) -> Self {
            Self {
                name: other.name.ref_into(),
                colon_token: default(),
            }
        }
    }

    // Block

    impl From<&syn::Block> for Block {
        fn from(other: &syn::Block) -> Self {
            Self {
                stmts: other.stmts.map_into(),
            }
        }
    }

    impl From<&Block> for syn::Block {
        fn from(other: &Block) -> Self {
            Self {
                brace_token: default(),
                stmts: other.stmts.map_into(),
            }
        }
    }

    // Stmt

    impl From<&syn::Stmt> for Stmt {
        fn from(other: &syn::Stmt) -> Self {
            use super::Stmt::*;
            use syn::Stmt;
            match other {
                Stmt::Local(x) => Local(x.ref_into()),
                Stmt::Item(x) => Item(x.ref_into()),
                Stmt::Expr(x) => Expr(x.ref_into()),
                Stmt::Semi(x, _) => Semi(x.ref_into()),
            }
        }
    }

    impl From<&Stmt> for syn::Stmt {
        fn from(other: &Stmt) -> Self {
            use syn::Stmt::*;
            match other {
                Stmt::Local(x) => Local(x.into()),
                Stmt::Item(x) => Item(x.into()),
                Stmt::Expr(x) => Expr(x.into()),
                Stmt::Semi(x) => Semi(x.into(), default()),
            }
        }
    }

    // Local

    impl From<&syn::Local> for Local {
        fn from(other: &syn::Local) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                pats: other.pats.map_into(),
                ty: other.ty.ref_map(|(_, x)| x.map_into()),
                init: other.init.ref_map(|(_, x)| x.map_into()),
            }
        }
    }

    impl From<&Local> for syn::Local {
        fn from(other: &Local) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                let_token: default(),
                pats: other.pats.map_into(),
                ty: other.ty.ref_map(|x| (default(), x.map_into())),
                init: other.init.ref_map(|x| (default(), x.map_into())),
                semi_token: default(),
            }
        }
    }

    // Pat

    impl From<&syn::Pat> for Pat {
        fn from(other: &syn::Pat) -> Self {
            use super::Pat::*;
            use syn::Pat;
            match other {
                Pat::Wild(_) => Wild,
                Pat::Ident(x) => Ident(x.ref_into()),
                Pat::Struct(x) => Struct(x.ref_into()),
                Pat::TupleStruct(x) => TupleStruct(x.ref_into()),
                Pat::Path(x) => Path(x.ref_into()),
                Pat::Tuple(x) => Tuple(x.ref_into()),
                Pat::Box(x) => Box(x.ref_into()),
                Pat::Ref(x) => Ref(x.ref_into()),
                Pat::Lit(x) => Lit(x.ref_into()),
                Pat::Range(x) => Range(x.ref_into()),
                Pat::Slice(x) => Slice(x.ref_into()),
                Pat::Macro(x) => Macro(x.ref_into()),
                Pat::Verbatim(x) => Verbatim(x.ref_into()),
            }
        }
    }

    impl From<&Pat> for syn::Pat {
        fn from(other: &Pat) -> Self {
            use syn::Pat::*;
            match other {
                Pat::Wild => Wild(syn::PatWild {
                    underscore_token: default(),
                }),
                Pat::Ident(x) => Ident(x.into()),
                Pat::Struct(x) => Struct(x.into()),
                Pat::TupleStruct(x) => TupleStruct(x.into()),
                Pat::Path(x) => Path(x.into()),
                Pat::Tuple(x) => Tuple(x.into()),
                Pat::Box(x) => Box(x.into()),
                Pat::Ref(x) => Ref(x.into()),
                Pat::Lit(x) => Lit(x.into()),
                Pat::Range(x) => Range(x.into()),
                Pat::Slice(x) => Slice(x.into()),
                Pat::Macro(x) => Macro(x.into()),
                Pat::Verbatim(x) => Verbatim(x.into()),
            }
        }
    }

    // PatIdent

    impl From<&syn::PatIdent> for PatIdent {
        fn from(other: &syn::PatIdent) -> Self {
            Self {
                by_ref: other.by_ref.is_some(),
                mutability: other.mutability.is_some(),
                ident: other.ident.ref_into(),
                subpat: other.subpat.ref_map(|(_, x)| x.map_into()),
            }
        }
    }

    impl From<&PatIdent> for syn::PatIdent {
        fn from(other: &PatIdent) -> Self {
            Self {
                by_ref: default_or_none(other.by_ref),
                mutability: default_or_none(other.mutability),
                ident: other.ident.ref_into(),
                subpat: other.subpat.ref_map(|x| (default(), x.map_into())),
            }
        }
    }

    // PatStruct

    impl From<&syn::PatStruct> for PatStruct {
        fn from(other: &syn::PatStruct) -> Self {
            Self {
                path: other.path.ref_into(),
                fields: other.fields.map_into(),
                dot2_token: other.dot2_token.is_some(),
            }
        }
    }

    impl From<&PatStruct> for syn::PatStruct {
        fn from(other: &PatStruct) -> Self {
            Self {
                path: other.path.ref_into(),
                brace_token: default(),
                fields: other.fields.map_into(),
                dot2_token: default_or_none(other.dot2_token),
            }
        }
    }

    // PatTupleStruct

    impl From<&syn::PatTupleStruct> for PatTupleStruct {
        fn from(other: &syn::PatTupleStruct) -> Self {
            Self {
                path: other.path.ref_into(),
                pat: other.pat.ref_into(),
            }
        }
    }

    impl From<&PatTupleStruct> for syn::PatTupleStruct {
        fn from(other: &PatTupleStruct) -> Self {
            Self {
                path: other.path.ref_into(),
                pat: other.pat.ref_into(),
            }
        }
    }

    // PatPath

    impl From<&syn::PatPath> for PatPath {
        fn from(other: &syn::PatPath) -> Self {
            Self {
                qself: other.qself.map_into(),
                path: other.path.ref_into(),
            }
        }
    }

    impl From<&PatPath> for syn::PatPath {
        fn from(other: &PatPath) -> Self {
            Self {
                qself: other.qself.map_into(),
                path: other.path.ref_into(),
            }
        }
    }

    // PatTuple

    impl From<&syn::PatTuple> for PatTuple {
        fn from(other: &syn::PatTuple) -> Self {
            Self {
                front: other.front.map_into(),
                dot2_token: other.dot2_token.is_some(),
                comma_token: other.comma_token.is_some(),
                back: other.back.map_into(),
            }
        }
    }

    impl From<&PatTuple> for syn::PatTuple {
        fn from(other: &PatTuple) -> Self {
            Self {
                paren_token: default(),
                front: other.front.map_into(),
                dot2_token: default_or_none(other.dot2_token),
                comma_token: default_or_none(other.comma_token),
                back: other.back.map_into(),
            }
        }
    }

    // PatBox

    impl From<&syn::PatBox> for PatBox {
        fn from(other: &syn::PatBox) -> Self {
            Self {
                pat: other.pat.map_into(),
            }
        }
    }

    impl From<&PatBox> for syn::PatBox {
        fn from(other: &PatBox) -> Self {
            Self {
                box_token: default(),
                pat: other.pat.map_into(),
            }
        }
    }

    // PatRef

    impl From<&syn::PatRef> for PatRef {
        fn from(other: &syn::PatRef) -> Self {
            Self {
                mutability: other.mutability.is_some(),
                pat: other.pat.map_into(),
            }
        }
    }

    impl From<&PatRef> for syn::PatRef {
        fn from(other: &PatRef) -> Self {
            Self {
                and_token: default(),
                mutability: default_or_none(other.mutability),
                pat: other.pat.map_into(),
            }
        }
    }

    // PatLit

    impl From<&syn::PatLit> for PatLit {
        fn from(other: &syn::PatLit) -> Self {
            Self {
                expr: other.expr.map_into(),
            }
        }
    }

    impl From<&PatLit> for syn::PatLit {
        fn from(other: &PatLit) -> Self {
            Self {
                expr: other.expr.map_into(),
            }
        }
    }

    // PatRange

    impl From<&syn::PatRange> for PatRange {
        fn from(other: &syn::PatRange) -> Self {
            Self {
                lo: other.lo.map_into(),
                limits: other.limits.ref_into(),
                hi: other.hi.map_into(),
            }
        }
    }

    impl From<&PatRange> for syn::PatRange {
        fn from(other: &PatRange) -> Self {
            Self {
                lo: other.lo.map_into(),
                limits: other.limits.ref_into(),
                hi: other.hi.map_into(),
            }
        }
    }

    // PatSlice

    impl From<&syn::PatSlice> for PatSlice {
        fn from(other: &syn::PatSlice) -> Self {
            Self {
                front: other.front.map_into(),
                middle: other.middle.ref_map(MapInto::map_into),
                dot2_token: other.dot2_token.is_some(),
                comma_token: other.comma_token.is_some(),
                back: other.back.map_into(),
            }
        }
    }

    impl From<&PatSlice> for syn::PatSlice {
        fn from(other: &PatSlice) -> Self {
            Self {
                bracket_token: default(),
                front: other.front.map_into(),
                middle: other.middle.ref_map(MapInto::map_into),
                dot2_token: default_or_none(other.dot2_token),
                comma_token: default_or_none(other.comma_token),
                back: other.back.map_into(),
            }
        }
    }

    // PatMacro

    impl From<&syn::PatMacro> for PatMacro {
        fn from(other: &syn::PatMacro) -> Self {
            Self {
                mac: other.mac.ref_into(),
            }
        }
    }

    impl From<&PatMacro> for syn::PatMacro {
        fn from(other: &PatMacro) -> Self {
            Self {
                mac: other.mac.ref_into(),
            }
        }
    }

    // PatVerbatim

    impl From<&syn::PatVerbatim> for PatVerbatim {
        fn from(other: &syn::PatVerbatim) -> Self {
            Self {
                tts: other.tts.ref_into(),
            }
        }
    }

    impl From<&PatVerbatim> for syn::PatVerbatim {
        fn from(other: &PatVerbatim) -> Self {
            Self {
                tts: other.tts.ref_into(),
            }
        }
    }

    // Arm

    impl From<&syn::Arm> for Arm {
        fn from(other: &syn::Arm) -> Self {
            let body = other.body.map_into();
            if requires_terminator(&*body) {
                assert!(other.comma.is_some(), "expected `,`");
            }

            Self {
                attrs: other.attrs.map_into(),
                leading_vert: other.leading_vert.is_some(),
                pats: other.pats.map_into(),
                guard: other.guard.ref_map(|(_, x)| x.map_into()),
                body,
            }
        }
    }

    impl From<&Arm> for syn::Arm {
        fn from(other: &Arm) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                leading_vert: default_or_none(other.leading_vert),
                pats: other.pats.map_into(),
                guard: other.guard.ref_map(|x| (default(), x.map_into())),
                fat_arrow_token: default(),
                body: other.body.map_into(),
                comma: default_or_none(requires_terminator(&*other.body)),
            }
        }
    }

    // RangeLimits

    impl From<&syn::RangeLimits> for RangeLimits {
        fn from(other: &syn::RangeLimits) -> Self {
            use super::RangeLimits::*;
            use syn::RangeLimits;
            match other {
                RangeLimits::HalfOpen(_) => HalfOpen,
                RangeLimits::Closed(_) => Closed,
            }
        }
    }

    impl From<&RangeLimits> for syn::RangeLimits {
        fn from(other: &RangeLimits) -> Self {
            use syn::RangeLimits::*;
            match other {
                RangeLimits::HalfOpen => HalfOpen(default()),
                RangeLimits::Closed => Closed(default()),
            }
        }
    }

    // FieldPat

    impl From<&syn::FieldPat> for FieldPat {
        fn from(other: &syn::FieldPat) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                member: other.member.ref_into(),
                colon_token: other.colon_token.is_some(),
                pat: other.pat.map_into(),
            }
        }
    }

    impl From<&FieldPat> for syn::FieldPat {
        fn from(other: &FieldPat) -> Self {
            Self {
                attrs: other.attrs.map_into(),
                member: other.member.ref_into(),
                colon_token: default_or_none(other.colon_token),
                pat: other.pat.map_into(),
            }
        }
    }
}
