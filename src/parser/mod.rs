mod errors;
mod list_of_active_formatting_elements;
mod open_elements_stack;
mod parser_struct;
mod states;
mod transition_result;
mod transitions;
pub mod encodings;

pub(self) use list_of_active_formatting_elements::ListOfActiveFormattingElements;
pub(self) use open_elements_stack::OpenElementsStack;
pub use parser_struct::Parser;
use states::States;
use transition_result::TransitionResult;

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
