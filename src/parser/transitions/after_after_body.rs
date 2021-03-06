use std::io;

use crate::{
    dom,
    parser::{parse_error, states::{self, States}, transitions::in_body, Parser, TransitionResult},
    tokenizer::{TagName, Token},
};

impl states::AfterAfterBody {
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
        Token::Comment(comment) => {
            let node = dom::Comment::new(comment.clone());
            parser.document.push_comment(node);
            current_state.into_transition_result()
        }
        Token::Doctype(_)
        | Token::Character('\t')
        | Token::Character('\n')
        | Token::Character(' ') => in_body::transition(current_state, parser, t),
        Token::StartTag(tag) if tag.name == TagName::Html => {
            todo!("AfterAfterBody::on_token('html')");
        }
        Token::Eof => States::term().into_transition_result(),
        _ => {
            parse_error("AfterAfterBody::on_token(_)");

            let mut ret = States::in_body().into_transition_result();
            ret.set_reprocess();
            ret
        }
    }
}
