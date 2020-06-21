#![allow(dead_code)]

use std::io;
use std::cell::RefCell;
use std::rc::Rc;

use log::{debug, trace};

use crate::{
    dom::{self, Document},
    tokenizer::{TagName, Token, Tokenizer},
};

mod errors;
mod open_elements_stack;
mod states;
mod transition_result;
mod transitions;

use open_elements_stack::OpenElementsStack;
use states::States;
use transition_result::TransitionResult;

pub struct Parser<R>
where
    R: io::Read + io::Seek,
{
    document: Document,

    tokenizer: Tokenizer<R>,

    insertion_mode: Option<States>,
    reprocess: bool,
    last_token: Option<Token>,

    // TODO open_elements 
    open_elements: OpenElementsStack,

    // Element pointsers
    head_element_pointer: Option<Rc<RefCell<dom::Element>>>,
}

impl<R> Parser<R>
where
    R: io::Read + io::Seek,
{
    pub fn new(r: R) -> Self {
        let document: Document = Default::default();
        let tokenizer = Tokenizer::new(r, false);

        Parser {
            document,
            tokenizer,

            insertion_mode: Some(States::new()),
            reprocess: false,
            last_token: None,
            open_elements: Default::default(),

            head_element_pointer: None,
        }
    }

    pub fn run(&mut self) {
        loop {
            let insertion_mode = self.insertion_mode.take().unwrap();

            let num_open_elements = self.open_elements.len();
            debug!(
                "State ({}) [{:3} elements]: {:?}",
                if self.reprocess { "R" } else { "-" },
                num_open_elements,
                insertion_mode
            );
            let res = match insertion_mode {
                States::Term(_) => return,
                _ => {
                    let token = if !self.reprocess {
                        self.tokenizer.next().unwrap()
                    } else {
                        self.last_token.take().unwrap()
                    };

                    // tree construction dispatcher
                    if self.is_tree_construction_first_case(&token) {
                        trace!("Received token {:?}", token);
                        let ret = insertion_mode.on_token(self, &token);
                        self.last_token = Some(token);
                        ret
                    } else {
                        todo!("Parser: Process the token according to the rules given in the section for parsing tokens in foreign content.");
                    }
                }
            };

            if res.is_err() {
                let next_state_error = res.state().unwrap_err();
                // TODO return err?
                panic!("Parser error: {}", next_state_error);
            }

            trace!("Document: {:?}", self.document);
            self.reprocess = res.reprocess();
            self.insertion_mode = Some(res.state().unwrap());
        }
    }

    fn set_head(&mut self, head_elem: Rc<RefCell<dom::Element>>) {
        self.head_element_pointer = Some(head_elem);
    }

    fn insert_character<C: AsRef<str>>(&mut self, data: C) {
        let (target, pos) = self.appropriate_place_for_inserting_a_node(None).unwrap();
        let mut target = target.borrow_mut();
        if pos > 0 {
            if let Some(dom::ElementChildNode::Text(ref mut text)) = target.get_mut(pos - 1) {
                return text.push_str(data.as_ref());
            }
        }
        let node = dom::Text::new(data.as_ref().to_string());
        target.insert(pos, node.into());
    }

    // Returning the parent element and the index to insert at
    //
    // ie. You cancall this then call `ret.0.insert(ret.1, new_elem)`
    fn appropriate_place_for_inserting_a_node(
        &mut self,
        r#override: Option<()>,
    ) -> Option<(Rc<RefCell<dom::Element>>, usize)> {
        if r#override.is_some() {
            todo!("Parser::appropriate_place_for_inserting_a_node with override")
        }
        let target = self.current_node()?;

        // TODO: foster parenting
        if target.borrow().name() == &TagName::Template {
            todo!("Parser::appropriate_place_for_inserting_a_node in template")
        }
        let pos = target.borrow().len();

        Some((target, pos))
    }

    // https://html.spec.whatwg.org/multipage/parsing.html#tree-construction
    fn is_tree_construction_first_case(&self, token: &Token) -> bool {
        // If the stack of open elements is empty
        if self.open_elements.is_empty() {
            return true;
        }

        let adjusted_current_node = self.adjusted_current_node().expect(
            "No adjusted_current_node when there are elements on the stack of open elements",
        );
        let node = adjusted_current_node.borrow();
        let is_token_start_tag = token.is_start_tag();
        let is_token_character = token.is_start_tag();
        let is_token_eof = token.is_eof();
        let token_tag_name = token.tag_name();

        // If the adjusted current node is an element in the HTML namespace
        if node.namespace() == dom::Namespace::HTML {
            return true;
        }

        // If the adjusted current node is a MathML text integration point and
        // the token is a start tag whose tag name is neither "mglyph" nor "malignmark"
        if node.is_mathml_text_integration_point()
            && is_token_start_tag
            && (token_tag_name != Some(&TagName::Mglyph)
                && token_tag_name != Some(&TagName::Malignmark))
        {
            return true;
        }

        // If the adjusted current node is a MathML text integration point and
        // the token is a character token
        if node.is_mathml_text_integration_point() && is_token_character {
            return true;
        }

        // If the adjusted current node is a MathML annotation-xml element and
        // the token is a start tag whose tag name is "svg"
        if node.name() == &TagName::AnnotationXml
            && is_token_start_tag
            && token_tag_name == Some(&TagName::Svg)
        {
            return true;
        }

        // If the adjusted current node is an HTML integration point and
        // the token is a start tag
        if node.is_html_integration_point() && is_token_start_tag {
            return true;
        }

        // If the adjusted current node is an HTML integration point and
        // the token is a character token
        if node.is_html_integration_point() && is_token_character {
            return true;
        }

        // If the token is an end-of-file token
        if is_token_eof {
            return true;
        }

        false
    }

    pub fn adjusted_current_node(&self) -> Option<Rc<RefCell<dom::Element>>> {
        if
        /* self.parsing_algorithm == HTMLFragmentParsing && self.open_elements.len() == 1 */
        false {
            todo!("Parser::adjusted_current_node with HTMLFragmentParsing");
        } else {
            self.current_node()
        }
    }

    pub fn current_node(&self) -> Option<Rc<RefCell<dom::Element>>> {
        self.open_elements.last().map(|e| Rc::clone(e))
    }
}
