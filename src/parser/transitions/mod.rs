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

pub(super) mod after_after_body;
pub(super) mod after_body;
pub(super) mod after_head;
pub(super) mod before_head;
pub(super) mod before_html;
pub(super) mod in_body;
pub(super) mod in_head;
pub(super) mod initial;

/*
 * Transition Helpers
 */

fn parse_error(msg: &str) {
    error!("Parse Error: {}", msg);
}
