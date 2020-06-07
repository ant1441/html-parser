use crate::{
    dom,
    parser::{states::*, Parser, TransitionResult},
    tokenizer::Token,
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
            Token::StartTag(tag) if tag.name == "html" => {
                todo!("InHead::on_token('html')");
            }
            Token::StartTag(tag)
                if (tag.name == "base"
                    || tag.name == "basefont"
                    || tag.name == "bgsound"
                    || tag.name == "link") =>
            {
                todo!("InHead::on_token('base|basefont|bgsound|link')");
            }
            Token::StartTag(tag) if tag.name == "meta" => {
                todo!("InHead::on_token('meta')");
            }
            Token::StartTag(tag) if tag.name == "title" => {
                todo!("InHead::on_token('title')");
            }
            Token::StartTag(tag) if tag.name == "noscript" /* && scripting_flag */ => {
                todo!("InHead::on_token('noscript') - scripting disabled");
            }
            Token::StartTag(tag)
                if (tag.name == "noframes"
                    || tag.name == "style") =>
            {
                todo!("InHead::on_token('base|basefont|bgsound|link')");
            }
            Token::StartTag(tag) if tag.name == "noscript" /* && !scripting_flag */ => {
                todo!("InHead::on_token('noscript') - scripting disabled");
            }
            Token::StartTag(tag) if tag.name == "script" => {
                todo!("InHead::on_token('script')");
            }
            Token::EndTag(tag) if tag.name == "head" => {
                todo!("InHead::on_token('head')");
            }
            Token::EndTag(tag)
                if (tag.name == "body"
                    || tag.name == "html"
                    || tag.name == "br") =>
            {
                // Pop the current node (which will be the head element) off the stack of open elements.

                let mut ret = States::after_head().into_transition_result();
                ret.set_reprocess();
                ret
            }
            Token::StartTag(tag) if tag.name == "template" => {
                todo!("InHead::on_token('template')");
            }
            Token::EndTag(tag) if tag.name == "template" => {
                todo!("InHead::on_token('template')");
            }
            Token::StartTag(tag) if tag.name == "head" => {
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
