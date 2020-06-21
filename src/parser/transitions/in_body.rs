use log::{trace, warn};

use crate::{
    dom::{self, Namespace},
    parser::{self, states::*, Parser, TransitionResult},
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
            warn!(
                "[TODO] InBody: '\\t|\\n| ' - Reconstruct the active formatting elements, if any."
            );
            parser.insert_character(ch.to_string());
            current_state.into_transition_result()
        }
        Token::Characters(ch) => {
            warn!("[TODO] InBody: _  - Reconstruct the active formatting elements, if any.");
            parser.insert_character(ch.to_string());
            parser.frameset_ok = parser::FramesetOkFlag::NotOk;

            current_state.into_transition_result()
        }
        Token::Character(ch) => {
            warn!("[TODO] InBody: _  - Reconstruct the active formatting elements, if any.");
            parser.insert_character(ch.to_string());
            parser.frameset_ok = parser::FramesetOkFlag::NotOk;

            current_state.into_transition_result()
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
            // TODO: If the stack of template insertion modes is not empty, then process the token using the rules for the "in template" insertion mode.

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

            States::term().into_transition_result()
        }
        Token::EndTag(tag) if tag.name == TagName::Body => {
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

            States::after_body().into_transition_result()
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
            if parser.open_elements.contains_element(&TagName::P) {
                todo!("InBody::on_token('address|...') - close a p element");
            }

            let node = dom::Element::new_html(tag.name.clone());
            parser.insert_html_element(node);

            current_state.into_transition_result()
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
        Token::EndTag(tag)
            if (tag.name == TagName::Address
                || tag.name == TagName::Article
                || tag.name == TagName::Aside
                || tag.name == TagName::Blockquote
                || tag.name == TagName::Button
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
                || tag.name == TagName::Listing
                || tag.name == TagName::Main
                || tag.name == TagName::Menu
                || tag.name == TagName::Nav
                || tag.name == TagName::Ol
                || tag.name == TagName::Pre
                || tag.name == TagName::Section
                || tag.name == TagName::Summary
                || tag.name == TagName::Ul) =>
        {
            if !parser.open_elements.contains_element(&tag.name) {
                parse_error(&format!("No {} in stack of open elements", &tag.name));
                return current_state.into_transition_result();
            }

            parser.generate_implied_end_tags();
            let current_node = parser.current_node().unwrap();
            let current_node = current_node.borrow();
            if !(current_node.namespace == Namespace::HTML && current_node.name == tag.name) {
                parse_error("Unexpected tag")
            }
            while let Some(e) = parser.open_elements.pop() {
                let elem = e.borrow();
                if elem.name == tag.name {
                    break;
                }
                trace!("InBody: Popped {:?} off stack", elem);
            }

            current_state.into_transition_result()
        }
        Token::EndTag(tag) if tag.name == TagName::Form => {
            todo!("InBody::on_token(EndTag('form'))");
        }
        Token::EndTag(tag) if tag.name == TagName::P => {
            todo!("InBody::on_token(EndTag('p'))");
        }
        Token::EndTag(tag) if tag.name == TagName::Li => {
            todo!("InBody::on_token(EndTag('li'))");
        }
        Token::EndTag(tag) if (tag.name == TagName::Dd || tag.name == TagName::Dt) => {
            todo!("InBody::on_token(EndTag('dd|dt'))");
        }
        Token::EndTag(tag)
            if (tag.name == TagName::H1
                || tag.name == TagName::H2
                || tag.name == TagName::H3
                || tag.name == TagName::H4
                || tag.name == TagName::H5
                || tag.name == TagName::H6) =>
        {
            todo!("InBody::on_token(EndTag('hN|...'))");
        }
        Token::EndTag(tag) if tag.name == TagName::Other("sarcasm".to_string()) => {
            panic!("This parser is very serious")
        }
        Token::StartTag(tag) if tag.name == TagName::A => {
            todo!("InBody::on_token('a')");
        }
        Token::StartTag(tag)
            if (tag.name == TagName::B
                || tag.name == TagName::Big
                || tag.name == TagName::Code
                || tag.name == TagName::Em
                || tag.name == TagName::Font
                || tag.name == TagName::I
                || tag.name == TagName::S
                || tag.name == TagName::Small
                || tag.name == TagName::Strike
                || tag.name == TagName::Strong
                || tag.name == TagName::Tt
                || tag.name == TagName::U) =>
        {
            todo!("InBody::on_token('b|...')");
        }
        Token::StartTag(tag) if tag.name == TagName::Nobr => {
            todo!("InBody::on_token('nobr')");
        }
        Token::EndTag(tag)
            if (tag.name == TagName::A
                || tag.name == TagName::B
                || tag.name == TagName::Big
                || tag.name == TagName::Code
                || tag.name == TagName::Em
                || tag.name == TagName::Font
                || tag.name == TagName::I
                || tag.name == TagName::Nobr
                || tag.name == TagName::S
                || tag.name == TagName::Small
                || tag.name == TagName::Strike
                || tag.name == TagName::Strong
                || tag.name == TagName::Tt
                || tag.name == TagName::U) =>
        {
            todo!("InBody::on_token(EndTag('b|...'))");
        }
        Token::StartTag(tag)
            if (tag.name == TagName::Applet
                || tag.name == TagName::Marquee
                || tag.name == TagName::Object) =>
        {
            todo!("InBody::on_token('applet|...')");
        }
        Token::EndTag(tag)
            if (tag.name == TagName::Applet
                || tag.name == TagName::Marquee
                || tag.name == TagName::Object) =>
        {
            todo!("InBody::on_token(endTag('applet|...'))");
        }
        Token::StartTag(tag) if tag.name == TagName::Table => {
            todo!("InBody::on_token('table')");
        }
        Token::EndTag(tag) if tag.name == TagName::Br => {
            todo!("InBody::on_token('br')");
        }
        Token::StartTag(tag)
            if (tag.name == TagName::Area
                || tag.name == TagName::Br
                || tag.name == TagName::Embed
                || tag.name == TagName::Img
                || tag.name == TagName::Keygen
                || tag.name == TagName::Wbr) =>
        {
            todo!("InBody::on_token('area|...')");
        }
        Token::StartTag(tag) if tag.name == TagName::Input => {
            todo!("InBody::on_token('input')");
        }
        Token::StartTag(tag)
            if (tag.name == TagName::Param
                || tag.name == TagName::Source
                || tag.name == TagName::Track) =>
        {
            todo!("InBody::on_token('param|...')");
        }
        Token::StartTag(tag) if tag.name == TagName::Hr => {
            todo!("InBody::on_token('hr')");
        }
        Token::StartTag(tag) if tag.name == TagName::Other("image".to_string()) => {
            todo!("InBody::on_token('image')");

            // Don't ask, apparently
            tag.name = TagName::Img;

            let mut ret = current_state.into_transition_result();
            ret.set_reprocess();
            ret
        }
        Token::StartTag(tag) if tag.name == TagName::Textarea => {
            todo!("InBody::on_token('textarea')");
        }
        Token::StartTag(tag) if tag.name == TagName::Xmp => {
            todo!("InBody::on_token('xmp')");
        }
        Token::StartTag(tag) if tag.name == TagName::Iframe => {
            todo!("InBody::on_token('iframe')");
        }
        Token::StartTag(tag) if tag.name == TagName::Noembed => {
            todo!("InBody::on_token('noembed')");
        }
        Token::StartTag(tag)
            if tag.name == TagName::Noscript
                && parser.scripting == parser::ScriptingFlag::Enabled =>
        {
            todo!("InBody::on_token('noscript')");
        }
        Token::StartTag(tag) if tag.name == TagName::Select => {
            todo!("InBody::on_token('select')");
        }
        Token::StartTag(tag) if (tag.name == TagName::Optgroup || tag.name == TagName::Option) => {
            todo!("InBody::on_token('Optgroup|option')");
        }
        Token::StartTag(tag) if (tag.name == TagName::Rb || tag.name == TagName::Rtc) => {
            todo!("InBody::on_token('rb|rtc')");
        }
        Token::StartTag(tag) if (tag.name == TagName::Rp || tag.name == TagName::Rt) => {
            todo!("InBody::on_token('rp|rt')");
        }
        Token::StartTag(tag) if tag.name == TagName::Math => {
            todo!("InBody::on_token('math')");
        }
        Token::StartTag(tag) if tag.name == TagName::Svg => {
            todo!("InBody::on_token('svg')");
        }
        Token::StartTag(tag)
            if (tag.name == TagName::Caption
                || tag.name == TagName::Col
                || tag.name == TagName::Colgroup
                || tag.name == TagName::Frame
                || tag.name == TagName::Head
                || tag.name == TagName::Tbody
                || tag.name == TagName::Td
                || tag.name == TagName::Tfoot
                || tag.name == TagName::Th
                || tag.name == TagName::Thead
                || tag.name == TagName::Tr) =>
        {
            todo!("InBody::on_token('caption|...')");
        }
        Token::StartTag(_tag) => {
            todo!("InBody::on_token(StartTag(_))");
        }
        Token::EndTag(_tag) => {
            todo!("InBody::on_token(EndTag(_))");
        }
    }
}
