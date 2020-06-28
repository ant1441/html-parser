use log::error;

mod errors;
mod list_of_active_formatting_elements;
mod open_elements_stack;
mod parser_struct;
mod states;
mod transition_result;
mod transitions;
pub mod encodings;

use list_of_active_formatting_elements::ListOfActiveFormattingElements;
use open_elements_stack::OpenElementsStack;
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

// TODO How to report parser errors?
fn parse_error(msg: &str) {
    error!("Parse Error: {}", msg);
}
