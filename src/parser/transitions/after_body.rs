use log::warn;

use crate::{
    dom,
    parser::{states::*, Parser, TransitionResult},
    tokenizer::{TagName, Token},
};
use std::io;

use super::parse_error;

impl AfterBody {
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
    _parser: &mut Parser<R>,
    t: &Token,
) -> TransitionResult
where
    R: io::Read + io::Seek,
{
    match t {
        Token::Character('\t') | Token::Character('\n') | Token::Character(' ') => {
            todo!("AfterBody::on_token('\\w')");
            // current_state.into_transition_result()
        }
        Token::Comment(_comment) => {
            todo!("AfterBody::on_token(Comment)");
        }
        Token::Doctype(_) => {
            parse_error("AfterBody::on_token(Doctype)");
            current_state.into_transition_result()
        }
        Token::StartTag(tag) if tag.name == TagName::Html => {
            todo!("AfterBody::on_token('html')");
        }
        Token::EndTag(tag) if tag.name == TagName::Html => {
            warn!("TODO: ...");
            States::after_after_body().into_transition_result()
        }
        Token::Eof => States::term().into_transition_result(),
        _ => {
            parse_error("AfterBody::on_token(_)");

            let mut ret = States::in_body().into_transition_result();
            ret.set_reprocess();
            ret
        }
    }
}
