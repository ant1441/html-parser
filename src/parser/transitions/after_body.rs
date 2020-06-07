use log::warn;

use crate::{
    dom,
    parser::{states::*, TransitionResult},
    tokenizer::Token,
};

use super::parse_error;

impl AfterBody {
    pub(in crate::parser) fn on_token(
        self,
        _document: &mut dom::Document,
        t: &Token,
    ) -> TransitionResult {
        match t {
            Token::Character('\t') | Token::Character('\n') | Token::Character(' ') => {
                todo!("AfterBody::on_token('\\w')");
                // States::from(self).into_transition_result()
            }
            Token::Comment(_comment) => {
                todo!("AfterBody::on_token(Comment)");
            }
            Token::Doctype(_) => {
                parse_error("AfterBody::on_token(Doctype)");
                States::from(self).into_transition_result()
            }
            Token::StartTag(tag) if tag.name == "html" => {
                todo!("AfterBody::on_token('html')");
            }
            Token::EndTag(tag) if tag.name == "html" => {
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
}
