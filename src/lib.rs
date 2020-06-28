#![allow(dead_code)]
#![warn(missing_debug_implementations, rust_2018_idioms)]
#![deny(unused_import_braces)]

pub mod dom;
pub mod parser;
pub mod tokenizer;

pub use parser::Parser;
pub use tokenizer::Tokenizer;

// NOTES
/*


#[derive(
    AsRef,
    Clone,
    Constructor,
    Copy,
    Debug,
    Default,
    Deref,
    DerefMut,
    Deserialize,
    Display,
    Eq,
    From,
    Hash,
    PartialEq,
    Serialize
)]

*/
