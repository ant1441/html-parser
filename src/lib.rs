#![warn(missing_debug_implementations, rust_2018_idioms, clippy::redundant_closure_for_method_calls)]
#![deny(unused_import_braces, clippy::unseparated_literal_suffix, clippy::default_trait_access, clippy::blacklisted_name)]
// Temporary...
#![allow(dead_code)]

// Too noisy for regular use, but useful for refactoring
// #![warn(clippy::pedantic)]

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
