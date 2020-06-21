mod parser_struct;
mod errors;
mod open_elements_stack;
mod states;
mod transition_result;
mod transitions;

use states::States;
use transition_result::TransitionResult;
pub use parser_struct::Parser;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ScriptingFlag {
    Enabled,
    Disabled,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FramesetOkFlag {
    Ok,
    NotOk,
}
