use serde_syn::Ident;

fn new(s: &str) -> Ident {
    Ident::new(s)
}

#[test]
fn ident_new() {
    new("String");
}

#[test]
fn ident_new_keyword() {
    new("abstract");
}

#[test]
#[should_panic(expected = "use Option<Ident>")]
fn ident_new_empty() {
    new("");
}

#[test]
#[should_panic(expected = "not a valid Ident")]
fn ident_new_lifetime() {
    new("'static");
}

#[test]
fn ident_new_underscore() {
    new("_");
}

#[test]
#[should_panic(expected = "use Literal instead")]
fn ident_new_number() {
    new("255");
}

#[test]
#[should_panic(expected = "\"a#\" is not a valid Ident")]
fn ident_new_invalid() {
    new("a#");
}
