use std::io;

use crate::{
    dom,
    parser::{states::*, Parser, TransitionResult},
    tokenizer::Token,
};

use super::parse_error;

impl AfterAfterBody {
    pub(in crate::parser) fn on_token<R>(
        self,
        parser: &mut Parser<R>,
        t: &Token,
    ) -> TransitionResult 
    where
        R: io::Read + io::Seek,
    {
        match t {
            Token::Comment(comment) => {
                let node = dom::Comment::new(comment.clone());
                parser.document.push(node);
                States::from(self).into_transition_result()
            }
            Token::Doctype(_d) => todo!("AfterAfterBody::on_token(Doctype)"),
            Token::Character('\t') | Token::Character('\n') | Token::Character(' ') => {
                todo!()
                // States::in_body().on_token(document, t)
            }
            Token::StartTag(tag) if tag.name == "html" => todo!("AfterAfterBody::on_token('html')"),
            Token::Eof => States::term().into_transition_result(),
            _ => {
                parse_error("AfterAfterBody::on_token(_)");

                let mut ret = States::in_body().into_transition_result();
                ret.set_reprocess();
                ret
            }
        }
    }
}
