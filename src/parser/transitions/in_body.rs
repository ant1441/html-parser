use std::io;

use log::{trace, warn};

use crate::{
    dom::{Namespace, Comment, Element, Category},
    parser::{parse_error, states::{self, States}, FramesetOkFlag, Parser, TransitionResult, ScriptingFlag},
    tokenizer::{TagName, Token},
};


impl states::InBody {
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

#[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
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
            parser.frameset_ok = FramesetOkFlag::NotOk;

            current_state.into_transition_result()
        }
        Token::Character(ch) => {
            warn!("[TODO] InBody: _  - Reconstruct the active formatting elements, if any.");
            parser.insert_character(ch.to_string());
            parser.frameset_ok = FramesetOkFlag::NotOk;

            current_state.into_transition_result()
        }
        Token::Comment(comment) => {
            let node = Comment::new(comment.clone());
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

            let node = Element::new_html(tag.name.clone());
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

            parser.generate_implied_end_tags(None);
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
            if parser
                .list_of_active_formatting_elements
                .iter()
                .rev()
                .take_while(|e| !e.is_marker())
                .any(|e| e.is_element(&TagName::A))
            {
                parse_error("Existing A in active formatting elements");
                // run the adoption agency algorithm for the token,
                // then remove that element from the list of active formatting elements and
                // the stack of open elements if the adoption agency algorithm
                // didn't already remove it (it might not have if the element is not in table scope).
                todo!("InBody::on_token('a')");
            }
            warn!("[TODO] InBody: 'A' - Reconstruct the active formatting elements, if any.");
            let node = Element::new_html(tag.name.clone());
            parser.insert_html_element(node.clone());

            parser.list_of_active_formatting_elements.push(node.into());

            current_state.into_transition_result()
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
            adoption_agency_algorithm(parser, t);

            current_state.into_transition_result()
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
            warn!("[TODO] InBody: 'input' - Reconstruct the active formatting elements, if any.");

            let node = Element::new_html(tag.name.clone());
            parser.insert_html_element(node);

            if tag.is_self_closing() {
                warn!(
                    "[TODO] InBody: 'input' - Acknowledge the token's self-closing flag, if it is set."
                );
            }

            if tag
                .attributes_iter()
                .any(|a| a.name == "type" && a.value.to_lowercase() == "hidden")
            {
                parser.frameset_ok = FramesetOkFlag::NotOk;
            }

            current_state.into_transition_result()
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
            /*
            // Don't ask, apparently
            tag.name = TagName::Img;

            let mut ret = current_state.into_transition_result();
            ret.set_reprocess();
            ret
            */

            todo!("InBody::on_token('image')");
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
                && parser.scripting == ScriptingFlag::Enabled =>
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
        Token::StartTag(tag) => {
            warn!("[TODO] InBody: '_' - Reconstruct the active formatting elements, if any.");

            let node = Element::new_html(tag.name.clone());
            parser.insert_html_element(node);

            current_state.into_transition_result()
        }
        Token::EndTag(tag) => {
            let open_elements_len = parser.open_elements.len();
            let mut i = parser.open_elements.len() - 1;
            let mut node = parser.open_elements.get(i).unwrap();
            let mut node_is_current_node = true;

            trace!(
                "InBody::on_token(EndTag(_)) - Finding matching Node for {:?}",
                tag
            );
            loop {
                trace!("InBody::on_token(EndTag(_)) - Node: {:?}", node);
                let tag_name = &tag.name;
                if node.borrow().is_html() && node.borrow().name() == tag_name {
                    parser.generate_implied_end_tags(Some(tag_name));
                    if !node_is_current_node {
                        parse_error("");
                    }

                    // Pop all the nodes from the current node up to node, including node, then stop these steps.
                    trace!(
                        "InBody::on_token(EndTag(_)) - Popping {} element(s)",
                        open_elements_len - i
                    );
                    while i != open_elements_len {
                        let e = parser.open_elements.pop();
                        trace!("InBody::on_token(EndTag(_)) - Popped {:?}", e);
                        i += 1;
                    }

                    break;
                } else if node.borrow().category() == Category::Special {
                    parse_error("Special Node found in body");
                    return current_state.into_transition_result();
                }

                node_is_current_node = false;
                i -= 1;
                node = parser.open_elements.get(i).unwrap();
            }

            current_state.into_transition_result()
        }
    }
}

fn adoption_agency_algorithm<R>(parser: &mut Parser<R>, token: &Token)
where
    R: io::Read + io::Seek,
{
    let subject = token.tag_name().unwrap();
    let current_node = parser.current_node().unwrap();
    if current_node.borrow().is_html()
        && current_node.borrow().name() == subject
        && parser
            .list_of_active_formatting_elements
            .contains(&current_node.clone().into())
    {
        let _ = parser.open_elements.pop();
        return;
    }

    let mut outer_loop_counter = 0;
    '_outer: loop {
        if outer_loop_counter >= 8 {
            return;
        }
        outer_loop_counter += 1;
        let _formatting_element = match parser
            .list_of_active_formatting_elements
            .iter()
            .rev()
            .take_while(|e| !e.is_marker())
            .find(|e| e.is_element(subject))
        {
            None => {
                todo!("adoption_agency_algorithm return and instead act as described in the \"any other end tag\" entry above.");
            }
            Some(e) => e,
        };

        todo!("adoption_agency_algorithm for {:?} {}", token, outer_loop_counter)
    }
}
