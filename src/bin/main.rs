use std::io;
use std::fs::File;

extern crate pretty_env_logger;

fn main() -> io::Result<()> {
    pretty_env_logger::init();
    let mut file = File::open("./test/testdata/simple.html")?;

    let mut tokenizer = html_parser::Tokenizer::new(&mut file);
    tokenizer.run();
    Ok(())
}
