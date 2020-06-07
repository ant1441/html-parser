use crate::{
    dom,
    parser::{states::*, TransitionResult},
    tokenizer::Token,
};

use super::parse_error;

impl AfterHead {
    pub(in crate::parser) fn on_token(
        self,
        document: &mut dom::Document,
        t: &Token,
    ) -> TransitionResult {
        match t {
            Token::Character('\t') | Token::Character('\n') | Token::Character(' ') => {
                todo!("AfterHead::on_token('\\w') - Insert the character.");
                // States::from(self).into_transition_result()
            }
            Token::Comment(comment) => {
                let node = dom::Comment::new(comment.clone());
                document.push(node);
                States::from(self).into_transition_result()
            }
            Token::Doctype(_) => {
                parse_error("AfterHead::on_token(Doctype)");
                States::from(self).into_transition_result()
            }
            Token::StartTag(tag) if tag.name == "html" => {
                todo!("AfterHead::on_token('html')");
            }
            Token::StartTag(tag) if tag.name == "body" => {
                todo!("AfterHead::on_token('body')");
            }
            Token::StartTag(tag) if tag.name == "frameset" => {
                todo!("AfterHead::on_token('frameset')");
            }
            Token::StartTag(tag)
                if (tag.name == "base"
                    || tag.name == "basefont"
                    || tag.name == "bgsound"
                    || tag.name == "link"
                    || tag.name == "meta"
                    || tag.name == "noframes"
                    || tag.name == "script"
                    || tag.name == "style"
                    || tag.name == "template"
                    || tag.name == "title") =>
            {
                parse_error("AfterHead::on_token(StartTag('base|basefont|bgsound|link|meta|noframes|script|style|template|title'))");
                todo!("AfterHead::on_token('base|basefont|bgsound|link|meta|noframes|script|style|template|title')");
            }
            Token::EndTag(tag) if tag.name == "template" => {
                todo!("AfterHead::on_token('template')");
            }
            Token::EndTag(tag)
                if (tag.name == "body" || tag.name == "html" || tag.name == "br") =>
            {
                // Insert an HTML element for a "body" start tag token with no attributes.
                let node = dom::Element::new("body".to_string());
                document.push(node);

                let mut ret = States::in_body().into_transition_result();
                ret.set_reprocess();
                ret
            }
            Token::StartTag(tag) if tag.name == "head" => {
                parse_error("AfterHead::on_token(StartTag('head'))");
                States::from(self).into_transition_result()
            }
            Token::EndTag(_) => {
                parse_error("AfterHead::on_token(EndTag(_))");
                States::from(self).into_transition_result()
            }
            _ => todo!("AfterHead::on_token(_)"),
        }
    }
}
