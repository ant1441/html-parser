use std::fs;

macro_rules! testdata_tests {
    ($($name:ident)*) => {
    $(
        #[test]
        fn $name() {
            let mut f = fs::File::open(concat!("./tests/testdata/", stringify!($name), ".html")).unwrap();
            let mut tokenizer = html_parser::Tokenizer::new(&mut f, true);
            tokenizer.run();
        }
    )*
    }
}

testdata_tests! {
    charref
    google
    serenity_welcome
    simple0
    simple1
    simple2
    simple3
    simple4
    simple5
    simple6
    twitter
}
