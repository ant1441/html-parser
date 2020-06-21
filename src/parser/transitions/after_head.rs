use crate::{
    dom,
    parser::{states::*, Parser, TransitionResult},
    tokenizer::{TagName, Token},
};
use std::io;

use super::parse_error;

impl AfterHead {
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
                todo!("AfterHead::on_token('\\w') - Insert the character.");
                // current_state.into_transition_result()
            }
            Token::Comment(comment) => {
                let node = dom::Comment::new(comment.clone());
                parser.document.push(node);
                current_state.into_transition_result()
            }
            Token::Doctype(_) => {
                parse_error("AfterHead::on_token(Doctype)");
                current_state.into_transition_result()
            }
            Token::StartTag(tag) if tag.name == TagName::Html => {
                todo!("AfterHead::on_token('html')");
            }
            Token::StartTag(tag) if tag.name == TagName::Body => {
                todo!("AfterHead::on_token('body')");
            }
            Token::StartTag(tag) if tag.name == TagName::Frameset => {
                todo!("AfterHead::on_token('frameset')");
            }
            Token::StartTag(tag)
                if (tag.name == TagName::Base
                    || tag.name == TagName::Basefont
                    || tag.name == TagName::Bgsound
                    || tag.name == TagName::Link
                    || tag.name == TagName::Meta
                    || tag.name == TagName::Noframes
                    || tag.name == TagName::Script
                    || tag.name == TagName::Style
                    || tag.name == TagName::Template
                    || tag.name == TagName::Title) =>
            {
                parse_error("AfterHead::on_token(StartTag('base|basefont|bgsound|link|meta|noframes|script|style|template|title'))");
                todo!("AfterHead::on_token('base|basefont|bgsound|link|meta|noframes|script|style|template|title')");
            }
            Token::EndTag(tag) if tag.name == TagName::Template => {
                todo!("AfterHead::on_token('template')");
            }
            Token::EndTag(tag)
                if (tag.name == TagName::Body || tag.name == TagName::Html || tag.name == TagName::Br) =>
            {
                // Insert an HTML element for a "body" start tag token with no attributes.
                let node = dom::Element::new_html(TagName::Body);
                parser.document.push(node);

                let mut ret = States::in_body().into_transition_result();
                ret.set_reprocess();
                ret
            }
            Token::StartTag(tag) if tag.name == TagName::Head => {
                parse_error("AfterHead::on_token(StartTag('head'))");
                current_state.into_transition_result()
            }
            Token::EndTag(_) => {
                parse_error("AfterHead::on_token(EndTag(_))");
                current_state.into_transition_result()
            }
            _ => todo!("AfterHead::on_token(_)"),
        }
    }
