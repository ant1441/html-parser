use std::fs;

macro_rules! testdata_tests {
    ($($name:ident)*) => {
    $(
        #[test]
        fn $name() {
            let mut f = fs::File::open(concat!("./tests/testdata/", stringify!($name), ".html")).unwrap();
            let mut tokenizer = html_parser::Tokenizer::new(&mut f);
            tokenizer.run();
        }
    )*
    }
}

testdata_tests! {
    simple0
    simple1
    serenity_welcome
    twitter
}
