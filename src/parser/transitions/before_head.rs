use crate::{
    dom,
    parser::{states::*, Parser, TransitionResult},
    tokenizer::{TagName, Token},
};
use std::io;

use super::parse_error;

impl BeforeHead {
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
        Token::Character('\t') | Token::Character('\n') | Token::Character(' ') => {
            current_state.into_transition_result()
        }
        Token::Comment(comment) => {
            let node = dom::Comment::new(comment.clone());
            parser.document.push(node);
            current_state.into_transition_result()
        }
        Token::Doctype(_) => {
            parse_error("BeforeHead::on_token(Doctype)");
            current_state.into_transition_result()
        }
        Token::StartTag(tag) if tag.name == TagName::Html => {
            todo!("Process the token using the rules for the \"in body\" insertion mode.")
        }
        Token::StartTag(tag) if tag.name == TagName::Head => todo!(
            "Insert an HTML element for the token.
Set the head element pointer to the newly created head element.
Switch the insertion mode to \"in head\". "
        ),
        Token::EndTag(tag)
            if (tag.name != TagName::Head
                && tag.name != TagName::Body
                && tag.name != TagName::Html
                && tag.name != TagName::Br) =>
        {
            // Parse error. Ignore the token.
            parse_error("BeforeHead::on_token(EndTag(_))");
            current_state.into_transition_result()
        }
        _ => {
            // Insert an HTML element for a "head" start tag token with no attributes.
            // Set the head element pointer to the newly created head element.
            // Switch the insertion mode to "in head".
            // Reprocess the current token.

            let node = dom::Element::new_html(TagName::Head);
            parser.document.set_head(node);

            let mut ret = States::in_head().into_transition_result();
            ret.set_reprocess();
            ret
        }
    }
}
