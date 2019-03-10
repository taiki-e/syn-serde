#[macro_export]
macro_rules! punctuated {
    ($($e:expr,)+) => {{
        let mut seq = ::syn::punctuated::Punctuated::new();
        $(
            seq.push($e);
        )+
        seq
    }};

    ($($e:expr),+) => {
        punctuated!($($e,)+)
    };
}
