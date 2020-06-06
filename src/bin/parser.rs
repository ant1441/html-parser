use std::{env, fs, io};

extern crate pretty_env_logger;

fn main() -> io::Result<()> {
    pretty_env_logger::init();

    let input = env::args().nth(1).unwrap();
    let mut f = fs::File::open(input)?;

    let mut parser = html_parser::Parser::new(&mut f);
    parser.run();
    Ok(())
}
