use log::{trace, warn};

use crate::{
    dom::{self, Namespace},
    parser::{self, states::*, FramesetOkFlag, Parser, TransitionResult},
    tokenizer::{TagName, Token},
};
use std::io;

use super::parse_error;

impl Text {
    pub(in crate::parser) fn on_token<R>(
        self,
        parser: &mut Parser<R>,
        t: &Token,
    ) -> TransitionResult
    where
        R: io::Read + io::Seek,
    {
        transition(States::from(self), parser, t)
    }
}

pub(super) fn transition<R>(
    current_state: States,
    parser: &mut Parser<R>,
    t: &Token,
) -> TransitionResult
where
    R: io::Read + io::Seek,
{
    match t {
        Token::Character('\0') => unreachable!(),
        Token::Character(ch) => {
            parser.insert_character(ch.to_string());
            current_state.into_transition_result()
        }
        _ => todo!("Text::on_token(_)"),
    }
}
