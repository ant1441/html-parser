use log::warn;

use crate::{
    dom,
    parser::{states::*, TransitionResult},
    tokenizer::Token,
};

use super::parse_error;

impl InBody {
    pub(in crate::parser) fn on_token(
        self,
        document: &mut dom::Document,
        t: &Token,
    ) -> TransitionResult {
        match t {
            Token::Character('\0') => {
                parse_error("InBody::on_token(\\0)");
                States::from(self).into_transition_result()
            }
            Token::Character('\t') | Token::Character('\n') | Token::Character(' ') => {
                warn!("TODO: ...");
                States::from(self).into_transition_result()
            }
            Token::Character(_) => {
                todo!("InBody::on_token(_)");
            }
            Token::Comment(comment) => {
                let node = dom::Comment::new(comment.clone());
                document.push(node);
                States::from(self).into_transition_result()
            }
            Token::Doctype(_) => {
                parse_error("InBody::on_token(Doctype)");
                States::from(self).into_transition_result()
            }
            Token::StartTag(tag) if tag.name == "html" => {
                parse_error("InBody::on_token(StartTag('html'))");
                todo!("InBody::on_token('html')");
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
                todo!("InBody::on_token('base|basefont|bgsound|link|meta|noframes|script|style|template|title')");
            }
            Token::EndTag(tag) if tag.name == "template" => {
                todo!("InBody::on_token('template')");
            }
            Token::StartTag(tag) if tag.name == "body" => {
                todo!("InBody::on_token('body')");
            }
            Token::StartTag(tag) if tag.name == "frameset" => {
                todo!("InBody::on_token('frameset')");
            }
            Token::Eof => {
                warn!("TODO: ...");
                States::term().into_transition_result()
            }
            Token::EndTag(tag) if tag.name == "body" => {
                todo!("InBody::on_token('body')");
            }
            Token::EndTag(tag) if tag.name == "html" => {
                warn!("TODO: ...");
                let mut ret = States::after_body().into_transition_result();
                ret.set_reprocess();
                ret
            }
            Token::StartTag(tag)
                if (tag.name == "address"
                    || tag.name == "article"
                    || tag.name == "aside"
                    || tag.name == "blockquote"
                    || tag.name == "center"
                    || tag.name == "details"
                    || tag.name == "dialog"
                    || tag.name == "dir"
                    || tag.name == "div"
                    || tag.name == "dl"
                    || tag.name == "fieldset"
                    || tag.name == "figcaption"
                    || tag.name == "figure"
                    || tag.name == "footer"
                    || tag.name == "header"
                    || tag.name == "hgroup"
                    || tag.name == "main"
                    || tag.name == "menu"
                    || tag.name == "nav"
                    || tag.name == "ol"
                    || tag.name == "p"
                    || tag.name == "section"
                    || tag.name == "summary"
                    || tag.name == "ul") =>
            {
                todo!("InBody::on_token('address|...')");
            }
            Token::StartTag(tag)
                if (tag.name == "h1"
                    || tag.name == "h2"
                    || tag.name == "h3"
                    || tag.name == "h4"
                    || tag.name == "h5"
                    || tag.name == "h6") =>
            {
                todo!("InBody::on_token('hN|...')");
            }
            Token::StartTag(tag) if (tag.name == "pre" || tag.name == "listing") => {
                todo!("InBody::on_token('pre|listing')");
            }
            Token::StartTag(tag) if tag.name == "form" => {
                todo!("InBody::on_token('form')");
            }
            Token::StartTag(tag) if tag.name == "li" => {
                todo!("InBody::on_token('li')");
            }
            Token::StartTag(tag) if (tag.name == "dd" || tag.name == "dt") => {
                todo!("InBody::on_token('dd|dt')");
            }
            Token::StartTag(tag) if tag.name == "plaintext" => {
                todo!("InBody::on_token('plaintext')");
            }
            Token::StartTag(tag) if tag.name == "button" => {
                todo!("InBody::on_token('button')");
            }
            _ => todo!("InBody::on_token(_)"),
        }
    }
}
