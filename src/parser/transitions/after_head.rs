use std::io;

use crate::{
    dom,
    parser::{parse_error, self, states::{self, States}, Parser, TransitionResult},
    tokenizer::{TagName, Token},
};

impl states::AfterHead {
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
            Token::Character(ch @ '\t') | Token::Character(ch @ '\n') | Token::Character(ch @ ' ') => {
                parser.insert_character(ch.to_string());
                current_state.into_transition_result()
            }
            Token::Comment(comment) => {
                let node = dom::Comment::new(comment.clone());
                parser.document.push_comment(node);
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
                let node = dom::Element::new_html(TagName::Body);
                parser.insert_html_element(node);
                parser.frameset_ok = parser::FramesetOkFlag::NotOk;

                States::in_body().into_transition_result()
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
                parser.document.push_element(node);

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
