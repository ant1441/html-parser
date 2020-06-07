//! Transit result is the outcome of a transition
//! It has a 'next_state', an optional error, and an array of things to be emitted

use std::cell::Cell;

use super::{
    errors::{Error, ParseError, Result},
    Emit, States, Token,
};

pub(crate) struct TransitionResult {
    state: Result<States>,
    reconsume: bool,
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
    pub(super) fn from_state(state: States) -> Self {
        TransitionResult::from_result(Ok(state))
    }

    fn from_result(res: Result<States>) -> Self {
        TransitionResult {
            state: res,
            reconsume: false,
            emit: Cell::new(vec![]),
        }
    }

    pub(super) fn is_err(&self) -> bool {
        self.state.is_err()
    }

    #[allow(dead_code)]
    pub(super) fn is_ok(&self) -> bool {
        self.state.is_ok()
    }

    pub(super) fn set_reconsume(&mut self) {
        self.reconsume = true
    }

    pub(super) fn reconsume(&self) -> bool {
        self.reconsume
    }

    pub(super) fn state(self) -> Result<States> {
        self.state
    }

    pub(super) fn emits(&mut self) -> Emit {
        self.emit.replace(Vec::new())
    }

    pub(super) fn push_emit<T: Into<Token>>(&mut self, token: T) {
        let mut emits = self.emit.take();
        emits.push(token.into());
        self.emit.replace(emits);
    }

    #[allow(dead_code)]
    pub(super) fn insert_emit<T: Into<Token>>(&mut self, index: usize, token: T) {
        let mut emits = self.emit.take();
        emits.insert(index, token.into());
        self.emit.replace(emits);
    }

    pub(super) fn push_parse_error(&mut self, err: ParseError) {
        // TODO: Handle parse errors
        println!("Parse Error: {}", err);
    }

    pub(super) fn insert_parse_error(&mut self, _index: usize, err: ParseError) {
        println!("Parse Error: {}", err);
    }
}
