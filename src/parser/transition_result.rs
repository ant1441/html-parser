use super::{errors::{Error, Result}, States};

#[derive(Debug)]
pub(super) struct TransitionResult {
    state: Result<States>,
    reprocess: bool,
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


impl TransitionResult {
    pub(super) fn from_state(state: States) -> Self {
        TransitionResult::from_result(Ok(state))
    }

    fn from_result(res: Result<States>) -> Self {
        TransitionResult {
            state: res,
            reprocess: false,
        }
    }

    pub(super) fn is_err(&self) -> bool {
        self.state.is_err()
    }

    #[allow(dead_code)]
    pub(super) fn is_ok(&self) -> bool {
        self.state.is_ok()
    }

    pub(super) fn set_reprocess(&mut self) {
        self.reprocess = true
    }

    pub(super) fn reprocess(&self) -> bool{
        self.reprocess
    }

    pub(super) fn state(self) -> Result<States> {
        self.state
    }
}
