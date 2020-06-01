use std::{error, fmt};

use auto_enums::enum_derive;
use derive_more::From;

mod parse_error;
mod transition_result;

use super::{Emit, States, Token};
pub use transition_result::TransitionResult;
pub use parse_error::ParseError;

pub type Result<T> = std::result::Result<T, Error>;

#[enum_derive(Error, From)]
pub enum Error {
    Utf8Error(std::str::Utf8Error),
    IOError(std::io::Error),
    CellBorrowMutError(std::cell::BorrowMutError),
    CellBorrowError(std::cell::BorrowError),

    StateTransitionError(StateTransitionError),
    ParseError(ParseError),
}

#[derive(Debug)]
pub struct StateTransitionError(States, &'static str);

impl StateTransitionError {
    pub fn new(state: States, transition: &'static str) -> Self {
        StateTransitionError(state, transition)
    }
}

impl error::Error for StateTransitionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for StateTransitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StateTransitionError: '{}' does not support transition '{}'",
            self.0, self.1
        )
    }
}
