use crate::{
    dom,
    parser::{states::*, Parser, TransitionResult},
    tokenizer::{TagName, Token},
};
use std::io;

use super::parse_error;

impl InHead {
    pub(in crate::parser) fn on_token<R>(
        self,
        parser: &mut Parser<R>,
        t: &Token,
    ) -> TransitionResult
    where
        R: io::Read + io::Seek,
    {
        match t {
            Token::Character('\t') | Token::Character('\n') | Token::Character(' ') => {
                todo!("InHead::on_token('\\w') - Insert the character.");
                // States::from(self).into_transition_result()
            }
            Token::Comment(comment) => {
                let node = dom::Comment::new(comment.clone());
                parser.document.push(node);
                States::from(self).into_transition_result()
            }
            Token::Doctype(_) => {
                parse_error("InHead::on_token(Doctype)");
                States::from(self).into_transition_result()
            }
            Token::StartTag(tag) if tag.name == TagName::Html => {
                todo!("InHead::on_token('html')");
            }
            Token::StartTag(tag)
                if (tag.name == TagName::Base
                    || tag.name == TagName::Basefont
                    || tag.name == TagName::Bgsound
                    || tag.name == TagName::Link) =>
            {
                todo!("InHead::on_token('base|basefont|bgsound|link')");
            }
            Token::StartTag(tag) if tag.name == TagName::Meta => {
                todo!("InHead::on_token('meta')");
            }
            Token::StartTag(tag) if tag.name == TagName::Title => {
                todo!("InHead::on_token('title')");
            }
            Token::StartTag(tag) if tag.name == TagName::Noscript /* && scripting_flag */ => {
                todo!("InHead::on_token('noscript') - scripting disabled");
            }
            Token::StartTag(tag)
                if (tag.name == TagName::Noframes
                    || tag.name == TagName::Style) =>
            {
                todo!("InHead::on_token('base|basefont|bgsound|link')");
            }
            Token::StartTag(tag) if tag.name == TagName::Noscript /* && !scripting_flag */ => {
                todo!("InHead::on_token('noscript') - scripting disabled");
            }
            Token::StartTag(tag) if tag.name == TagName::Script => {
                todo!("InHead::on_token('script')");
            }
            Token::EndTag(tag) if tag.name == TagName::Head => {
                todo!("InHead::on_token('head')");
            }
            Token::EndTag(tag)
                if (tag.name == TagName::Body
                    || tag.name == TagName::Html
                    || tag.name == TagName::Br) =>
            {
                // Pop the current node (which will be the head element) off the stack of open elements.

                let mut ret = States::after_head().into_transition_result();
                ret.set_reprocess();
                ret
            }
            Token::StartTag(tag) if tag.name == TagName::Template => {
                todo!("InHead::on_token('template')");
            }
            Token::EndTag(tag) if tag.name == TagName::Template => {
                todo!("InHead::on_token('template')");
            }
            Token::StartTag(tag) if tag.name == TagName::Head => {
                parse_error("InHead::on_token(StartTag('head'))");
                States::from(self).into_transition_result()
            }
             Token::EndTag(_) => {
                parse_error("InHead::on_token(EndTag(_))");
                States::from(self).into_transition_result()
            }
            _ => todo!("InHead::on_token(_)"),
        }
    }
}
