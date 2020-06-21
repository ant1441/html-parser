use std::io;

use crate::{
    dom,
    parser::{states::*, Parser, TransitionResult},
    tokenizer::{TagName, Token},
};

use super::parse_error;

impl BeforeHtml {
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
        Token::Doctype(_) => {
            parse_error("BeforeHtml::on_token(Doctype)");
            current_state.into_transition_result()
        }
        Token::Character('\t') | Token::Character('\n') | Token::Character(' ') => {
            current_state.into_transition_result()
        }
        Token::StartTag(tag) if tag.name == TagName::Html => {
            let node = dom::Element::new(tag.name.clone());
            parser.document.push(node);

            // TODO: Put this element in the stack of open elements.

            // TODO: If the Document is being loaded as part of navigation of a browsing context and the result of executing Is environment settings object a secure context? on the Document's relevant settings object is true, then:
            // ...
            States::before_head().into_transition_result()
        }
        Token::EndTag(tag)
            if (tag.name != TagName::Head
                && tag.name != TagName::Body
                && tag.name != TagName::Html
                && tag.name != TagName::Br) =>
        {
            // Parse error. Ignore the token.
            parse_error("BeforeHtml::on_token(EndTag(_))");
            current_state.into_transition_result()
        }
        _ => todo!("BeforeHtml::on_token({:?})", t),
    }
}
