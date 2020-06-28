use std::{error, fmt, result};

use auto_enums::enum_derive;
use derive_more::From;

use crate::tokenizer::States;

mod parse_error;

pub use parse_error::ParseError;

pub type Result<T> = result::Result<T, Error>;

#[enum_derive(Error, From)]
pub enum Error {
    Utf8(std::str::Utf8Error),
    IO(std::io::Error),
    CellBorrowMut(std::cell::BorrowMutError),
    CellBorrow(std::cell::BorrowError),

    StateTransition(StateTransitionError),
    Parse(ParseError),
}

#[derive(Debug)]
pub struct StateTransitionError(States, &'static str);

impl StateTransitionError {
    pub(super) fn new(state: States, transition: &'static str) -> Self {
        StateTransitionError(state, transition)
    }
}

impl error::Error for StateTransitionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for StateTransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StateTransitionError: '{}' does not support transition '{}'",
            self.0, self.1
        )
    }
}
