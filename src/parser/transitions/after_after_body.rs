use crate::{
    dom,
    parser::{states::*, TransitionResult},
    tokenizer::Token,
};

use super::parse_error;

impl AfterAfterBody {
    pub(in crate::parser) fn on_token(
        self,
        document: &mut dom::Document,
        t: &Token,
    ) -> TransitionResult {
        match t {
            Token::Comment(comment) => {
                let node = dom::Comment::new(comment.clone());
                document.push(node);
                States::from(self).into_transition_result()
            }
            Token::Doctype(_d) => todo!("AfterAfterBody::on_token(Doctype)"),
            Token::Character('\t') | Token::Character('\n') | Token::Character(' ') => {
                States::in_body().on_token(document, t)
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
