//! Transit result is the outcome of a transition
//! It has a 'next_state', an optional error, and an array of things to be emitted

use std::cell::Cell;

use super::{Emit, ParseError, Error, Result, States, Token};

pub struct TransitionResult {
    state: Result<States>,
    emit: Cell<Emit>,
}

impl<E> From<E> for TransitionResult
where
    E: Into<Error>,
{
    fn from(e: E) -> Self {
        TransitionResult::from_result(Err(e.into()))
    }
}

impl<E> From<::std::result::Result<States, E>> for TransitionResult
where
    E: Into<Error>,
{
    fn from(res: ::std::result::Result<States, E>) -> Self {
        TransitionResult::from_result(res.map_err(|e| e.into()))
    }
}

impl From<States> for TransitionResult {
    fn from(state: States) -> Self {
        TransitionResult::from_state(state)
    }
}

impl TransitionResult {
    pub fn from_state(state: States) -> Self {
        TransitionResult::from_result(Ok(state))
    }

    fn from_result(res: Result<States>) -> Self {
        TransitionResult {
            state: res,
            emit: Cell::new(vec![]),
        }
    }

    pub fn is_err(&self) -> bool {
        self.state.is_err()
    }

    pub fn is_ok(&self) -> bool {
        self.state.is_ok()
    }

    pub fn state(self) -> Result<States> {
        self.state
    }

    pub fn emits(&mut self) -> Emit {
        self.emit.replace(Vec::new())
    }

    pub fn push_emit(&mut self, token: Token) {
        let mut emits = self.emit.take();
        emits.push(token);
        self.emit.replace(emits);
    }

    pub fn insert_emit(&mut self, index: usize, token: Token) {
        let mut emits = self.emit.take();
        emits.insert(index, token);
        self.emit.replace(emits);
    }

    pub fn push_parse_error(&mut self, err: ParseError) {
        println!("Parse Error: {}", err);
    }

    // TODO Parse errors should be an enum, not a str
    pub fn insert_parse_error(&mut self, _index: usize, err: ParseError) {
        println!("Parse Error: {}", err);
    }
}
