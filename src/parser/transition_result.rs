use super::{errors::Result, States};

#[derive(Debug)]
pub(super) struct TransitionResult {
    state: Result<States>,
}
