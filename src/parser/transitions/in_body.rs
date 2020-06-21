use log::warn;

use crate::{
    dom,
    parser::{states::*, Parser, TransitionResult},
    tokenizer::{TagName, Token},
};
use std::io;

use super::parse_error;

impl InBody {
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
        Token::Character('\0') => {
            parse_error("InBody::on_token(\\0)");
            current_state.into_transition_result()
        }
        Token::Character(ch @ '\t') | Token::Character(ch @ '\n') | Token::Character(ch @ ' ') => {
            warn!("[TODO] InBody: \\t|\\n|  - Reconstruct the active formatting elements, if any.");
            parser.insert_character(ch.to_string());
            current_state.into_transition_result()
        }
        Token::Character(_) => {
            todo!("InBody::on_token(_)");
        }
        Token::Comment(comment) => {
            let node = dom::Comment::new(comment.clone());
            parser.document.push_comment(node);
            current_state.into_transition_result()
        }
        Token::Doctype(_) => {
            parse_error("InBody::on_token(Doctype)");
            current_state.into_transition_result()
        }
        Token::StartTag(tag) if tag.name == TagName::Html => {
            parse_error("InBody::on_token(StartTag('html'))");
            todo!("InBody::on_token('html')");
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
            todo!("InBody::on_token('base|basefont|bgsound|link|meta|noframes|script|style|template|title')");
        }
        Token::EndTag(tag) if tag.name == TagName::Template => {
            todo!("InBody::on_token('template')");
        }
        Token::StartTag(tag) if tag.name == TagName::Body => {
            todo!("InBody::on_token('body')");
        }
        Token::StartTag(tag) if tag.name == TagName::Frameset => {
            todo!("InBody::on_token('frameset')");
        }
        Token::Eof => {
            warn!("TODO: EOF");
            States::term().into_transition_result()
        }
        Token::EndTag(tag) if tag.name == TagName::Body => {
            todo!("InBody::on_token('body')");
        }
        Token::EndTag(tag) if tag.name == TagName::Html => {
            if !parser.open_elements.contains_element(&TagName::Body) {
                parse_error("No Body in stack of open elements");
                // ignore the token.
                return current_state.into_transition_result();
            }
            let has_unexpected_elem = parser
                .open_elements
                .iter()
                .filter(|e| {
                    let elem = e.borrow();
                    let name = elem.name();

                    name != &TagName::Dd
                        && name != &TagName::Dt
                        && name != &TagName::Li
                        && name != &TagName::Optgroup
                        && name != &TagName::Option
                        && name != &TagName::P
                        && name != &TagName::Rb
                        && name != &TagName::Rp
                        && name != &TagName::Rt
                        && name != &TagName::Rtc
                        && name != &TagName::Tbody
                        && name != &TagName::Td
                        && name != &TagName::Tfoot
                        && name != &TagName::Th
                        && name != &TagName::Thead
                        && name != &TagName::Tr
                        && name != &TagName::Body
                        && name != &TagName::Html
                })
                .count()
                > 0;
            if has_unexpected_elem {
                parse_error("Unexpected element(s) in stack of open elements");
            }

            let mut ret = States::after_body().into_transition_result();
            ret.set_reprocess();
            ret
        }
        Token::StartTag(tag)
            if (tag.name == TagName::Address
                || tag.name == TagName::Article
                || tag.name == TagName::Aside
                || tag.name == TagName::Blockquote
                || tag.name == TagName::Center
                || tag.name == TagName::Details
                || tag.name == TagName::Dialog
                || tag.name == TagName::Dir
                || tag.name == TagName::Div
                || tag.name == TagName::Dl
                || tag.name == TagName::Fieldset
                || tag.name == TagName::Figcaption
                || tag.name == TagName::Figure
                || tag.name == TagName::Footer
                || tag.name == TagName::Header
                || tag.name == TagName::Hgroup
                || tag.name == TagName::Main
                || tag.name == TagName::Menu
                || tag.name == TagName::Nav
                || tag.name == TagName::Ol
                || tag.name == TagName::P
                || tag.name == TagName::Section
                || tag.name == TagName::Summary
                || tag.name == TagName::Ul) =>
        {
            todo!("InBody::on_token('address|...')");
        }
        Token::StartTag(tag)
            if (tag.name == TagName::H1
                || tag.name == TagName::H2
                || tag.name == TagName::H3
                || tag.name == TagName::H4
                || tag.name == TagName::H5
                || tag.name == TagName::H6) =>
        {
            todo!("InBody::on_token('hN|...')");
        }
        Token::StartTag(tag) if (tag.name == TagName::Pre || tag.name == TagName::Listing) => {
            todo!("InBody::on_token('pre|listing')");
        }
        Token::StartTag(tag) if tag.name == TagName::Form => {
            todo!("InBody::on_token('form')");
        }
        Token::StartTag(tag) if tag.name == TagName::Li => {
            todo!("InBody::on_token('li')");
        }
        Token::StartTag(tag) if (tag.name == TagName::Dd || tag.name == TagName::Dt) => {
            todo!("InBody::on_token('dd|dt')");
        }
        Token::StartTag(tag) if tag.name == TagName::Plaintext => {
            todo!("InBody::on_token('plaintext')");
        }
        Token::StartTag(tag) if tag.name == TagName::Button => {
            todo!("InBody::on_token('button')");
        }
        _ => todo!("InBody::on_token(_)"),
    }
}
