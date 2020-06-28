use std::io;

use crate::{
    dom,
    parser::{encodings, parse_error, states::{self, States}, Parser, ScriptingFlag, TransitionResult},
    tokenizer::{TagName, Token},
};

impl states::InHead {
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

#[allow(clippy::too_many_lines)]
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
            parse_error("InHead::on_token(Doctype)");
            current_state.into_transition_result()
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
            let node = dom::Element::new_html(tag.name.clone());
            parser.insert_html_element(node);
            let _ = parser.open_elements.pop();

            if tag.is_self_closing() {
                todo!("InHead:on_token(Self closing 'meta')");
            }

            if let Some(attr) = tag.attributes_iter().find(|a| a.name == "charset") {
                if let Some(encoding) = encodings::get_encoding(&attr.value) {
                    if encoding.name != "UTF-8" {
                        panic!("Unsupported encoding - {:?}", encoding);
                    }
                };
            }
            if let Some(attr) = tag.attributes_iter().find(|a| a.name == "http-equiv") {
                if attr.value.eq_ignore_ascii_case("content-type") {
                    if let Some(_attr) = tag.attributes_iter().find(|a| a.name == "content") {
                        todo!("InHead(StartTag('meta')) Unsupported <meta http-equiv='Content-Type' content='...'>")
                    }
                }
            }

            current_state.into_transition_result()
        }
        Token::StartTag(tag) if tag.name == TagName::Title => {
            // Follow the generic RCDATA element parsing algorithm.
            parser.generic_rcdata_element_parse(current_state, t)
        }
        Token::StartTag(tag)
            if tag.name == TagName::Noscript && parser.scripting == ScriptingFlag::Enabled =>
        {
            parser.generic_raw_text_element_parse(current_state, t)
        }
        Token::StartTag(tag) if (tag.name == TagName::Noframes || tag.name == TagName::Style) => {
            parser.generic_raw_text_element_parse(current_state, t)
        }
        Token::StartTag(tag)
            if tag.name == TagName::Noscript && parser.scripting == ScriptingFlag::Disabled =>
        {
            let node = dom::Element::new_html(tag.name.clone());
            parser.insert_html_element(node);

            States::in_head_noscript().into_transition_result()
        }
        Token::StartTag(tag) if tag.name == TagName::Script => {
            todo!("InHead::on_token('script')");
        }
        Token::EndTag(tag) if tag.name == TagName::Head => {
            // Pop the current node (which will be the head element) off the stack of open elements.
            let elem = parser
                .open_elements
                .pop()
                .expect("Expected element on the stack of open elements");
            if elem.borrow().name() != &TagName::Head {
                panic!(
                    "Unexpected element on the stack of open elements: {:?} (Expected 'head')",
                    elem
                );
            }
            States::after_head().into_transition_result()
        }
        Token::EndTag(tag)
            if (tag.name == TagName::Body
                || tag.name == TagName::Html
                || tag.name == TagName::Br) =>
        {
            // Pop the current node (which will be the head element) off the stack of open elements.
            let elem = parser
                .open_elements
                .pop()
                .expect("Expected element on the stack of open elements");
            if elem.borrow().name() != &TagName::Head {
                panic!(
                    "Unexpected element on the stack of open elements: {:?} (Expected 'head')",
                    elem
                );
            }

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
            current_state.into_transition_result()
        }
        Token::EndTag(_) => {
            parse_error("InHead::on_token(EndTag(_))");
            current_state.into_transition_result()
        }
        _ => {
            let elem = parser
                .open_elements
                .pop()
                .expect("Expected element on the stack of open elements");
            if elem.borrow().name() != &TagName::Head {
                panic!(
                    "Unexpected element on the stack of open elements: {:?} (Expected 'head')",
                    elem
                );
            }

            let mut ret = States::after_head().into_transition_result();
            ret.set_reprocess();
            ret
        }
    }
}
