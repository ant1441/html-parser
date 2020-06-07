#![allow(unused_imports)]

use log::{error, trace, warn};

use crate::{
    dom,
    parser::{states::*, TransitionResult},
    tokenizer::Token,
};

mod force_quirks_check;

/*
 * Transition Impls
 */

// TODO: unwraps
//
mod after_after_body;
mod after_body;
mod after_head;
mod before_head;
mod before_html;
mod in_body;
mod in_head;
mod initial;

/*
 * Transition Helpers
 */

fn parse_error(msg: &str) {
    error!("Parse Error: {}", msg);
}
