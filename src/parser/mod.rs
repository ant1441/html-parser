mod parser_struct;
mod errors;
mod open_elements_stack;
mod list_of_active_formatting_elements;
mod states;
mod transition_result;
mod transitions;

use states::States;
use transition_result::TransitionResult;
pub use parser_struct::Parser;
pub(self) use open_elements_stack::OpenElementsStack;
pub(self) use list_of_active_formatting_elements::ListOfActiveFormattingElements;

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
